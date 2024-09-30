pub(crate) mod frame_mgr;
pub(crate) mod runtime_track;
pub(crate) mod wed;
pub(crate) mod window_context;
pub(crate) mod windows_process;

use self::frame_mgr::FrameMgr;
use crate::{
    application::{self, Application, APP_STOPPED, IS_UI_MAIN_THREAD, IS_UI_THREAD, SHARED_CHANNEL, UI_THREAD_CNT},
    application_window::ApplicationWindow,
    backend::{opengl_backend::OpenGLBackend, raster_backend::RasterBackend, Backend, BackendType},
    graphics::board::Board,
    opti::tracker::Tracker,
    platform::logic_window::LogicWindow,
    prelude::*,
    primitive::{close_handler::CloseHandlerMgr, cpu_balance::CpuBalance, Message},
};
use std::{
    sync::atomic::Ordering,
    thread::{self, JoinHandle},
};
use tipc::IpcType;
use tlib::{
    events::{downcast_event, EventType, ResizeEvent},
    global::SemanticExt,
    payload::PayloadWeight,
    r#async::tokio_runtime,
    timer::TimerHub,
};

pub(crate) fn start_ui_runtime<T, M>(
    index: usize,
    ui_stack_size: usize,
    logic_window: LogicWindow<T, M>,
) -> JoinHandle<()>
where
    T: 'static + Copy + Sync + Send,
    M: 'static + Copy + Sync + Send,
{
    thread::Builder::new()
        .name(format!("tmui-main-{}", index))
        .stack_size(ui_stack_size)
        .spawn(move || ui_runtime::<T, M>(logic_window))
        .unwrap()
}

pub(crate) fn ui_runtime<T, M>(mut logic_window: LogicWindow<T, M>)
where
    T: 'static + Copy + Sync + Send,
    M: 'static + Copy + Sync + Send,
{
    let track = Tracker::start("window_initialize");
    let on_activate = logic_window.on_activate.take();
    let on_user_event_receive = logic_window.on_user_event_receive.take();
    let on_request_receive = logic_window.on_request_receive.take();

    let context = logic_window.context.take().unwrap();
    let (input_receiver, output_sender) = (context.input_receiver.0, context.output_sender);

    // Set up the ipc shared channel.
    if let Some(shared_channel) = logic_window.shared_channel.take() {
        SHARED_CHANNEL.with(|s| *s.borrow_mut() = Some(Box::new(shared_channel)));
    }

    // Set the UI thread to the `Main` thread.
    IS_UI_THREAD.with(|is_ui| *is_ui.borrow_mut() = true);
    if UI_THREAD_CNT.fetch_add(1, Ordering::Release) == 0 {
        IS_UI_MAIN_THREAD.with(|is_main| *is_main.borrow_mut() = true);
    }

    // Setup the tokio async runtime
    let _guard = tokio_runtime().enter();

    // Initialize the `ActionHub`.
    ActionHub::initialize();

    let bitmap = logic_window.bitmap();
    let read_guard = bitmap.read();
    let width = read_guard.width();
    let height = read_guard.height();
    drop(read_guard);

    // Create the [`Backend`] based on the backend type specified by the user.
    let backend: Box<dyn Backend> = match logic_window.backend_type {
        BackendType::Raster => RasterBackend::new(bitmap),
        BackendType::OpenGL => {
            // Make gl context current and load if the backend was `OpenGl`.
            logic_window.gl_make_current();
            logic_window.gl_load();

            OpenGLBackend::new(bitmap, logic_window.gl_config_unwrap())
        }
    };

    // Prepare ApplicationWindow env: Create the `Board`.
    let mut board = Box::new(Board::new(logic_window.bitmap(), backend));

    let mut window =
        ApplicationWindow::new(logic_window.platform_type, width as i32, height as i32);
    if let Some(parent_win) = logic_window.get_parent_window() {
        window.set_parent_window(parent_win);
    }
    if let Some(rwh) = logic_window.raw_window_handle() {
        window.set_raw_window_handle(rwh)
    }
    window.set_board(board.as_mut());
    window.register_output(output_sender);
    window.set_ipc_bridge(logic_window.create_ipc_bridge());
    window.set_outer_position(logic_window.initial_position);
    window.set_params(logic_window.params.take());

    if let Some(window_id) = logic_window.window_id() {
        window.set_winit_id(window_id)
    }

    if let Some(on_activate) = on_activate {
        on_activate(&mut window);
    }

    board.add_element(window.as_mut());
    window.initialize();
    window.run_after();

    let mut cpu_balance = CpuBalance::new();
    let mut frame_manager = FrameMgr::new();
    let mut resized = false;
    let size = window.size();
    let mut size_record = (size.width() as u32, size.height() as u32);

    Application::<T, M>::set_app_started();
    drop(track);
    board.shuffle();

    loop {
        if APP_STOPPED.load(Ordering::Relaxed) {
            break;
        }
        cpu_balance.loop_start();

        let update = frame_manager.process(
            board.as_mut(),
            window.as_mut(),
            &mut logic_window,
            &mut cpu_balance,
            &mut resized,
            &size_record,
        );

        TimerHub::with(|timer_hub| timer_hub.check_timers());
        tlib::r#async::async_callbacks();

        if let Ok(event) = input_receiver.try_recv() {
            match event {
                Message::Event(mut evt) => {
                    if evt.event_type() == EventType::Resize {
                        let resize_evt = downcast_event::<ResizeEvent>(evt).unwrap();

                        if resize_evt.width() > 0 && resize_evt.height() > 0 {
                            size_record = (resize_evt.width() as u32, resize_evt.height() as u32);
                            resized = true;
                        } else {
                            window.set_minimized(true);
                        }

                        evt = resize_evt;

                        application::request_high_load(true);
                    }

                    cpu_balance.add_payload(evt.payload_wieght());
                    let evt = window.dispatch_event(evt);
                    if let Some(ref evt) = evt {
                        if logic_window.ipc_type == IpcType::Master {
                            Application::<T, M>::send_event_ipc(evt);
                        }
                    }
                }
                Message::WindowResponse(_, closure) => {
                    closure(&mut window);
                }
                Message::WindowClosed => {
                    break;
                }
                Message::WindowMoved(position) => window.set_outer_position(position),
                _ => {}
            }
        }

        if logic_window.ipc_type == IpcType::Slave {
            let size = window.ipc_bridge().as_ref().unwrap().size();
            if size_record != size {
                size_record.0 = size.0;
                size_record.1 = size.1;
                resized = true;

                let evt = ResizeEvent::new(size.0 as i32, size.1 as i32).boxed();

                cpu_balance.add_payload(evt.payload_wieght());

                window.dispatch_event(evt);

                Board::force_update();

                application::request_high_load(true);
            }
        }

        if let Some(ref on_user_event_receive) = on_user_event_receive {
            Application::<T, M>::process_user_events(
                &mut window,
                &mut cpu_balance,
                on_user_event_receive,
            );
        }
        if let Some(ref on_rqst_receive) = on_request_receive {
            Application::<T, M>::process_request(&mut window, &mut cpu_balance, on_rqst_receive);
        } else {
            Application::<T, M>::process_request_ignored()
        }

        window.iter_execute();
        window.check_show_on_ready();

        cpu_balance.payload_check();
        if window.is_high_load_requested() {
            cpu_balance.request_high_load();
        }

        cpu_balance.sleep(update);
    }

    CloseHandlerMgr::process();
}

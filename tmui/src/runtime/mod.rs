pub(crate) mod frame_manager;
pub(crate) mod runtime_track;
pub(crate) mod wed;
pub(crate) mod window_context;
pub(crate) mod windows_process;

use crate::{
    application::{self, Application, APP_STOPPED, IS_UI_MAIN_THREAD, SHARED_CHANNEL},
    application_window::ApplicationWindow,
    backend::{opengl_backend::OpenGLBackend, raster_backend::RasterBackend, Backend, BackendType},
    graphics::board::Board,
    platform::logic_window::LogicWindow,
    prelude::*,
    primitive::{cpu_balance::CpuBalance, Message},
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
use self::frame_manager::FrameManager;

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
    IS_UI_MAIN_THREAD.with(|is_main| *is_main.borrow_mut() = true);

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
    let backend: Box<dyn Backend>;
    match logic_window.backend_type {
        BackendType::Raster => backend = RasterBackend::new(bitmap),
        BackendType::OpenGL => backend = OpenGLBackend::new(bitmap),
    }

    // Prepare ApplicationWindow env: Create the `Board`.
    let mut board = Box::new(Board::new(logic_window.bitmap(), backend));

    let mut window =
        ApplicationWindow::new(logic_window.platform_type, width as i32, height as i32);
    window.set_board(board.as_mut());
    window.register_output(output_sender);
    window.set_ipc_bridge(logic_window.create_ipc_bridge());

    if let Some(window_id) = logic_window.window_id() {
        window.set_winit_id(window_id)
    }
    if let Some(raw_window_handle) = logic_window.raw_window_handle() {
        window.set_raw_window_handle(raw_window_handle)
    }

    if let Some(on_activate) = on_activate {
        on_activate(&mut window);
        drop(on_activate);
    }

    board.add_element(window.as_mut());
    window.initialize();
    window.run_after();

    let mut cpu_balance = CpuBalance::new();
    let mut frame_manager = FrameManager::new();
    let mut resized = false;
    let size = window.size();
    let mut size_record = (size.width() as u32, size.height() as u32);

    Application::<T, M>::set_app_started();

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

        if let Ok(Message::Event(mut evt)) = input_receiver.try_recv() {
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
                    Application::<T, M>::send_event_ipc(&evt);
                }
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

        cpu_balance.payload_check();
        if window.is_high_load_requested() {
            cpu_balance.request_high_load();
        }
        cpu_balance.sleep(update);
    }
}

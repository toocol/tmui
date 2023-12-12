pub(crate) mod runtime_track;
pub(crate) mod window_context;
pub(crate) mod windows_process;
pub(crate) mod wed;

use crate::{
    application::{Application, APP_STOPPED, FRAME_INTERVAL, IS_UI_MAIN_THREAD, SHARED_CHANNEL},
    application_window::ApplicationWindow,
    backend::{opengl_backend::OpenGLBackend, raster_backend::RasterBackend, Backend, BackendType},
    graphics::board::Board,
    platform::logic_window::LogicWindow,
    prelude::*,
    primitive::{cpu_balance::CpuBalance, frame::Frame, Message},
};
use log::debug;
use std::{sync::atomic::Ordering, time::Instant};
use tlib::{
    events::{downcast_event, EventType, ResizeEvent},
    payload::PayloadWeight,
    r#async::tokio_runtime,
    timer::TimerHub,
};

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

    // Create and initialize the `ActionHub`.
    let mut action_hub = ActionHub::new();
    action_hub.initialize();

    // Create and initialize the `TimerHub`.
    let mut timer_hub = TimerHub::new();
    timer_hub.initialize();

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

    if let Some(on_activate) = on_activate {
        on_activate(&mut window);
        drop(on_activate);
    }

    board.add_element(window.as_mut());
    window.initialize();
    window.run_after();

    let mut cpu_balance = CpuBalance::new();
    let mut frame = Frame::empty_frame();
    let mut last_frame = Instant::now();
    let mut update = true;
    let mut resized = false;
    let mut size_record = (0, 0);
    let mut frame_cnt = 0;
    let (mut time_17, mut time_17_20, mut time_20_25, mut time_25) = (0, 0, 0, 0);
    let mut log_instant = Instant::now();

    Application::<T, M>::set_app_started();

    loop {
        if APP_STOPPED.load(Ordering::Relaxed) {
            break;
        }

        cpu_balance.loop_start();
        let elapsed = last_frame.elapsed();

        update = if elapsed.as_micros() >= FRAME_INTERVAL || Board::is_force_update() {
            if resized {
                logic_window.resize(size_record.0, size_record.1);
                board.resize();
                resized = false;
            }

            last_frame = Instant::now();
            let frame_time = elapsed.as_micros() as f32 / 1000.;
            frame_cnt += 1;
            match frame_time as i32 {
                0..=16 => time_17 += 1,
                17..=19 => time_17_20 += 1,
                20..=24 => time_20_25 += 1,
                _ => time_25 += 1,
            }
            if log_instant.elapsed().as_secs() >= 1 {
                debug!(
                    "frame time distribution rate: [<17ms: {}%, 17-20ms: {}%, 20-25ms: {}%, >=25ms: {}%], frame time: {}ms",
                    time_17 as f32 / frame_cnt as f32 * 100., time_17_20 as f32 / frame_cnt as f32 * 100., time_20_25 as f32 / frame_cnt as f32 * 100., time_25 as f32 / frame_cnt as f32 * 100., frame_time
                    );
                log_instant = Instant::now();
            }

            frame = frame.next();

            let update = board.invalidate_visual();
            if window.minimized() {
                window.set_minimized(false);
            }
            if update {
                let msg = Message::VSync(Instant::now());
                cpu_balance.add_payload(msg.payload_wieght());
                window.send_message(msg);
            }
            update
        } else {
            update
        };

        timer_hub.check_timers();
        action_hub.process_multi_thread_actions();
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
            }

            cpu_balance.add_payload(evt.payload_wieght());
            let evt = window.dispatch_event(evt);
            if let Some(ref evt) = evt {
                Application::<T, M>::send_event_ipc(&evt);
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

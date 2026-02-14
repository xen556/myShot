use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    window::Window,
    window::WindowAttributes,
};

#[derive(Default)]
struct App {
    window: Option<Window>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_attributes = WindowAttributes::default()
            .with_decorations(false)
            .with_blur(false)
            .with_transparent(true)
            .with_active(true)
            .with_fullscreen(Some(winit::window::Fullscreen::Borderless(None)));

        let window = event_loop
            .create_window(window_attributes)
            .unwrap();

        self.window = Some(window);
        #[cfg(target_os = "windows")]
        {
            use winit::platform::windows::WindowExtWindows;
            use winapi::um::winuser::{SetWindowLongW, GetWindowLongW, GWL_EXSTYLE, WS_EX_TOOLWINDOW};

            let hwnd = window.hwnd() as *mut _;
            unsafe {
                let ex_style = GetWindowLongW(hwnd, GWL_EXSTYLE);
                SetWindowLongW(hwnd, GWL_EXSTYLE, ex_style | WS_EX_TOOLWINDOW as i32);
            }
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        if let WindowEvent::CloseRequested = event {
            event_loop.exit();
        }
    }
}

pub fn test() {
    let event_loop = EventLoop::new().unwrap();
    let mut app = App::default();
    event_loop.run_app(&mut app).unwrap();
}
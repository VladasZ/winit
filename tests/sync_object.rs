#[allow(dead_code)]
fn needs_sync<T: Sync>() {}

#[test]
fn event_loop_proxy_send() {
    // Ensures that `winit::EventLoopProxy<T: Send>` implements `Sync`.
    needs_sync::<winit::event_loop::EventLoopProxy>();
}

#[test]
fn window_sync() {
    // Ensures that `winit::Window` implements `Sync`.
    needs_sync::<winit::window::Window>();
}

#[test]
fn window_builder_sync() {
    needs_sync::<winit::window::WindowAttributes>();
}

#[test]
fn custom_cursor_sync() {
    needs_sync::<winit::window::CustomCursorSource>();
    needs_sync::<winit::window::CustomCursor>();
}

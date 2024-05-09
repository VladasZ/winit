use std::marker::PhantomData;

use crate::application::ApplicationHandler;
use crate::error::EventLoopError;
use crate::event::Event;
use crate::event_loop::ActiveEventLoop as RootActiveEventLoop;

use super::{backend, device, window};

mod proxy;
pub(crate) mod runner;
mod state;
mod window_target;

pub(crate) use proxy::EventLoopProxy;
pub(crate) use window_target::{ActiveEventLoop, OwnedDisplayHandle};

pub struct EventLoop {
    elw: RootActiveEventLoop,
}

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub(crate) struct PlatformSpecificEventLoopAttributes {}

impl EventLoop {
    pub(crate) fn new(_: &PlatformSpecificEventLoopAttributes) -> Result<Self, EventLoopError> {
        let elw = RootActiveEventLoop { p: ActiveEventLoop::new(), _marker: PhantomData };
        Ok(EventLoop { elw })
    }

    pub fn run_app<A: ApplicationHandler>(self, app: &mut A) -> ! {
        let target = RootActiveEventLoop { p: self.elw.p.clone(), _marker: PhantomData };

        // SAFETY: Don't use `move` to make sure we leak the `event_handler` and `target`.
        let handler: Box<dyn FnMut(Event)> = Box::new(|event| handle_event(app, &target, event));

        // SAFETY: The `transmute` is necessary because `run()` requires `'static`. This is safe
        // because this function will never return and all resources not cleaned up by the point we
        // `throw` will leak, making this actually `'static`.
        let handler = unsafe { std::mem::transmute(handler) };
        self.elw.p.run(handler, false);

        // Throw an exception to break out of Rust execution and use unreachable to tell the
        // compiler this function won't return, giving it a return type of '!'
        backend::throw(
            "Using exceptions for control flow, don't mind me. This isn't actually an error!",
        );

        unreachable!();
    }

    pub fn spawn_app<A: ApplicationHandler + 'static>(self, mut app: A) {
        let target = RootActiveEventLoop { p: self.elw.p.clone(), _marker: PhantomData };

        self.elw.p.run(Box::new(move |event| handle_event(&mut app, &target, event)), true);
    }

    pub fn create_proxy(&self) -> EventLoopProxy {
        EventLoopProxy::new(self.elw.p.waker())
    }

    pub fn window_target(&self) -> &RootActiveEventLoop {
        &self.elw
    }
}

fn handle_event<T: 'static, A: ApplicationHandler<T>>(
    app: &mut A,
    target: &RootActiveEventLoop,
    event: Event,
) {
    match event {
        Event::NewEvents(cause) => app.new_events(target, cause),
        Event::Window { window_id, event } => app.window_event(target, window_id, event),
        Event::Device { device_id, event } => app.device_event(target, device_id, event),
        Event::UserWakeUp => app.user_wake_up(target),
        Event::Suspended => app.suspended(target),
        Event::Resumed => app.resumed(target),
        Event::AboutToWait => app.about_to_wait(target),
        Event::LoopExiting => app.exiting(target),
        Event::MemoryWarning => app.memory_warning(target),
    }
}

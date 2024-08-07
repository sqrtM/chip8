use std::marker::PhantomData;
use std::rc::Rc;
use std::sync::{Arc, RwLock};

use winit::application::ApplicationHandler;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::{Window, WindowAttributes, WindowId};

use crate::Controller;

pub(crate) fn init<E>(event_loop: EventLoop<E>, mut app: impl ApplicationHandler<E>) {
    #[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
    event_loop.run_app(&mut app).unwrap();

    #[cfg(any(target_arch = "wasm32", target_arch = "wasm64"))]
    winit::platform::web::EventLoopExtWebSys::spawn_app(event_loop, app);
}

pub(crate) fn make_window(
    elwt: &ActiveEventLoop,
    f: impl FnOnce(WindowAttributes) -> WindowAttributes,
) -> Rc<Window> {
    let attributes = f(WindowAttributes::default());
    #[cfg(target_arch = "wasm32")]
    let attributes = winit::platform::web::WindowAttributesExtWebSys::with_append(attributes, true);
    let window = elwt.create_window(attributes);
    Rc::new(window.unwrap())
}

pub(crate) struct WinitApp<T, Init, Handler, E> {
    init: Init,
    event: Handler,
    state: Option<T>,
    controller: Arc<RwLock<Controller>>,
    _event_marker: Option<E>,
}

pub(crate) struct WinitAppBuilder<T, Init, E> {
    init: Init,
    _marker: PhantomData<Option<T>>,
    _event_marker: PhantomData<Option<E>>,
}

impl<T, Init, E: 'static> WinitAppBuilder<T, Init, E>
where
    Init: FnMut(&ActiveEventLoop) -> T,
{
    pub(crate) fn with_init(init: Init) -> Self {
        Self {
            init,
            _marker: PhantomData,
            _event_marker: PhantomData,
        }
    }

    pub(crate) fn with_event_handler<F>(
        self,
        handler: F,
        controller: Arc<RwLock<Controller>>,
    ) -> WinitApp<T, Init, F, E>
    where
        F: FnMut(&mut T, Event<E>, &ActiveEventLoop, Arc<RwLock<Controller>>),
    {
        WinitApp::new(self.init, handler, controller)
    }
}

impl<T, Init, Handler, E> WinitApp<T, Init, Handler, E>
where
    Init: FnMut(&ActiveEventLoop) -> T,
    Handler: FnMut(&mut T, Event<E>, &ActiveEventLoop, Arc<RwLock<Controller>>),
{
    pub(crate) fn new(init: Init, event: Handler, controller: Arc<RwLock<Controller>>) -> Self {
        Self {
            init,
            event,
            controller,
            state: None,
            _event_marker: None,
        }
    }
}

impl<T, Init, Handler, E: 'static> ApplicationHandler<E> for WinitApp<T, Init, Handler, E>
where
    Init: FnMut(&ActiveEventLoop) -> T,
    Handler: FnMut(&mut T, Event<E>, &ActiveEventLoop, Arc<RwLock<Controller>>),
{
    fn resumed(&mut self, el: &ActiveEventLoop) {
        debug_assert!(self.state.is_none());
        self.state = Some((self.init)(el));
    }

    fn suspended(&mut self, _event_loop: &ActiveEventLoop) {
        let state = self.state.take();
        debug_assert!(state.is_some());
        drop(state);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        let state = self.state.as_mut().unwrap();
        (self.event)(
            state,
            Event::WindowEvent { window_id, event },
            event_loop,
            Arc::clone(&self.controller),
        );
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        if let Some(state) = self.state.as_mut() {
            (self.event)(
                state,
                Event::AboutToWait,
                event_loop,
                Arc::clone(&self.controller),
            );
        }
    }

    fn user_event(&mut self, event_loop: &ActiveEventLoop, event: E) {}
}

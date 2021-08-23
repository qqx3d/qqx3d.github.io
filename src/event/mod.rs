pub mod callback;

use callback::callbacks;
use crate::Window;
use glium::glutin::event::{Event, WindowEvent};

pub use glium::glutin::event_loop::ControlFlow;

type EventLoop = glium::glutin::event_loop::EventLoop <()>;

static mut EVENTLOOP: Option <EventLoop> = None;

#[ctor::ctor]
#[inline(always)]
fn init() {
    unsafe { EVENTLOOP = Some(EventLoop::new()); }
}

impl crate::Stt {
    pub fn eventloop() -> &'static mut EventLoop {
        unsafe { EVENTLOOP.as_mut().unwrap() }
    }
}

pub fn eventloop() -> ! {
    unsafe {
        EVENTLOOP.take().unwrap().run(|event, _, control_flow| {
            match event {
                Event::WindowEvent { event, window_id } => {
                    let window = Window::by_id(window_id);

                    match event {
                        WindowEvent::CloseRequested => {
                            if let Some(cb) = &mut callbacks().on_close { cb(window) }
                            *control_flow = ControlFlow::Exit;
                            return
                        },
                        _ => { }
                    }
                },
                _ => { }
            }

            if let Some(cb) = &mut callbacks().on_frame { *control_flow = cb() }
        })
    }
}

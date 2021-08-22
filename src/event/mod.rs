use crate::Window;
use winit::{
    platform::run_return::EventLoopExtRunReturn,
    event::{Event as WEvent, WindowEvent},
    event_loop::ControlFlow
};

#[derive(Copy, Clone)]
#[repr(u8)]
pub enum Event {
    Nothing,
    RedrawRequest(Window),
    CloseRequested(Window)
}

pub fn eventloop <F> (mut handler: F) -> ! where F: FnMut(Event) {
    loop {
        crate::init::Stt::eventloop().run_return(|ev, _, control_flow| {
            *control_flow = ControlFlow::Poll;
            match ev {
                WEvent::RedrawRequested(window) => {
                    let window = Window::by_wid(window);
                    window.update_on_frame();
                    handler(Event::RedrawRequest(window))
                },
                WEvent::WindowEvent {
                    window_id,
                    event
                } => {
                    let window = Window::by_wid(window_id);
                    window.update_on_frame();
                    match event {
                        WindowEvent::CloseRequested => {
                            handler(Event::CloseRequested(window));
                            std::process::exit(0)
                        },
                        _ => handler(Event::Nothing)
                    }
                },
                _ => handler(Event::Nothing)
            }
        });
    }
}

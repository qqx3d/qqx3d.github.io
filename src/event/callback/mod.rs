use crate::Window;

pub use glium::glutin::event_loop::ControlFlow;

macro_rules! callbacks {
    ($($big:ident ($($tts:tt)*) : $low:ident,)*) => {
        $(
            pub trait $big = $($tts)*;

            pub fn $low <F> (cb: F) where F: $big + 'static {
                callbacks().$low = Some(Box::new(cb))
            }
        )*

        pub(crate) struct Callbacks {
            $(pub $low: Option <Box <dyn $big>>,)*
        }

        static mut CALLBACKS: Callbacks = Callbacks {
            $($low: None,)*
        };

        pub(super) fn callbacks() -> &'static mut Callbacks { unsafe { &mut CALLBACKS } }
    }
}

callbacks! {
    OnFrame(FnMut() -> ControlFlow) : on_frame,
    OnClose(FnMut(Window))          : on_close,
}

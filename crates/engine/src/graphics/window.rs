// use std::ops::Deref;

// use winit::{event_loop::{EventLoop, ControlFlow, EventLoopWindowTarget}, event::Event, error::OsError};

// pub use winit::window::WindowBuilder;

// pub struct Window {
//     pub window: winit::window::Window,
//     pub event_loop: EventLoop<()>,
// }

// impl Window {
//     pub fn new(builder: WindowBuilder) -> Result<Self, OsError> {
//         let event_loop = EventLoop::new();
//         let window = builder.build(&event_loop)?;
//         Ok(Self {
//             window,
//             event_loop,
//         })
//     }

// }

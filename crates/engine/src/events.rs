use instant::{Instant, Duration};
use winit::{event::{Event, WindowEvent, ElementState}, event_loop::ControlFlow, window::Window};
use hashbrown::HashSet;

use crate::graphics::Graphics;

pub type KeyCode = winit::event::VirtualKeyCode;

pub enum WindowEvents {
    /// Update delta
    Draw(Option<Duration>),
    Exit,
}

pub struct EventProcessor {
    pub now: Instant,
    pub ups: Duration,
    pub keyboard: Keyboard,
}

#[derive(Default)]
pub struct Keyboard {
    pub pressed: HashSet<KeyCode>,
    pub down: HashSet<KeyCode>,
}

impl EventProcessor {

    pub const DEFAULT_UPS: Duration = Duration::from_micros(0_016_667);

    // pub fn new() 

    pub fn process<'a>(&mut self, event: Event<'a, ()>, control_flow: &mut ControlFlow, window: &Window, graphics: &mut Graphics) -> Result<Option<WindowEvents>, Event<'a, ()>> {
        match event {
            Event::WindowEvent { window_id, event } => match event {
                WindowEvent::Resized(size) => {
                    graphics.resize(size);
                    Ok(None)
                },
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                    Ok(None)
                },
                WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                    graphics.resize(*new_inner_size);
                    Ok(None)
                },
                WindowEvent::KeyboardInput { device_id, input, is_synthetic } => {
                    if let Some(key) = (!is_synthetic).then(|| input.virtual_keycode).flatten() {
                        match input.state {
                            ElementState::Pressed => {
                                self.keyboard.pressed.insert(key);
                            },
                            ElementState::Released => {
                                self.keyboard.pressed.remove(&key);
                                self.keyboard.down.remove(&key);
                            },
                        }
                    }
                    Ok(None)
                }
                event => Err(Event::WindowEvent { window_id, event }),
            },
            Event::RedrawRequested(id) => {
                let elapsed = self.now.elapsed();
                let delta = match elapsed > self.ups {
                    true => {
                        let mut past = elapsed;
                        let mut delta = Duration::ZERO; 
                        while past > self.ups {
                            past -= self.ups;
                            delta += self.ups;
                        }
                        self.now = Instant::now();
                        self.now += past;
                        Some(delta)
                    },
                    false => None,
                };

                Ok(Some(WindowEvents::Draw(delta)))
            },
            Event::MainEventsCleared => {
                window.request_redraw();
                Ok(None)
            }
            Event::LoopDestroyed => Ok(Some(WindowEvents::Exit)),
            e => Err(e),
        }
    }

}

impl Default for EventProcessor {
    fn default() -> Self {
        Self { now: Instant::now(), ups: Self::DEFAULT_UPS, keyboard: Default::default() }
    }
}
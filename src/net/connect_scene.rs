use firecore_game::input::{pressed, Control};
use firecore_game::macroquad::camera::Camera2D;
use firecore_game::macroquad::camera::set_camera;
use firecore_game::macroquad::prelude::KeyCode;
use firecore_game::macroquad::prelude::Rect;
use firecore_game::macroquad::prelude::error;
use firecore_game::macroquad::prelude::is_key_pressed;
use firecore_game::macroquad::prelude::vec2;
use firecore_game::macroquad::prelude::screen_height;
use firecore_game::macroquad::prelude::screen_width;
use firecore_game::macroquad::prelude::{info, warn};
use firecore_game::network::message::{ClientMessage, ServerMessage};
use firecore_game::scene::SceneState;
use firecore_game::client::{ClientSocket, Packet, LinkConditionerConfig};

use firecore_game::scene::Scenes;

use game::macroquad::ui::{root_ui, widgets, hash};

use crate::scene::Scene;

pub struct ConnectScene {

    state: SceneState,

    connecting: bool,
    accumulator: f32,
    address: String,

}

impl Scene for ConnectScene {
    fn new() -> Self where Self: Sized {
        Self {
            state: SceneState::Continue,
            connecting: false,
            accumulator: 0.0,
            address: String::new(),
        }
    }

    fn on_start(&mut self) {
        set_camera(Camera2D::from_display_rect(Rect::new(0.0, 0.0, screen_width(), screen_height())));
        self.connecting = false;
    }

    fn input(&mut self, _: f32) {
        if self.connecting {
            if pressed(Control::B) || is_key_pressed(KeyCode::Escape) {
                self.connecting = false;
            }
        }
    }

    fn update(&mut self, _: f32) {
        if !self.connecting {
            widgets::Window::new(hash!(), vec2(0.0, 0.0), vec2(screen_width(), screen_height())).ui(&mut root_ui(), |ui| {
                ui.input_text(hash!(), "Server address", &mut self.address);
                if ui.button(vec2(5.0, 30.0), "Connect") {
                    match self.address.parse::<std::net::SocketAddr>() {
                        Ok(address) => {
                            // match ClientSocket::connect(address) {
                                // Ok(socket) => {
                                    let mut socket = ClientSocket::connect(address).with_link_conditioner(&LinkConditionerConfig::average_condition());
                                    let sender = socket.get_sender();
                                    let server = unsafe {
                                        super::SERVER = Some(super::Server {
                                            socket,
                                            sender,
                                            id: None,
                                        });
                                        super::SERVER.as_mut().unwrap()
                                    };
                                    if let Err(err) = server.sender.send(Packet::new(postcard::to_allocvec(&ClientMessage::Connect).unwrap())) {
                                        game::macroquad::prelude::error!("Could not send connect message with error {}", err);
                                    } else {
                                        self.connecting = true;
                                    }
                                // }
                                // Err(err) => {
                                    // warn!("Could not connect to server with error {:?}", err);
                                // }
                            // }
                            
                        },
                        Err(err) => {

                        }
                    }
                    
                    self.connecting = true;
                }
            });
        } else {
            if let Some(server) = unsafe{super::SERVER.as_mut()} {
                match server.socket.receive() {
                    Ok(packet) => if let Some(packet) = packet {
                        match postcard::from_bytes(packet.payload()) {
                            Ok(message) => match message {
                                ServerMessage::Connect(player_id) => {
                                    info!("Connected!");
                                    server.id = Some(player_id);
                                    self.state = SceneState::Scene(Scenes::Game);
                                }
                                _ => (),
                            }
                            Err(err) => {
                                warn!("Could not deserialize packet from server! Disconnecting.");
                                self.connecting = false;
                            }
                        }
                    }
                    Err(err) => {
                        error!("Could not receive packets from client socket with error {}", err);
                        self.connecting = false;
                    }
                }
                
            }
        }
    }

    fn render(&self) {
        
    }

    fn quit(&mut self) {
        set_camera(crate::util::game_camera())
    }

    fn state(&self) -> firecore_game::scene::SceneState {
        self.state
    }
}
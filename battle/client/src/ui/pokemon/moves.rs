use game::{
    deps::tetra::{Context, graphics::DrawParams, math::Vec2},
    pokedex::battle2::script::{BattleAction, BattleActionInstance, BattleActionScriptInstance},
};

use crate::ui::BattleGuiPosition;

pub struct MoveRenderer {
    flip: bool,
    script: Option<BattleActionScriptInstance>,
}

impl MoveRenderer {

    pub fn new(position: BattleGuiPosition) -> Self {
        Self {
            flip: matches!(position, BattleGuiPosition::Top),
            script: None,
        }
    }

    pub fn init(&mut self, script: BattleActionScriptInstance) {
        self.script = Some(script);
    }

    pub fn update(&mut self, delta: f32) {
        if let Some(script) = &mut self.script {
            match &mut script.script.current {
                None => match script.script.actions.pop_front() {
                    Some(next) => {
                        if let Some(instance) = match next {
                            BattleAction::MoveAndReturnPokemon(len) => Some(BattleActionInstance::MoveAndReturn(0.0, len, false)),
                            BattleAction::MoveTexture(x, y, speed) => Some(BattleActionInstance::MoveTexture(Vec2::new(x, y), speed)),
                            BattleAction::SpawnTexture(x, y) => {
                                script.script.texture = Some(Vec2::new(x, y));
                                None
                            },
                            BattleAction::Wait(wait) => Some(BattleActionInstance::Wait(wait)),
                            BattleAction::DespawnTexture => {
                                script.script.texture = None;
                                None
                            },
                        } {
                            script.script.current = Some(instance);
                        }
                    },
                    None => self.script = None,
                },
                Some(current) => match current {
                    BattleActionInstance::MoveAndReturn(current, max, returning) => {
                        match *returning {
                            true => {
                                *current -= delta * 120.0;
                                if *current < 0.0 {
                                    script.script.current = None;
                                }
                            }
                            false => {
                                *current += delta * 120.0;
                                if current > max {
                                    *returning = true;
                                }
                            },
                        }
                    },
                    BattleActionInstance::MoveTexture(offset, speed) => {
                        let delta = delta * 60.0;
                        match script.script.texture.as_mut() {
                            Some(pos) => {
                                if offset.x > 0.0 {
                                    offset.x -= delta;
                                    pos.x += delta;
                                    if offset.x < 0.0 {
                                        pos.x += 0.0 - offset.x * *speed;
                                    }
                                }
                                if offset.y > 0.0 {
                                    offset.y -= delta;
                                    pos.y += delta;
                                    if offset.y < 0.0 {
                                        pos.y += 0.0 - offset.y * *speed;
                                    }
                                }
                                if offset.x < 0.0 && offset.y < 0.0 {
                                    script.script.current = None;
                                }
                            },
                            None => script.script.current = None,
                        }
                    }
                    BattleActionInstance::Wait(remaining) => {
                        *remaining -= delta;
                        if *remaining < 0.0 {
                            script.script.current = None;
                        }
                    }
                    ,
                    _ => unreachable!(),
                },
            }
        }
    }

    pub fn draw(&self, ctx: &mut Context) {
        if let Some(script) = &self.script {
            if let Some(texture) = &script.texture {
                if let Some(position) = script.script.texture {
                    let params = if self.flip {
                        DrawParams::default().position(position).scale(Vec2::new(-1.0, 1.0))
                    } else {
                        DrawParams::default().position(position)
                    };
                    texture.draw(ctx, params);
                }
            }
        }
    }

    #[deprecated]
    pub fn pokemon_x(&self) -> f32 {
        match &self.script {
            Some(script) => match &script.script.current {
                Some(current) => match current {
                    BattleActionInstance::MoveAndReturn(c, ..) => *c,
                    _ => Default::default(),
                },
                None => Default::default(),
            },
            None => Default::default(),
        }
    }

    pub fn finished(&self) -> bool {
        self.script.is_none()
    }

}
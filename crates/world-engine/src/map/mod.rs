use crate::engine::{graphics::Color, log::warn, math::ivec2};
use crate::pokengine::gui::SizedStr;

use pokengine::engine::notan::draw::{Draw, DrawShapes, DrawTextSection};
use worldlib::{map::WorldMap, state::WorldState, TILE_SIZE};

use self::data::ClientWorldData;

pub mod input;
pub mod manager;

pub mod data;
pub mod warp;

mod screen;
pub use screen::RenderCoords;

pub fn draw(
    draw: &mut Draw,
    map: &WorldMap,
    world: &WorldState,
    data: &ClientWorldData,
    screen: &RenderCoords,
    border: bool,
    color: Color,
) {
    for yy in screen.y.clone() {
        let y = yy - screen.offset.y;
        let render_y = (yy << 4) as f32 - screen.focus.y; // old = y_tile w/ offset - player x pixel
        let row = y.saturating_mul(map.width);

        for xx in screen.x.clone() {
            let x = xx - screen.offset.x;
            let render_x = (xx << 4) as f32 - screen.focus.x;

            if !(x < 0 || y < 0 || y >= map.height as _ || x >= map.width as _) {
                let index = x as usize + row as usize;

                if world.debug_draw {
                    let num = map.movements[index];
                    // let str = firecore_battle_gui::pokedex::gui::IntegerStr4::new(num as _).unwrap();
                    let str = match SizedStr::<3>::new(format_args!("{:X}", num)) {
                        Ok(str) => str,
                        Err(err) => {
                            warn!(
                                "Could not create movement id at ({}, {}) with error {}",
                                x, y, err
                            );
                            continue;
                        }
                    };
                    let color = match num {
                        1 => Color::BLACK,
                        0xC => Color::WHITE,
                        0x4 => Color::AQUA,
                        _ => Color::RED,
                    };
                    let inverse = Color {
                        r: 1.0 - color.r,
                        g: 1.0 - color.g,
                        b: 1.0 - color.b,
                        a: color.a,
                    };

                    draw.rect((render_x, render_y), (TILE_SIZE, TILE_SIZE))
                        .color(color);
                    draw.rect((render_x, render_y), (TILE_SIZE, TILE_SIZE))
                        .stroke(1.0)
                        .color(inverse);
                    draw.text(&data.debug_font, &str)
                        .position(render_x + 3.0, render_y + 1.0)
                        .color(inverse)
                        .h_align_left()
                        .v_align_top();
                } else {
                    let tile = map.tiles[index];

                    data.tiles
                        .draw_tile(draw, &map.palettes, tile, render_x, render_y, color);
                    // if let Some(door) = door {
                    //     if door.position == index {
                    //         textures.tiles.draw_door(ctx, door, render_x, render_y);
                    //     }
                    // }
                }
            } else if border {
                let tile = map.border[if x % 2 == 0 {
                    //  x % 2 + if y % 2 == 0 { 0 } else { 2 }
                    if y % 2 == 0 {
                        0
                    } else {
                        2
                    }
                } else if y % 2 == 0 {
                    1
                } else {
                    3
                }];
                data.tiles
                    .draw_tile(draw, &map.palettes, tile, render_x, render_y, color);
            }

            if world.debug_draw {
                for warp in map.warps.iter() {
                    for coordinate in warp.area.iter() {
                        let coordinate = ivec2(coordinate.x, coordinate.y) + screen.offset;
                        let render = (coordinate * (16)).as_vec2() - screen.focus;
                        draw.rect((render.x, render.y), (TILE_SIZE, TILE_SIZE))
                            .stroke(2.0)
                            .color(Color::RED);
                    }
                }
            }
        }
    }
    for npc in map.npcs.values().filter(|npc| !npc.character.hidden).chain(
        world
            .scripts
            .npcs
            .values()
            .filter(|(loc, ..)| loc == &map.id)
            .map(|(.., n)| n),
    ) {
        data.npc.draw(draw, npc, screen, color);
    }

    data.object.draw(
        draw,
        &map.id,
        &map.objects,
        &map.items,
        world,
        screen,
        color,
    );
    // for script in map.scripts.iter() {
    //     if script.alive() {
    //         if let Some(action) = script.actions.front() {
    //             match action {
    //                 WorldActionKind::Conditional { .. } => {
    //                     if script.option > 1 {
    //                         textures
    //                             .gui
    //                             .get(&texture::gui::GuiTexture::Condition)
    //                             .draw(ctx, position(162.0, 66.0));
    //                         draw_cursor(ctx, 170.0, 77.0 + (script.option - 2) as f32 * 16.0);
    //                     }
    //                 }
    //                 _ => (),
    //             }
    //         }
    //     }
    // }
}

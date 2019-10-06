use std::collections::HashMap;

use rand::*;
use noise::*;

use quicksilver::prelude::*;


// TODO 
//      palette colors use
//      map generation- groups of tiles, side step tile for Ls
//      add status effects- chance for player or monster to take intended action
//      another monster with different movement
//      interpolate characters between tiles
//      animations between frames
//      idle animations


const TEXT_COLOR: Color = Color::WHITE;
const BACKGROUND_COLOR: Color = Color::BLACK;

const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;

const MAP_WIDTH: usize = 10;
const MAP_HEIGHT: usize = 10;

const MAP_DRAW_X_OFFSET: usize  = 50;
const MAP_DRAW_Y_OFFSET: usize  = 120;
const TILE_WIDTH_PX: u32 = 30; // 24;
const TILE_HEIGHT_PX: u32 = 30; // 24;


#[derive(Clone, Debug, PartialEq)]
enum GameState {
    Playing,
    Lost,
}

#[derive(Clone, Debug, PartialEq)]
struct Tile {
    pos: Vector,
    glyph: char,
    color: Color,
    blocks: bool,
}


#[derive(Clone, Copy, Debug, PartialEq)]
enum Status {
    Berserk,
}

type Hp = i32;

type EntityId = usize;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Trap {
    Berserk, Kill, Bump,
    Teleport,
    CountDown(u8),
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Monster {
    hp: Hp,
    max_hp: Hp,
    status: Option<Status>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Player {
    hp: Hp,
    max_hp: Hp,
    status: Option<Status>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum EntityType {
    Trap(Trap),
    Monster(Monster),
    Player(Player),
}

impl EntityType {
    fn monster(max_hp: Hp) -> EntityType {
        return EntityType::Monster(Monster {
            hp: max_hp,
            max_hp: max_hp,
            status: None,
        });
    }

    fn trap(trap: Trap) -> EntityType {
        return EntityType::Trap(trap);
    }

    fn is_monster(&self) -> bool {
        return match self {
            EntityType::Monster(_) => true,
            _ => false,
        };
    }

    fn is_player(&self) -> bool {
        return match self {
            EntityType::Player(_) => true,
            _ => false,
        };
    }

    fn is_trap(&self) -> bool {
        return match self {
            EntityType::Trap(_) => true,
            _ => false,
        };
    }

    fn lose_hp(&mut self, amount: Hp) {
        match self {
            EntityType::Player(player) => {
                player.hp -= amount;
            },

            EntityType::Monster(monster) => {
                monster.hp -= amount;
            },

            _ => panic!("This entity cannot lose HP!"),
        }
    }
}

trait HasHp {
    fn max_hp(&self) -> Hp;
    fn hp(&self) -> Hp;
}

impl HasHp for Player {
    fn max_hp(&self) -> Hp {
        return self.max_hp;
    }

    fn hp(&self) -> Hp {
        return self.max_hp;
    }
}

impl HasHp for Monster {
    fn max_hp(&self) -> Hp {
        return self.max_hp;
    }

    fn hp(&self) -> Hp {
        return self.max_hp;
    }
}

impl HasHp for Entity {
    fn max_hp(&self) -> Hp {
        return match &self.typ {
            EntityType::Monster(monster) => monster.max_hp,
            EntityType::Player(player) => player.max_hp,
            _ => panic!("Tried to get hp from entity with no HP!"),
        };
    }

    fn hp(&self) -> Hp {
        return match &self.typ {
            EntityType::Monster(monster) => monster.hp,
            EntityType::Player(player) => player.hp,
            _ => panic!("Tried to get hp from entity with no HP!"),
        };
    }
}

type Map = Vec<Tile>;


fn generate_map(size: Vector) -> Vec<Tile> {
    let width = size.x as usize;
    let height = size.y as usize;
    let mut map = Vec::with_capacity(width * height);
    for x in 0..width {
        for y in 0..height {
            let mut tile = Tile {
                pos: Vector::new(x as f32, y as f32),
                glyph: 219 as char,
                color: TEXT_COLOR,
                blocks: false,
            };

            if x == 0 || x == width - 1 || y == 0 || y == height - 1 {
                tile.glyph = '#';
                tile.blocks = true;
            };
            map.push(tile);
        }
    }

    return map;
}

fn blocked_tile(pos: Vector, map: &Map) -> bool {
    return map.iter().any(|tile| tile.blocks && tile.pos == pos);
}

fn occupied_tile(pos: Vector, entities: &Vec<Entity>) -> Option<Entity> {
    return entities.iter().find(|entity| entity.pos == pos).map(|entity| entity.clone());
}

fn trap_tile(pos: Vector, entities: &Vec<Entity>) -> Option<Entity> {
    return entities.iter().find(|entity| entity.typ.is_trap() && entity.pos == pos).map(|entity| entity.clone());
}

fn clamp(min: f32, max: f32, value: f32) -> f32 {
    let result: f32;

    if value < min {
        result = min;
    } else if value > max {
        result = max;
    } else {
        result = value;
    }

    return result;
}

#[derive(Clone, Debug, PartialEq)]
struct Entity {
    pos: Vector,
    glyph: char,
    color: Color,
    typ: EntityType,
}

impl Entity {
    fn trap(pos: Vector, trap: Trap) -> Entity {
        let chr = match trap {
            Trap::Kill => '%',
            Trap::Berserk => '*',
            Trap::Bump => '+',
            Trap::Teleport => '!',
            Trap::CountDown(n) => ('0' as u8 + n) as char,
        };

        Entity {
            pos: pos,
            glyph: chr,
            color: Color::GREEN,
            typ: EntityType::trap(trap),
        }
    }

    fn goblin(pos: Vector) -> Entity {
        Entity {
            pos: pos,
            glyph: 'g',
            color: Color::RED,
            typ: EntityType::monster(1),
        }
    }
}

fn generate_entities(entities: &mut Vec<Entity>) {
    entities.push(Entity::goblin(Vector::new(9, 10)));
    entities.push(Entity::goblin(Vector::new(2, 14)));
    entities.push(Entity::trap(Vector::new(6, 6), Trap::Bump)); 
    entities.push(Entity::trap(Vector::new(8, 8), Trap::Berserk)); 
    entities.push(Entity::trap(Vector::new(3, 8), Trap::Berserk)); 
    entities.push(Entity::trap(Vector::new(9, 8), Trap::Berserk)); 
    entities.push(Entity::trap(Vector::new(7, 6), Trap::Kill)); 
    entities.push(Entity::trap(Vector::new(7, 8), Trap::Kill)); 
    entities.push(Entity::trap(Vector::new(7, 2), Trap::Teleport)); 
    entities.push(Entity::trap(Vector::new(4, 8), Trap::Teleport)); 
    entities.push(Entity::trap(Vector::new(1, 2), Trap::CountDown(3))); 
    entities.push(Entity::trap(Vector::new(4, 2), Trap::CountDown(1)));
}

struct Game {
    game_state: GameState,
    title: Asset<Image>,
    mononoki_font_info: Asset<Image>,
    square_font_info: Asset<Image>,
    lost_game_message: Asset<Image>,
    char_map: Asset<HashMap<u32, Image>>,
    inventory: Asset<Image>,
    map_size: Vector,
    map: Map,
    entities: Vec<Entity>,
    player_id: usize,
    tileset: Asset<HashMap<char, Image>>,
    noise: Perlin,
}

impl State for Game {
    /// Load the assets and initialise the game
    fn new() -> Result<Self> {
        // The Mononoki font: https://madmalik.github.io/mononoki/
        // License: SIL Open Font License 1.1
        let font_mononoki = "mononoki-Regular.ttf";

        let title = Asset::new(Font::load(font_mononoki).and_then(|font| {
            font.render("Ludum Dare 45", &FontStyle::new(72.0, TEXT_COLOR))
        }));

        let font_image = "rexpaint16x16.png";
        let char_map = Asset::new(Image::load(font_image).and_then(|image| {
            let mut char_map = HashMap::new();
            let char_size = Vector::new(16, 16);
            for char_ix in 0..256 {
                let char_x = char_ix % 16;
                let char_y = char_ix / 16;
                let char_pos = Vector::new(char_x * 16, char_y * 16);
                char_map.insert(char_ix,
                                image.subimage(Rectangle::new(char_pos, char_size)));
            }

            return Ok(char_map);
        }));

        let lost_game_message = Asset::new(Font::load(font_mononoki).and_then(|font| {
            font.render("You Lose!", &FontStyle::new(72.0, TEXT_COLOR))
        }));

        let mononoki_font_info = Asset::new(Font::load(font_mononoki).and_then(|font| {
            font.render(
                "",
                &FontStyle::new(20.0, TEXT_COLOR),
            )
        }));

        let square_font_info = Asset::new(Font::load(font_mononoki).and_then(move |font| {
            font.render(
                "A Ludum Dare Game by Joel and Noah Ryan",
                &FontStyle::new(20.0, TEXT_COLOR),
            )
        }));

        let inventory = Asset::new(Font::load(font_mononoki).and_then(move |font| {
            font.render(
                "Inventory:\n[A] Sword\n[B] Shield\n[C] Darts",
                &FontStyle::new(20.0, TEXT_COLOR),
            )
        }));

        let map_size = Vector::new(MAP_WIDTH as u8, MAP_HEIGHT as u8);
        let map = generate_map(map_size);
        let player_id = 0;

        let mut entities = Vec::new();
        entities.push(Entity {
            pos: Vector::new(5, 3),
            glyph: '@',
            color: Color::ORANGE,
            typ: EntityType::Player(Player { 
                hp: 5,
                max_hp: 5,
                status: None,
            }),
        });
        generate_entities(&mut entities);

        // The Square font: http://strlen.com/square/?s[]=font
        // License: CC BY 3.0 https://creativecommons.org/licenses/by/3.0/deed.en_US
        let font_square = "square.ttf";
        let game_glyphs = "#@g.%";
        let tile_size_px = Vector::new(TILE_WIDTH_PX, TILE_HEIGHT_PX);
        let tileset = Asset::new(Font::load(font_square).and_then(move |text| {
            let tiles = text
                .render(game_glyphs, &FontStyle::new(tile_size_px.y, TEXT_COLOR))
                .expect("Could not render the font tileset.");
            let mut tileset = HashMap::new();
            for (index, glyph) in game_glyphs.chars().enumerate() {
                let pos = (index as i32 * tile_size_px.x as i32, 0);
                let tile = tiles.subimage(Rectangle::new(pos, tile_size_px));
                tileset.insert(glyph, tile);
            }
            Ok(tileset)
        }));

        Ok(Self {
            game_state: GameState::Playing,
            title,
            mononoki_font_info,
            square_font_info,
            lost_game_message,
            char_map,
            inventory,
            map_size,
            map,
            entities,
            player_id,
            tileset,
            noise: Perlin::new(),
        })
    }

    /// Process keyboard and mouse, update the game state
    fn update(&mut self, window: &mut Window) -> Result<()> {

        match self.game_state {
            GameState::Playing => {
                let took_turn = update_player(self, window);

                if took_turn {
                    update_monsters(self, window);
                    resolve_traps(&mut self.entities, &self.map);
                }

                if window.keyboard()[Key::Escape].is_down() {
                    window.close();
                }

                if self.entities[self.player_id].hp() <= 0 {
                    self.game_state = GameState::Lost;
                }

                self.entities = self.entities.iter().filter(|entity| {
                    if entity.typ.is_monster() {
                        return entity.hp() > 0;
                    }

                    return true;
                }).map(|ent| ent.clone()).collect();
            },

            GameState::Lost => {
            },
        }

        Ok(())
    }

    /// Draw stuff on the screen
    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(BACKGROUND_COLOR)?;

        // Draw the game title
        self.title.execute(|image| {
            window.draw(
                &image
                    .area()
                    .with_center((window.screen_size().x as i32 / 2, 40)),
                Img(&image),
            );
            Ok(())
        })?;

        // Draw the mononoki font credits
        self.mononoki_font_info.execute(|image| {
            window.draw(
                &image
                    .area()
                    .translate((2, window.screen_size().y as i32 - 60)),
                Img(&image),
            );
            Ok(())
        })?;

        // Draw the Square font credits
        self.square_font_info.execute(|image| {
            window.draw(
                &image
                    .area()
                    .translate((2, window.screen_size().y as i32 - 30)),
                Img(&image),
            );
            Ok(())
        })?;

        let tile_size_px = Vector::new(TILE_WIDTH_PX, TILE_HEIGHT_PX);
        let offset_px = Vector::new(MAP_DRAW_X_OFFSET as u8, MAP_DRAW_Y_OFFSET as u8);

        // draw map
        for tile in self.map.iter() {
            let pos_px = tile.pos.times(tile_size_px);
            let pos = offset_px + pos_px;
            let color_noise =
                self.noise.get([6.0 * (pos.x as f64 / WINDOW_WIDTH as f64),
                                6.0 * (pos.y as f64 / WINDOW_HEIGHT as f64)]);
            dbg!(color_noise);
            dbg!(pos);
            // TODO use palette for mixing
            let dark_tile_color = Color::from_rgba(120, 128, 144, 1.0);
            let light_tile_color = Color::from_rgba(81, 97, 102, 1.0);
            let tile_color = lerp_color(dark_tile_color, light_tile_color, color_noise as f32);
            self.char_map.execute(|char_map| {
                draw_char(&char_map, window, pos, tile.glyph, tile_color);
                return Ok(());
            });
        }

        // draw entities
        // draw traps
        for entity in self.entities.iter() {
            if entity.typ.is_trap() {
                draw_entity(entity, offset_px, window, &mut self.char_map);
            }
        }

        // draw other entities
        for entity in self.entities.iter() {
            if !entity.typ.is_trap() {
                draw_entity(entity, offset_px, window, &mut self.char_map);
            }
        }

        let player = &self.entities[self.player_id];
        let full_health_width_px = 100.0;
        let current_health_width_px =
            (player.hp() as f32 / player.max_hp() as f32) * full_health_width_px;

        let map_size_px = self.map_size.times(tile_size_px);
        let health_bar_pos_px = offset_px + Vector::new(map_size_px.x, 0.0);

        // Full health
        window.draw(
            &Rectangle::new(health_bar_pos_px, (full_health_width_px, tile_size_px.y)),
            Col(Color::RED.with_alpha(0.5)),
        );

        // Current health
        window.draw(
            &Rectangle::new(health_bar_pos_px, (current_health_width_px, tile_size_px.y)),
            Col(Color::RED),
        );

        // Current health
        self.inventory.execute(|image| {
            window.draw(
                &image
                    .area()
                    .translate(health_bar_pos_px + Vector::new(0, tile_size_px.y)),
                Img(&image),
            );
            Ok(())
        })?;

        // Draw Message
        if self.game_state == GameState::Lost {
            self.lost_game_message.execute(|image| {
                window.draw(
                    &image
                        .area()
                        .translate((MAP_DRAW_X_OFFSET as u16 + 100, MAP_DRAW_X_OFFSET as u16 + 120)),
                    Img(&image),
                );
                Ok(())
            })?;
        }

        Ok(())
    }
}

// Update Functions
fn update_monsters(game: &mut Game, _window: &mut Window) {
    let player = game.entities[game.player_id].clone();
    // NOTE copies all entities every frame!
    let entities = game.entities.clone();

    let mut attacks: Vec<(EntityId, EntityId)> = Vec::new();

    for (index, monster) in game.entities.iter_mut().filter(|entity| entity.typ.is_monster()).enumerate() {
        let prev_position = monster.pos;
        let pos_diff = player.pos - monster.pos;

        monster.pos += Vector::new(pos_diff.x.signum(), pos_diff.y.signum());
        
        if blocked_tile(monster.pos, &game.map) {
            monster.pos = prev_position;
        } else if let Some(entity) = occupied_tile(monster.pos, &entities) {
            if entity.typ.is_player() {
                monster.pos = prev_position;
                attacks.push((index, entities.iter().enumerate().find(|(_index, ent)| **ent == entity).unwrap().0));
            }  else if entity.typ.is_monster() {
                monster.pos = prev_position;
            }
        }
    }

    for attack in attacks {
        let typ = &mut game.entities[attack.1].typ;
        match typ {
            EntityType::Player(_player) => {
                typ.lose_hp(1);
            },

            EntityType::Monster(_monster) => {
                typ.lose_hp(1);
            },

            _ => { },
        }
    }

    //let mut remove_indices: Vec<usize> = Vec::new();
}

fn lerp_color(src: Color, dst: Color, amount: f32) -> Color {
    return Color {
        r: lerp(src.r, dst.r, amount),
        g: lerp(src.g, dst.g, amount),
        b: lerp(src.b, dst.b, amount),
        a: lerp(src.a, dst.a, amount),
    };
}

fn attempt_move(pos: Vector, offset: Vector, map: &Map) -> Vector {
    let mut new_pos = pos + offset;

    if blocked_tile(new_pos, map) {
        new_pos = pos;
    }

    return new_pos;
}

fn update_player(game: &mut Game, window: &mut Window) -> bool {
    use ButtonState::*;

    let mut took_turn: bool = false;

    let player = &mut game.entities[game.player_id];
    let previous_pos = player.pos;
    if window.keyboard()[Key::Left] == Pressed {
        player.pos.x = clamp(0.0, MAP_WIDTH as f32, player.pos.x - 1.0);
        took_turn = true;
    }
    if window.keyboard()[Key::Right] == Pressed {
        player.pos.x = clamp(0.0, MAP_WIDTH as f32, player.pos.x + 1.0);
        took_turn = true;
    }
    if window.keyboard()[Key::Up] == Pressed {
        player.pos.y = clamp(0.0, MAP_HEIGHT as f32, player.pos.y - 1.0);
        took_turn = true;
    }
    if window.keyboard()[Key::Down] == Pressed {
        player.pos.y = clamp(0.0, MAP_HEIGHT as f32, player.pos.y + 1.0);
        took_turn = true;
    }

    if blocked_tile(player.pos, &game.map) {
        player.pos = previous_pos;
        took_turn = false;
    }

    return took_turn;
}

fn resolve_traps(entities: &mut Vec<Entity>, map: &Map) {
    let mut rng = thread_rng();
    let entities_clone = entities.clone();
    let mut removals: Vec<usize> = Vec::new();
    let mut count_downs: Vec<(usize, u8)> = Vec::new();

    let trap_iter =
        entities.iter_mut()
                .enumerate()
                .filter(|(_index, ent)| ent.typ.is_player() || ent.typ.is_monster());
    for (index, entity) in trap_iter {
        if let Some(trap_entity) = trap_tile(entity.pos, &entities_clone) {
            let trap_index = entities_clone.iter().position(|other| *other == trap_entity).unwrap();
            match trap_entity.typ {
                EntityType::Trap(trap) => {
                    match trap {
                        Trap::Berserk => {
                            match entity.typ {
                                EntityType::Monster(mut monster) => {
                                    monster.status = Some(Status::Berserk);
                                },

                                EntityType::Player(mut player) => {
                                    player.status = Some(Status::Berserk);
                                },

                                _ => panic!("Unexpected entity type!"),
                            }
                        },

                        Trap::Kill => {
                            entity.typ.lose_hp(5);
                            removals.push(trap_index);
                        },

                        Trap::Teleport => {
                            // find next teleport. if one is find, move character there.
                            let entities_len = entities_clone.len();
                            for other_index in 0..entities_len {
                                let offset_index = (other_index + trap_index + 1) % entities_len;
                                let other_entity = &entities_clone[offset_index];
                                match other_entity.typ {
                                    EntityType::Trap(Trap::Teleport) => {
                                        entity.pos = other_entity.pos;
                                        break;
                                    },

                                    _ => { },
                                }
                            }
                        },

                        Trap::Bump => {
                            let pos = entity.pos;
                            let x_offset = rng.gen_range(-1, 1);
                            let y_offset = rng.gen_range(-1, 1);
                            entity.pos =
                                attempt_move(pos,
                                             Vector::new(x_offset, y_offset),
                                             &map);
                        }

                        Trap::CountDown(mut n) => {
                            if n == 0 {
                                entity.typ.lose_hp(5);
                            } else {
                                count_downs.push((trap_index, n - 1));
                            }
                        },
                    }
                },

                _ => panic!("Unreachable?"),
            }
        }
    }

    for (ix, new_n) in count_downs.iter() {
        entities[*ix].typ = EntityType::Trap(Trap::CountDown(*new_n));
        entities[*ix].glyph = ('0' as u8 + *new_n) as char;
    }

    removals.sort();
    removals.reverse();
    for index in removals.iter() {
        entities.swap_remove(*index);
    }
}

fn draw_entity(entity: &Entity, offset_px: Vector, window: &mut Window, char_map: &mut Asset<HashMap<u32, Image>>) {
    let tile_size_px = Vector::new(TILE_WIDTH_PX, TILE_HEIGHT_PX);
    let pos_px = entity.pos.times(tile_size_px);
    let pos = offset_px + pos_px;
    char_map.execute(|char_map| {
        draw_char(&char_map, window, pos, entity.glyph, entity.color);
        return Ok(());
    }).unwrap();
}

// Draw Function
fn draw_char(char_map: &HashMap<u32, Image>, window: &mut Window, pos: Vector, chr: char, color: Color) {
    let char_ix = chr as u32;
    let rect = Rectangle::new(pos, Vector::new(16, 16));
    window.draw_ex(&rect,
                   Blended(&char_map[&char_ix], color),
                   Transform::scale(Vector::new(2.0, 2.0)),
                   2.0);
}

fn main() {
    // NOTE: Set HIDPI to 1.0 to get pixel-perfect rendering.
    // Otherwise the window resizes to whatever value the OS sets and
    // scales the contents.
    // https://docs.rs/glutin/0.19.0/glutin/dpi/index.html
    std::env::set_var("WINIT_HIDPI_FACTOR", "1.0");

    let settings = Settings {
        // If the graphics do need to be scaled (e.g. using
        // `with_center`), blur them. This looks better with fonts.
        scale: quicksilver::graphics::ImageScaleStrategy::Blur,
        ..Default::default()
    };
    run::<Game>("Ludem Dare 45", Vector::new(WINDOW_WIDTH, WINDOW_HEIGHT), settings);
}

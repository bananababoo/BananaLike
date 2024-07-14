use rltk::{GameState, RandomNumberGenerator, RGB, Rltk, VirtualKeyCode};
use specs::prelude::*;
use specs_derive::Component;

#[derive(Component)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Component)]
struct Renderable {
    glyph: rltk::FontCharType,
    fg: RGB,
    bg: RGB,
}

struct State {
    ecs: World
}
impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();
        
        player_input(self, ctx);
        self.run_systems();

        let map =  self.ecs.fetch::<Vec<TileType>>();
        draw_map(&map, ctx);

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        for (pos,render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph)
        }

    }
}

impl State {
    fn run_systems(&mut self){
        self.ecs.maintain();
    }
}

#[derive(PartialEq,Copy,Clone)]
enum TileType {
    Wall, Floor
}

pub fn xy_idx(x: i32, y: i32) -> usize {
    (y as usize * 80) + x as usize
}

fn new_map() -> Vec<TileType> {
    let mut map = vec![TileType::Floor; 80*50];

    for x in 0..80 {
        map[xy_idx(x,0)] = TileType::Wall;
        map[xy_idx(x,49)] = TileType::Wall
    }
    for y in 0..50 {
        map[xy_idx(0,y)] = TileType::Wall;
        map[xy_idx(79, y)] = TileType::Wall;
    }

    let mut rng = RandomNumberGenerator::new();
    for _i in 0..100 {
        let x = rng.roll_dice(1, 79);
        let y = rng.roll_dice(1, 49);
        let idx = xy_idx(x, y);
        if idx != xy_idx(40, 25) {
            map[idx] = TileType::Wall;
        }
    }
    map
}

fn draw_map(map: &[TileType], ctx: &mut Rltk){
    let mut x = 0;
    let mut y = 0;
    for tile in map.iter(){
        match tile{
            TileType::Floor => {
                ctx.set(x,y,RGB::from_f32(0.5, 0.5, 0.5), RGB::from_f32(0., 0., 0.), rltk::to_cp437('.'))
            },
            TileType::Wall => {
                ctx.set(x,y,RGB::from_f32(0.0, 1.0, 1.0), RGB::from_f32(0.,0.,0.), rltk::to_cp437('#'))
            }
        }

        x += 1;
        if x > 79 {
            x = 0;
            y += 1;
        }
    }
}

#[derive(Component, Debug)]
struct Player {}

fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World){
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    
    for(pos, _player) in (&mut positions, &mut players).join(){
        let x = (delta_x + pos.x).clamp(0, 79);
        let y = (delta_y + pos.y).clamp(0, 49);
        let map = ecs.fetch::<Vec<TileType>>();
        
        if map[xy_idx(x,y,)] != TileType::Wall { // if not colliding with wall
            pos.x = (delta_x + pos.x).clamp(0, 79);
            pos.y = (delta_y + pos.y).clamp(0, 49);
        }
    }

}

fn player_input(gs: &mut State, ctx: &mut Rltk){
    match ctx.key {
        None => {},
        Some(key) => match key {
            VirtualKeyCode::Left => try_move_player(-1, 0, &mut gs.ecs),
            VirtualKeyCode::Right => try_move_player(1, 0, &mut gs.ecs),
            VirtualKeyCode::Up => try_move_player(0, -1, &mut gs.ecs),
            VirtualKeyCode::Down => try_move_player(0, 1, &mut gs.ecs),
            _ => {}
        }
    }
}


fn main() -> rltk::BError {
    use rltk::RltkBuilder;
    let context = RltkBuilder::simple80x50()
        .with_title("BananaLike")
        .build()?;
    let mut gs = State{
        ecs: World::new()
    };
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();

    gs.ecs.insert(new_map());

    gs.ecs
        .create_entity()
        .with(Position { x: 40, y: 25 })
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
        })
        .with(Player {})
        .build();


    rltk::main_loop(context, gs)

}
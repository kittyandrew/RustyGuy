extern crate sdl2;

// use sdl2::render::{TextureCreator, Texture};
use sdl2::image::{InitFlag, LoadTexture};
use sdl2::keyboard::{Keycode, Scancode};
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::rect::Rect;
// use sdl2::ttf::Font;
use std::time::Duration;

mod entities;
use entities::{GameContext, Object, Agent};


const TRASHGUY: &'static str = "(> ^_^)>";
const GRAVITY: i32 = 3;
const MAX_VELOCITY: i32 = 120;
const WINDOW_W: u32 = 1500;
const WINDOW_H: u32 = 1000;
const PLAYER_W: u32 = 128;
const PLAYER_H: u32 = 64;


fn handle_events(event_pump: &mut sdl2::EventPump, context: &mut GameContext) {
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit {..} |
            Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                context.running = false;
                break
            },
            _ => {},
        }
    }
    for code in event_pump.keyboard_state().pressed_scancodes() {
        match code {
            Scancode::W  |
            Scancode::Up => {
                if context.players[0].is_jump { continue }
                context.players[0].is_jump = true;
                context.players[0].vv += 34;
            },
            Scancode::D     |
            Scancode::Right => {
                if context.players[0].is_left { context.players[0].is_left = false; }
                context.players[0].hv += context.players[0].speed;
            },
            Scancode::A    |
            Scancode::Left => {
                if !context.players[0].is_left { context.players[0].is_left = true; }
                context.players[0].hv -= context.players[0].speed;
            },
            _ => {}
        }
    }
}


fn render(context: &mut GameContext) {
    context.canvas.clear();
    context.apply_physics();
    context.render_scene();
    // println!("{:#?}", context.players[0].texture.query());
    context.canvas.present();
}


fn main() {
    let sdl_context = sdl2::init().unwrap();
    let ttf_context = sdl2::ttf::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG).unwrap();

    let window = video_subsystem.window("Trashguy", WINDOW_W, WINDOW_H)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();

    // let font = ttf_context.load_font("./assets/font.ttf", 72).unwrap();
    let font = ttf_context.load_font("./assets/wild.ttf", 72).unwrap();
    // Main Texture
    let partial = font.render(TRASHGUY);
    let surface = partial.solid((0, 0, 0, 255)).unwrap();
    // Display name of main character
    let name_font = ttf_context.load_font("./assets/wild.ttf", 26).unwrap();
    let dname_font = name_font.render("Andrew");
    let dname = dname_font.solid((0, 0, 0, 255)).unwrap();
    let hoe_font = name_font.render("Hoe");
    let hoe_name = hoe_font.solid((0, 0, 0, 255)).unwrap();

    let mut context = GameContext {
        canvas: &mut canvas,
        window_w: WINDOW_W,
        window_h: WINDOW_H,
        gravity: GRAVITY,
        max_vv: MAX_VELOCITY,
        players: vec![
            Agent {
                id: 0,
                texture: surface.as_texture(&texture_creator).unwrap(),
                // Original generate size: (272, 77)
                src: Rect::new(0, 0, 272-8, 77),
                pos: Rect::new(400 - 32, 500 - 32, PLAYER_W, PLAYER_H),
                name: dname.as_texture(&texture_creator).unwrap(),
                speed: 7,
                vv: 0,
                hv: 0,
                is_left: false,
                is_jump: false,
            },
            Agent {
                id: 1,
                texture: surface.as_texture(&texture_creator).unwrap(),
                // Original generate size: (272, 77)
                src: Rect::new(0, 0, 272-8, 77),
                pos: Rect::new((WINDOW_W / 2 - 500) as i32, 500 - 32, PLAYER_W, PLAYER_H),
                name: hoe_name.as_texture(&texture_creator).unwrap(),
                speed: 7,
                vv: 0,
                hv: 0,
                is_left: false,
                is_jump: false,
            },
        ],
        platform: Object {
            texture: texture_creator.load_texture("./assets/platform.png").unwrap(),
            src: Rect::new(0, 1350, 3072, 186),
            pos: Rect::new(0, (WINDOW_H - 100) as i32, WINDOW_W, 100),
        },
        background: Object {
            texture: texture_creator.load_texture("./assets/background.png").unwrap(),
            src: Rect::new(300, 200, 3000, 2400),
            pos: Rect::new(0, 0, WINDOW_W, WINDOW_H),
        },
        running: true,
    };

    context.canvas.set_draw_color(Color::RGBA(0, 0, 0, 255));
    context.canvas.clear();

    // let mut timer = sdl_context.timer().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    while context.running {
        handle_events(&mut event_pump, &mut context);
        render(&mut context);
        //let ticks = timer.ticks() as i32;
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}


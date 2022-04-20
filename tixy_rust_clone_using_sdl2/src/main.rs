const GRADATION_MODE: bool = false; // グラデーションにするか？
const SIDE_SIZE: f32 = 16.0; // 辺の長さ
const DIAMETER_RATE: f32 = 0.9; // セルの辺の最大値(比率)
const VIEW_SIZE_RATE: f32 = 0.95; // 画面に対する表示領域の大きさ
const COLOR_MAX: i32 = 255; // 色の要素の最大
const FPS: u32 = 60; // 決め打ち

const SCREEN_WIDTH: u32 = 480;
const SCREEN_HEIGHT: u32 = 480;

pub struct Item {
    name: &'static str,
    favorite: bool,
    func: fn(f32, f32, f32, f32) -> f32,
}

mod items_factory;

#[derive(Default)]
struct Vec2 {
    x: f32,
    y: f32,
}

impl Vec2 {
    fn scale(&self, value: f32) -> Vec2 {
        Vec2 {
            x: self.x * value,
            y: self.y * value,
        }
    }

    fn add(&self, other: &Vec2) -> Vec2 {
        Vec2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    fn sub(&self, other: &Vec2) -> Vec2 {
        Vec2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

#[derive(Default)]
struct Model {
    srect: Vec2,
    cell_wh: Vec2, // 一つのセルの縦横
    top_left: Vec2,
    counter: usize,
    item_index: isize, // items の index
    items: Vec<Item>,  // いろんな式を入れとく
}

impl Model {
    fn setup_vars(&mut self) {
        self.cell_wh = self.srect.scale((1.0 / SIDE_SIZE) * VIEW_SIZE_RATE); // 画面の大きさから1つのセルのサイズを求める
        self.top_left = self
            .srect
            .scale(0.5)
            .sub(&self.cell_wh.scale(SIDE_SIZE * 0.5)); // 左上
    }

    fn half_cell_wh(&self) -> Vec2 {
        self.cell_wh.scale(0.5)
    }

    fn time(&self) -> f32 {
        self.counter as f32 / FPS as f32
    }

    fn current_item(&self) -> &Item {
        &self.items[self.item_index as usize]
    }

    fn item_change(&mut self, sign: isize) {
        self.item_index = self.items.len() as isize + self.item_index + sign;
        self.item_index %= self.items.len() as isize;
        self.counter = 0;

        println!(
            "{:?} {:?}",
            self.current_item().name,
            self.current_item().favorite
        );
    }

    fn retval_to_color(&self, retval: f32) -> (u8, u8, u8) {
        let v: f32;
        if GRADATION_MODE {
            v = retval
        } else {
            if retval > 0.0 {
                v = 1.0
            } else {
                v = -1.0
            }
        }
        let c = (v.abs() * COLOR_MAX as f32) as u8;
        if v > 0.0 {
            (c, c, c)
        } else {
            (0, c, 0)
        }
    }

    fn retval_to_radius_rate(&self, retval: f32) -> f32 {
        retval.abs() * DIAMETER_RATE
    }
}

extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Tixy Rust clone using SDL2", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();

    let mut model = Model {
        srect: Vec2 {
            x: SCREEN_WIDTH as f32,
            y: SCREEN_HEIGHT as f32,
        },
        items: items_factory::items_factory(),
        ..Default::default()
    };
    model.setup_vars();

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::Q),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => model.item_change(1),
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => model.item_change(-1),
                _ => {}
            }
        }

        let mut i = 0.0;
        let t = model.time();
        let item = model.current_item();
        for y in 0..SIDE_SIZE as usize {
            for x in 0..SIDE_SIZE as usize {
                let x = x as f32;
                let y = y as f32;
                let retval = (item.func)(t, i, x, y);
                if retval != 0.0 {
                    let retval = retval.clamp(-1.0, 1.0);
                    let v = Vec2 {
                        x: model.cell_wh.x * x,
                        y: model.cell_wh.y * y,
                    };
                    let v = model.top_left.add(&v);
                    let radius = model
                        .half_cell_wh()
                        .scale(model.retval_to_radius_rate(retval)); // 楕円の半径 = 最大半径 * 割合
                    let center = v.add(&model.half_cell_wh()); // セルの中心
                    let v2 = center.sub(&radius); // 長方形の左上
                    let (r, g, b) = model.retval_to_color(retval);
                    canvas.set_draw_color(Color::RGB(r, g, b));
                    let rect = sdl2::rect::Rect::new(
                        v2.x as i32,
                        v2.y as i32,
                        (radius.x * 2.0) as u32,
                        (radius.y * 2.0) as u32,
                    );
                    let _ = canvas.fill_rect(rect);
                }
                i += 1.0;
            }
        }

        model.counter += 1;
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / FPS)); // FIXME: VSYNC同期で切り替えるべき
    }
}

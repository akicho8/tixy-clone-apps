const CELL_N: f32 = 16.0; // 辺のセル個数
const COLOR_MAX: i32 = 255; // 色の要素の最大
const DIAMETER_RATE: f32 = 0.9; // セルの辺の最大値(比率)
const PADDING: f32 = 16.0; // 余白
const DEBUG_MODE: bool = true; // 画面上の情報
const GRADATION_MODE: bool = false; // グラデーションにするか？

const SCREEN_WIDTH: u32 = 640;
const SCREEN_HEIGHT: u32 = 480;

use nannou::geom::Rect;
use nannou::prelude::*;

pub struct Item {
    name: &'static str,
    favorite: bool,
    func: fn(f32, f32, f32, f32) -> f32,
}

mod items_factory;

struct Model {
    view_rect: Rect,     // 表示領域
    cell_wh: Vec2,       // 一つのセルの縦横
    start_time: f32,     // 切り替えた時点の時間
    favorite_only: bool, // お気に入りだけ
    item_index: isize,   // items の index
    items: Vec<Item>,    // いろんな式を入れとく
}

impl Model {
    fn current_item(&self) -> &Item {
        &self.items[self.item_index as usize]
    }

    fn item_change(&mut self, app: &App, sign: isize) {
        self.item_index = self.items.len() as isize + self.item_index + sign;
        self.item_index %= self.items.len() as isize;
        self.start_time = app.time;
    }

    fn favorite_only_toggle(&mut self) {
        self.favorite_only = !self.favorite_only;
        self.items = items_factory::items_factory();
        self.item_index = 0;
        if self.favorite_only {
            self.items = items_factory::items_factory()
                .into_iter()
                .filter(|e| e.favorite)
                .collect();
        }
    }

    // 長方形の辺の比率を返す
    fn retval_to_diameter_rate(&self, retval: f32) -> f32 {
        retval.abs() * DIAMETER_RATE
    }

    // 長方形の辺の長さ
    fn cell_diameter(&self, retval: f32) -> Vec2 {
        self.cell_wh * self.retval_to_diameter_rate(retval)
    }

    // 関数の結果から色を決定
    fn retval_to_color(&self, retval: f32) -> Rgb8 {
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
            rgb8(c, c, c)
        } else {
            rgb8((c as f32 * 0.8) as u8, 0, c)
        }
    }
}

fn main() {
    nannou::app(model)
        .event(event)
        .update(update)
        .run();
}

fn model(app: &App) -> Model {
    app.new_window()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .mouse_pressed(mouse_pressed)
        .view(view)
        .build()
        .unwrap();

    app.main_window().set_title("Tixy Rust clone using nannou");

    let mut model = Model {
        view_rect: Rect::from_w_h(0.0, 0.0),
        cell_wh: Vec2::ZERO,
        start_time: 0.0,
        favorite_only: false,
        item_index: 0,
        items: items_factory::items_factory(),
    };

    model.item_index = (model.items.len() - 1) as isize;
    model
}

fn event(app: &App, model: &mut Model, event: Event) {
    if let Event::WindowEvent {
        simple: Some(KeyPressed(key)),
        ..
    } = event
    {
        match key {
            Key::Z => model.item_change(&app, 1),
            Key::X => model.item_change(&app, -1),
            Key::Left => model.item_change(&app, -1),
            Key::Right => model.item_change(&app, 1),
            Key::F => model.favorite_only_toggle(),
            Key::R => model.item_change(&app, 0),
            Key::Q => app.quit(),
            Key::Key3 => app.set_loop_mode(LoopMode::rate_fps(30.0)), // 効いてない
            Key::Key6 => app.set_loop_mode(LoopMode::rate_fps(60.0)), // 効いてない
            _ => {}
        }
    }
}

fn mouse_pressed(app: &App, model: &mut Model, button: MouseButton) {
    match button {
        MouseButton::Left => model.item_change(&app, 1),
        MouseButton::Right => model.item_change(&app, -1),
        _ => {}
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    model.view_rect = app.window_rect().pad(PADDING);
    model.cell_wh = model.view_rect.wh() / CELL_N; // 画面の大きさから1つのセルのサイズを求める
}

fn view(app: &App, model: &Model, frame: Frame) {
    frame.clear(BLACK);

    let draw = app.draw();
    let mut i = 0.0;
    let t = app.time - model.start_time;
    let item = model.current_item();
    for y in 0..CELL_N as usize {
        for x in 0..CELL_N as usize {
            let xy = vec2(x as f32, y as f32);
            let retval = (item.func)(t, i, xy.x, xy.y);
            if retval != 0.0 {
                let retval = retval.clamp(-1.0, 1.0);
                let v = model.cell_wh * xy * vec2(1.0, -1.0); // セルの右上
                let v = model.view_rect.top_left() + v; // セルの集合の右上を足す
                let v = v + model.cell_wh * vec2(0.5, -0.5); // セルの中心に移動
                let color = model.retval_to_color(retval);
                let diameter = model.cell_diameter(retval); // 直径
                draw.rect().xy(v).wh(diameter).color(color); // xy は右上ではなく中心の座標(使いやすい)
            }
            i += 1.0;
        }
    }

    if DEBUG_MODE {
        let win = app.window_rect();
        let r = Rect::from_w_h(win.w(), 15.0).top_left_of(win.pad(0.0));
        draw.rect().xy(r.xy()).wh(r.wh()).rgba(0.0, 0.0, 0.0, 0.9);
        let text = format!(
            "{} {:.2} FPS:{:.0} {} {}",
            frame.nth(),
            app.time,
            app.fps(),
            model.item_index,
            model.current_item().name,
        );
        draw.text(&text)
            .xy(r.xy())
            .wh(r.wh())
            .left_justify()
            .align_text_top()
            .color(WHITE);
    }

    draw.to_frame(app, &frame).unwrap();
}

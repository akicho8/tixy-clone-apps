const GRADATION_MODE: bool = false;   // グラデーションにするか？
const SIDE_SIZE: f32       = 16.0;    // 辺の長さ
const VIEW_SIZE_RATE: f32  = 1.0;    // 画面に対する表示領域の大きさ
const COLOR_MAX: i32       = 255;     // 色の要素の最大
// const FPS: u32             = 60;      // 決め打ち

const S_WIDTH: u32  = 600;
const S_HEIGHT: u32 = 400;

use nannou::prelude::*;

use rand::Rng;

#[derive(Default)]
struct Model {
    srect: Vec2,
    cell_wh: Vec2,
    half_cell_wh: Vec2,
    top_left: Vec2,
    counter: usize,
    preset_index: isize,
    preset_list: Vec<fn(f32, f32, f32, f32) -> f32>,
    // preset_list<F: Fn()>: Vec<F>,
    // rng: rand::ThreadRng,

    // preset_index: isize,
}

fn main() {
    // nannou::app(model).run();
    nannou::app(model).update(update).run();
    // nannou::app(model).simple_window(view).size(S_WIDTH, S_HEIGHT).run();
}

fn func1(t: f32, i: f32, x: f32, y: f32) -> f32 {
    t.sin()
}
fn func2(t: f32, i: f32, x: f32, y: f32) -> f32 {
    rand::thread_rng().gen_range(-1.0..=1.0)
}
fn func3(t: f32, i: f32, x: f32, y: f32) -> f32 {
    (t - ((x - 7.5).pow(2.0)+(y - 6.0).pow(2.0)).sqrt()).sin()
}

fn model(app: &App) -> Model {
    // イベントハンドラなどを設定
    app.new_window()
        .size(S_WIDTH, S_HEIGHT)
        .mouse_pressed(mouse_pressed)
        .view(view)
        .build()
        .unwrap();

    let mut model = Model {
        srect: vec2(S_WIDTH as f32, S_HEIGHT as f32),
        ..Default::default()
    };

    model.cell_wh      = model.srect * ((1.0 / SIDE_SIZE) * VIEW_SIZE_RATE);            // 画面の大きさから1つのセルのサイズを求める
    model.half_cell_wh = model.cell_wh * 0.5;                                         // 扱いやすいように半分バージョンも作っておく
    model.top_left     = vec2(model.cell_wh.x * -0.5 * SIDE_SIZE, model.cell_wh.y * 0.5 * SIDE_SIZE);

    // model.rng = rand::thread_rng();

    // model.preset_list = vec![
    //     |t:f32, i:f32, x:f32, y:f32| { model.rng.gen_range(-1.0..=1.0); },
    //     // |t:f32, i:f32, x:f32, y:f32| { my_sin(t) },
    // ];

    // model.preset_list.push(&func1 as fn());
    // model.preset_list = vec![
    //     func1 as fn(f32, f32, f32, f32) -> f32,
    //     func2 as fn(f32, f32, f32, f32) -> f32,
    //     func3 as fn(f32, f32, f32, f32) -> f32,
    // ];

    model.preset_list = vec![
        |t, i, x, y| t.sin(),
        |t, i, x, y| rand::thread_rng().gen_range(-1.0..=1.0),
        |t, i, x, y| (t - ((x - 7.5).pow(2.0)+(y - 6.0).pow(2.0)).sqrt()).sin(),
    ];
    // model.preset_list.push(|t, i, x, y| { t.sin() });
    // }
    // fn func2(t: f32, i: f32, x: f32, y: f32) -> f32 {
    //     rand::thread_rng().gen_range(-1.0..=1.0)
    // }
    // fn func3(t: f32, i: f32, x: f32, y: f32) -> f32 {
    //  
    // }
    model.preset_index = 2;

    model

}

fn mouse_pressed(_app: &App, model: &mut Model, _button: MouseButton) {
    // model.
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.counter += 1;
    // println!("{:?}", model.counter);
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(BLACK);

    // アプリケーションが起動してからの秒数を t に格納
    let t = app.time;

    // sin、cos を使って円運動を表現
    let center = vec2(t.cos(), t.sin()) * 100.0;

    let mut index = 0;

    for y in 0..SIDE_SIZE as usize {
        for x in 0..SIDE_SIZE as usize {
            let x = x as f32;
            let y = y as f32;

            // println!("{:?}", app.time());

            // let mut retval = func_call(t, index as f32, x as f32, y as f32);
            // let mut retval = func_call(app.time(), index as f32, x as f32, y as f32);
            let idx = model.preset_index as usize % model.preset_list.len();
            let func = model.preset_list[idx as usize];
            let mut retval = func(app.time, index as f32, x, y);

            // let mut retval = t.sin();

            // let mut retval: f32 = rand::thread_rng().gen_range(-1.0..=1.0);

            if retval != 0.0 {
                retval = retval.clamp(-1.0, 1.0);
                // let v = @top_left + @cell_wh.map2([x, y]) { |a, b| a * b }
                let v = model.cell_wh * vec2(x, -y);
                let v = model.top_left + v;

                let center = vec2(v.x + model.half_cell_wh.x, v.y - model.half_cell_wh.y);    // セルの中心
                // let v2 = vec2(center.x - radius.x, center.y - radius.y);                                        // 長方形の左上
                let v2 = center;

                let color = value_to_color(retval);
                // canvas.set_draw_color(Color::RGB(r, g, b));
                // let rect = sdl2::rect::Rect::new(v2.x as i32, v2.y as i32, (radius.x*2.0) as u32, (radius.y*2.0) as u32);
                // let _ = canvas.fill_rect(rect);

                let radius = model.cell_wh * value_to_radius_rate(retval);  // 楕円の半径 = 最大半径 * 割合

                draw.rect().xy(v2).wh(radius).color(color);

                // @renderer.draw_color =
                //         @renderer.fill_rect(SDL2::Rect.new(*v2, *(radius*2)))  # v2 から [radius, radius] の長方形を描画
            }
            index += 1;
        }
    }

    draw.to_frame(app, &frame).unwrap();

}

//   # 楕円の半径の割り合いを返す
fn value_to_radius_rate(rv: f32) -> f32 {
    rv.abs() * 0.9
}

// 0 は来ない
fn value_to_color(v: f32) -> Rgb8 {
    let v2: f32;
    if GRADATION_MODE {
        v2 = v
    } else {
        if v > 0.0 {
            v2 = 1.0
        } else {
            v2 = -1.0
        }
    }
    let c = (v2.abs() * COLOR_MAX as f32) as u8;
    if v2 > 0.0 {
        rgb8(c, c, c)
    } else {
        rgb8(0, c, 0)
    }
}

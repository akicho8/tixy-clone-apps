const GRADATION_MODE: bool = false; // グラデーションにするか？
const SIDE_N: f32 = 16.0; // 辺のセル個数
const COLOR_MAX: i32 = 255; // 色の要素の最大
const DIAMETER_RATE: f32 = 0.9; // セルの辺の最大値(比率)
const PADDING: f32 = 16.0; // 余白
                           // const FPS: u32             = 60;  // 決め打ち

const SCREEN_WIDTH: u32 = 640;
const SCREEN_HEIGHT: u32 = 480;

use nannou::geom::Rect;
use nannou::prelude::*;
use rand::Rng;

struct Pattern {
    favorite: bool,
    name: String,
    func: fn(f32, f32, f32, f32) -> f32,
}

struct Model {
    view_rect: Rect,
    cell_wh: Vec2,
    half_cell_wh: Vec2,
    counter: usize,
    start_time: f32,
    selected_index: isize,
    patterns: Vec<Pattern>,
}

impl Model {
    fn current_pattern(&self) -> &Pattern {
        &self.patterns[self.selected_index as usize]
    }

    fn preset_change(&mut self, app: &App, sign: isize) {
        self.selected_index = (self.patterns.len() as isize + self.selected_index + sign)
            % self.patterns.len() as isize;
        self.start_time = app.time;
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
            rgb8(0, c, (c as f32 * 0.8) as u8)
        }
    }
}

fn btof(v: bool) -> f32 {
    if v {
        1.0
    } else {
        0.0
    }
}

fn main() {
    nannou::app(model).event(event).update(update).run();
}

fn model(app: &App) -> Model {
    app.new_window()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .mouse_pressed(mouse_pressed)
        .view(view)
        .build()
        .unwrap();

    let mut model = Model {
        view_rect: Rect::from_w_h(0.0, 0.0),
        cell_wh: Vec2::ZERO,
        half_cell_wh: Vec2::ZERO,
        counter: 0,
        start_time: 0.0,
        selected_index: 0,
        patterns: vec![
            Pattern {
                name: "default".to_string(),
                favorite: false,
                func: |_t, _i, _x, _y| (_y / 8.0 + _t).sin(),
            },
            Pattern {
                name: "for every dot return 0 or 1 to change the visibility".to_string(),
                favorite: false,
                func: |_t, _i, _x, _y| {
                    btof(rand::random::<f32>() < 0.1)
                },
            },
            Pattern {
                name: "use a float between 0 and 1 to define the size".to_string(),
                favorite: false,
                func: |_t, _i, _x, _y| rand::random::<f32>(),
            },
            Pattern {
                name: "parameter `t` is the time in seconds".to_string(),
                favorite: false,
                func: |_t, _i, _x, _y| _t.sin(),
            },
            Pattern {
                name: "parameter `i` is the index of the dot (0..255)".to_string(),
                favorite: false,
                func: |_t, _i, _x, _y| _i / 256.0,
            },
            Pattern {
                name: "`x` is the column index from 0 to 15".to_string(),
                favorite: false,
                func: |_t, _i, _x, _y| _x / 16.0,
            },
            Pattern {
                name: "`y` is the row also from 0 to 15".to_string(),
                favorite: false,
                func: |_t, _i, _x, _y| _y / 16.0,
            },
            Pattern {
                name: "positive numbers are white, negatives are red".to_string(),
                favorite: false,
                func: |_t, _i, _x, _y| _y - 7.5,
            },
            Pattern {
                name: "use the time to animate values".to_string(),
                favorite: false,
                func: |_t, _i, _x, _y| _y - _t,
            },
            Pattern {
                name: "multiply the time to change the speed".to_string(),
                favorite: false,
                func: |_t, _i, _x, _y| _y - _t * 4.0,
            },
            Pattern {
                name: "create PresetInfo using different color".to_string(),
                favorite: false,
                func: |_t, _i, _x, _y| [1.0, 0.0, -1.0][(_i % 3.0) as usize],
            },
            Pattern {
                name: "skip `Math` to use methods".to_string(),
                favorite: true,
                func: |_t, _i, _x, _y| {
                    (_t - ((_x - 7.5).pow(2.0) + (_y - 6.0).pow(2.0)).sqrt()).sin()
                },
            },
            Pattern {
                name: "more examples".to_string(),
                favorite: false,
                func: |_t, _i, _x, _y| (_y / 8.0 + _t).sin(),
            },
            Pattern {
                name: "simple triangle".to_string(),
                favorite: false,
                func: |_t, _i, _x, _y| _y - _x,
            },
            Pattern {
                favorite: false,
                name: "quarter triangle".to_string(),
                func: |_t, _i, _x, _y| { btof((_y > _x) && ((14.0 - _x) < _y)) },
            },
            Pattern {
                name: "pattern".to_string(),
                favorite: false,
                func: |_t, _i, _x, _y| _i % 4.0 - _y % 4.0,
            },
            Pattern {
                name: "grid".to_string(),
                favorite: false,
                func: |_t, _i, _x, _y| {
                    if (_i % 4.0) > 0.0 && (_y % 4.0) > 0.0 {
                        1.0
                    } else {
                        0.0
                    }
                },
            },
            Pattern {
                name: "square".to_string(),
                favorite: false,
                func: |_t, _i, _x, _y| {
                    if _x > 3.0 && _y > 3.0 && _x < 12.0 && _y < 12.0 {
                        1.0
                    } else {
                        0.0
                    }
                },
            },
            Pattern {
                name: "animated square".to_string(),
                favorite: false,
                func: |_t, _i, _x, _y| {
                    if _x > _t && _y > _t && _x < 15.0 - _t && _y < 15.0 - _t {
                        -1.0
                    } else {
                        0.0
                    }
                },
            },
            Pattern {
                name: "mondrian squares".to_string(),
                favorite: false,
                func: |_t, _i, _x, _y| (_y - 6.0) * (_x - 6.0),
            },
            Pattern {
                name: "moving cross".to_string(),
                favorite: true,
                func: |_t, _i, _x, _y| (_y - 4.0 * _t) * (_x - 2.0 - _t),
            },
            Pattern {
                name: "sierpinski".to_string(),
                favorite: false,
                func: |_t, _i, _x, _y| {
                    if ((4.0 * _t) as isize & _i as isize & _x as isize & _y as isize) != 0 {
                        1.0
                    } else {
                        0.0
                    }
                },
            },
            Pattern {
                name: "binary clock".to_string(),
                favorite: false,
                func: |_t, _i, _x, _y| {
                    if _y == 8.0 {
                        if ((_t * 10.0) as isize & (1 << (_x as usize))) != 0 {
                            1.0
                        } else {
                            0.0
                        }
                    } else {
                        0.0
                    }
                },
            },
            Pattern {
                name: "random noise".to_string(),
                favorite: false,
                func: |_t, _i, _x, _y| rand::thread_rng().gen_range(-1.0..=1.0),
            },
            Pattern {
                name: "static smooth noise".to_string(),
                favorite: false,
                func: |_t, _i, _x, _y| _i.pow(2.0).sin(),
            },
            Pattern {
                name: "animated smooth noise".to_string(),
                favorite: true,
                func: |_t, _i, _x, _y| (_t + _i + _x * _y).cos(),
            },
            Pattern {
                name: "waves".to_string(),
                favorite: true,
                func: |_t, _i, _x, _y| (_x / 2.0).sin() - (_x - _t).sin() - _y + 6.0,
            },
            Pattern {
                name: "bloop bloop bloop by @v21".to_string(),
                favorite: true,
                func: |_t, _i, _x, _y| (_x - 8.0) * (_y - 8.0) - _t.sin() * 64.0,
            },
            Pattern {
                name: "fireworks by @p_malin and @aemkei".to_string(),
                favorite: true,
                func: |_t, _i, _x, _y| -0.4 / ((_x - _t % 10.0).hypot(_y - _t % 8.0) - _t % 2.0 * 9.0),
            },
            Pattern {
                name: "ripples by @thespite".to_string(),
                favorite: true,
                func: |_t, _i, _x, _y| (_t - (_x * _x + _y * _y).sqrt()).sin(),
            },
            Pattern {
                favorite: true,
                name: "3d checker board by @p_malin".to_string(),
                func: |_t, _i, _x, _y| {
                    if _y > 0.0 {
                        (((_x-8.0) / _y + _t*5.0) as usize & 1 ^ (1.0/_y*8.0) as usize & 1) as f32 * _y / 5.0
                    } else {
                        0.0
                    }
                },
            },
            Pattern {
                name: "scrolling TIXY font by @atesgoral".to_string(),
                favorite: true,
                func: |_t, _i, _x, _y| {
                    let x = _x as usize;
                    let y = _y as usize;
                    let pos = y + ((_t * 9.0) as usize);
                    if let Some(v) = [5463, 2194, 2386].get(pos & 7) {
                        if (v & (1 << x)) != 0 {
                            1.0
                        } else {
                            0.0
                        }
                    } else {
                        0.0
                    }
                },
            },
            // Pattern {
            //     name: "sticky blood by @joeytwiddle".to_string(),
            //     favorite: false,
            //     func: |_t, _i, _x, _y| {
            //         _y - _t * 3.0 + 9.0 + 3.0 * (_x * 3.0 - _t).cos() - 5.0 * (_x * 7.0).sin()
            //     },
            // },
            // // Pattern { favorite: true,  name: "3d starfield by @p_malin".to_string(),                             func: |_t, _i, _x, _y| d=_y*_y%5.9+1;(((_x+_t*50/d).to_i&15).zero? ? 1/d : 0)             , },
            // Pattern {
            //     name: "dialogue with an alien by @chiptune".to_string(),
            //     favorite: false,
            //     func: |_t, _i, _x, _y| 1.0 / 32.0 * (_t / 64.0 * _x * (_i - _x).tan()).tan(),
            // },
            // // Pattern { favorite: true,  name: "space invader by @keithclarkcouk + @zozuar".to_string(),           func: |_t, _i, _x, _y| 'p}¶¼<¼¶}p'.codepoints[_x] & 2**_y.to_i                        , },
            // // Pattern { favorite: true,  name: "hungry pac man by @p_malin and @aemkei".to_string(),               func: |_t, _i, _x, _y| hypot(_x-=_t%4*5,_y-=8)<6 && (_x<_y || _y<-_x)                        , },
            // // Pattern { favorite: false, name: "spectrum analyser by @joeytwiddle".to_string(),                    func: |_t, _i, _x, _y| _x.to_i.even? && _y < 9 && _y > (4 + sin(8*_t+_x*_x) + _x / 4)        , },
            // // Pattern { favorite: false, name: "diagonals".to_string(),                                            func: |_t, _i, _x, _y| _y == _x || ((15-_x == _y) ? -1 : 0 )                              , },
            // // Pattern { favorite: false, name: "frame".to_string(),                                                func: |_t, _i, _x, _y| _x==0 || _x==15 || _y==0 || _y==15                                 , },
            // // Pattern { favorite: true,  name: "drop".to_string(),                                                 func: |_t, _i, _x, _y| 8*_t%13 - hypot(_x-7.5, _y-7.5)                                   , },
            // Pattern {
            //     name: "rotation".to_string(),
            //     favorite: true,
            //     func: |_t, _i, _x, _y| (2.0 * ((_y - 7.5) / (_x - 7.5)).atan() + 5.0 * _t).sin(),
            // },
            // Pattern {
            //     name: "wipe".to_string(),
            //     favorite: true,
            //     func: |_t, _i, _x, _y| (_x - _y) - _t.sin() * 16.0,
            // },
            // Pattern {
            //     name: "soft wipe".to_string(),
            //     favorite: false,
            //     func: |_t, _i, _x, _y| (_x - _y) / 24.0 - _t.sin(),
            // },
            // Pattern {
            //     name: "disco".to_string(),
            //     favorite: false,
            //     func: |_t, _i, _x, _y| (_t * 5.0).sin() * (_t * 7.0).tan(),
            // },
            // Pattern {
            //     name: "input is limited to 32 characters!".to_string(),
            //     favorite: false,
            //     func: |_t, _i, _x, _y| (_x - 5.0).pow(2.0) + (_y - 5.0).pow(2.0) - 99.0 * _t.sin(),
            // },
        ],
    };

    model.selected_index = (model.patterns.len() - 1) as isize;
    model
}

fn event(app: &App, model: &mut Model, event: Event) {
    if let Event::WindowEvent {
        simple: Some(KeyPressed(key)),
        ..
    } = event
    {
        match key {
            Key::Z => model.preset_change(&app, 1),
            Key::X => model.preset_change(&app, -1),
            Key::Left => model.preset_change(&app, -1),
            Key::Right => model.preset_change(&app, 1),
            Key::R => model.preset_change(&app, 0),
            Key::Q => app.quit(),
            Key::Key3 => app.set_loop_mode(LoopMode::rate_fps(30.0)), // 動かない
            Key::Key6 => app.set_loop_mode(LoopMode::rate_fps(60.0)),
            _ => {}
        }
    }
}

fn mouse_pressed(app: &App, model: &mut Model, button: MouseButton) {
    match button {
        MouseButton::Left => model.preset_change(&app, 1),
        MouseButton::Right => model.preset_change(&app, -1),
        _ => {}
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    model.view_rect = app.window_rect().pad(PADDING);
    model.cell_wh = model.view_rect.wh() / SIDE_N; // 画面の大きさから1つのセルのサイズを求める
    model.half_cell_wh = model.cell_wh * vec2(0.5, -0.5); // 扱いやすいように半分バージョンも作っておく

    if false {
        if app.keys.down.contains(&Key::Return) {
            model.preset_change(&app, 1);
        }
    }

    // app.keys.down
    //     if frame.nth() == 0 func: || app.keys.down.contains(&Key::R) { },
    //     draw.background().color(BLACK);
    // }

    model.counter += 1;
    // model.start_time += 1;
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    // draw.background().color(BLACK);
    frame.clear(BLACK);

    let mut i = 0.0;
    let t = app.time - model.start_time;
    let pattern = model.current_pattern();
    for y in 0..SIDE_N as usize {
        for x in 0..SIDE_N as usize {
            let xy = vec2(x as f32, y as f32);
            let retval = (pattern.func)(t, i, xy.x, xy.y);
            if retval != 0.0 {
                let retval = retval.clamp(-1.0, 1.0);
                let v = model.cell_wh * xy * vec2(1.0, -1.0); // セルの右上
                let v = model.view_rect.top_left() + v; // セルの集合の右上を足す
                let v = v + model.half_cell_wh; // セルの中心
                let color = model.retval_to_color(retval);
                let diameter = model.cell_diameter(retval); // 直径
                draw.rect().xy(v).wh(diameter).color(color); // xy は右上ではなく中心の座標(使いやすい)
            }
            i += 1.0;
        }
    }

    let win = app.window_rect();
    let r = Rect::from_w_h(win.w(), 15.0).top_left_of(win.pad(0.0));
    draw.rect().xy(r.xy()).wh(r.wh()).rgba(0.0, 0.0, 0.0, 0.9);
    let text = format!(
        "{} {} {:.2} FPS:{:.0} {} {}",
        model.counter,
        frame.nth(),
        app.time,
        app.fps(),
        model.selected_index,
        model.current_pattern().name,
    );
    draw.text(&text)
        .xy(r.xy())
        .wh(r.wh())
        .left_justify()
        .align_text_top()
        .color(WHITE);

    draw.to_frame(app, &frame).unwrap();
}

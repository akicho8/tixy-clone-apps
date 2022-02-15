const GRADATION_MODE: bool = false;  // グラデーションにするか？
const SIDE_N: f32          = 16.0;   // 辺のセル個数
const VIEW_SIZE_RATE: f32  = 1.0;    // 画面に対する表示領域の大きさ
const COLOR_MAX: i32       = 255;    // 色の要素の最大
const DIAMETER_RATE: f32   = 0.9;    // セルの辺の最大値(比率)
const PADDING: f32         = 16.0;   // 余白
// const FPS: u32             = 60;  // 決め打ち

const SCREEN_WIDTH: u32  = 600;
const SCREEN_HEIGHT: u32 = 400;

use nannou::prelude::*;
use nannou::geom::Rect;
use rand::Rng;

struct Model {
    view_rect: Rect,
    cell_wh: Vec2,
    half_cell_wh: Vec2,
    counter: usize,
    selected_index: isize,
    patterns: Vec<fn(f32, f32, f32, f32) -> f32>,
}

impl Model {
    fn preset_change(&mut self, sign: isize) {
        self.selected_index += sign
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

    // app.set_loop_mode(LoopMode::rate_fps(30.0)); ← 効かない

    // let btoi = |v| {
    //     if v { 1.0 } else { 0.0 }
    // };

    let mut model = Model {
        view_rect: Rect::from_w_h(0.0, 0.0),
        cell_wh: Vec2::ZERO,
        half_cell_wh: Vec2::ZERO,
        counter: 0,
        selected_index: 0,
        patterns: vec![
            |_t, _i, _x, _y| if rand::random::<f32>() < 0.1 { 1.0 } else { 0.0 },
            |_t, _i, _x, _y| rand::random::<f32>(),
            |_t, _i, _x, _y| _t.sin(),
            |_t, _i, _x, _y| _i / 256.0,
            |_t, _i, _x, _y| _x / 16.0,
            |_t, _i, _x, _y| _y / 16.0,
            |_t, _i, _x, _y| _y - 7.5,
            |_t, _i, _x, _y| _y - _t,
            |_t, _i, _x, _y| _y - _t * 4.0,
            |_t, _i, _x, _y| [1.0, 0.0, -1.0][(_i % 3.0) as usize],
            |_t, _i, _x, _y| (_t - ((_x - 7.5).pow(2.0) + (_y - 6.0).pow(2.0)).sqrt()).sin(),
            |_t, _i, _x, _y| (_y / 8.0 + _t).sin(),
            |_t, _i, _x, _y| _y - _x,
            // |_t, _i, _x, _y| { btoi((_y > _x) && btoi((14.0 - _x) < _y)) },
            |_t, _i, _x, _y| _i % 4.0 - _y % 4.0,
            |_t, _i, _x, _y| if (_i % 4.0) > 0.0 && (_y % 4.0) > 0.0 { 1.0 } else { 0.0 },
            |_t, _i, _x, _y| if _x > 3.0 && _y > 3.0 && _x < 12.0 && _y < 12.0 { 1.0 } else { 0.0 },
            |_t, _i, _x, _y| if _x > _t && _y > _t && _x < 15.0 - _t && _y < 15.0 - _t { -1.0 } else { 0.0 },
            |_t, _i, _x, _y| (_y - 6.0) * (_x - 6.0),
            |_t, _i, _x, _y| (_y - 4.0 * _t) * (_x - 2.0 - _t),
            |_t, _i, _x, _y| if ((4.0 * _t) as isize & _i as isize & _x as isize & _y as isize) != 0 { 1.0 } else { 0.0 },
            |_t, _i, _x, _y| if _y == 8.0 { if ((_t * 10.0) as isize & (1<<(_x as usize))) != 0 {1.0} else {0.0} } else { 0.0 },
            |_t, _i, _x, _y| rand::thread_rng().gen_range(-1.0..=1.0),
            |_t, _i, _x, _y| _i.pow(2.0).sin(),
            |_t, _i, _x, _y| (_t + _i + _x * _y).cos(),
            |_t, _i, _x, _y| (_x / 2.0).sin() - (_x - _t).sin() - _y + 6.0,
            |_t, _i, _x, _y| (_x - 8.0) * (_y - 8.0) - _t.sin() * 64.0,
            // |_t, _i, _x, _y| -0.4 / hypot(_x-_t % 10.0, _y-_t%8.0) - _t % 2.0 * 9.0,
            |_t, _i, _x, _y| (_t - (_x*_x+_y*_y).sqrt()).sin()                                           ,
            //         |_t, _i, _x, _y| [5463,2194,2386][_y.to_i + (_t*9).to_i & 7] & (1 << _x-1)         ,
            //         |_t, _i, _x, _y| _y>0 && (((_x-8) / _y + _t*5).to_i & 1 ^ (1/_y*8).to_i & 1) * _y / 5 ,
            //         |_t, _i, _x, _y| _y-_t*3+9+3*cos(_x*3-_t)-5*sin(_x*7)                                ,
            //         |_t, _i, _x, _y| d=_y*_y%5.9+1;(((_x+_t*50/d).to_i&15).zero? ? 1/d : 0)             ,
            //         |_t, _i, _x, _y| 1.0/32.0*tan(_t/64.0*_x*tan(_i-_x))                                ,
            //         |_t, _i, _x, _y| 'p}¶¼<¼¶}p'.codepoints[_x] & 2**_y.to_i                        ,
            // |_t, _i, _x, _y| hypot(_x-=_t%4*5,_y-=8)<6 && (_x<_y || _y<-_x)                        ,
            // |_t, _i, _x, _y| _x.to_i.even? && _y < 9 && _y > (4 + sin(8*_t+_x*_x) + _x / 4)        ,
            // |_t, _i, _x, _y| _y == _x || ((15-_x == _y) ? -1 : 0 )                              ,
            // |_t, _i, _x, _y| _x==0 || _x==15 || _y==0 || _y==15                                 ,
            // |_t, _i, _x, _y| 8*_t%13 - hypot(_x-7.5, _y-7.5)                                   ,
            // |_t, _i, _x, _y| sin(2*atan((_y-7.5)/(_x-7.5))+5*_t)                               ,
            // |_t, _i, _x, _y| (_x-_y) - sin(_t) * 16                                            ,
            // |_t, _i, _x, _y| (_x-_y)/24 - sin(_t)                                              ,
            // |_t, _i, _x, _y| sin(_t*5) * tan(_t*7)                                            ,
            // |_t, _i, _x, _y| (_x-5)**2 + (_y-5)**2 - 99*sin(_t)                                ,

            //     { favorite: false, name: "default",                                              func: -> (t, i, x, y) { sin(y/8+t)                                                     }},
            //     { favorite: false, name: "for every dot return 0 or 1 to change the visibility", func: -> (t, i, x, y) { rand < 0.1                                                     }},
            //     { favorite: false, name: "use a float between 0 and 1 to define the size",       func: -> (t, i, x, y) { rand                                                           }},
            //     { favorite: false, name: "parameter `t` is the time in seconds",                 func: -> (t, i, x, y) { sin(t)                                                         }},
            //     { favorite: false, name: "parameter `i` is the index of the dot (0..255)",       func: -> (t, i, x, y) { i / 256                                                        }},
            //     { favorite: false, name: "`x` is the column index from 0 to 15",                 func: -> (t, i, x, y) { x / 16                                                         }},
            //     { favorite: false, name: "`y` is the row also from 0 to 15",                     func: -> (t, i, x, y) { y / 16                                                         }},
            //     { favorite: false, name: "positive numbers are white, negatives are red",        func: -> (t, i, x, y) { y - 7.5                                                        }},
            //     { favorite: false, name: "use the time to animate values",                       func: -> (t, i, x, y) { y - t                                                          }},
            //     { favorite: false, name: "multiply the time to change the speed",                func: -> (t, i, x, y) { y - t*4                                                        }},
            //     { favorite: false, name: "create PresetInfo using different color",              func: -> (t, i, x, y) { [1, 0, -1][i%3]                                                }},
            //     { favorite: true,  name: "skip `Math` to use methods",                           func: -> (t, i, x, y) { sin(t-sqrt((x-7.5)**2+(y-6)**2))                               }},
            //     { favorite: false, name: "more examples",                                        func: -> (t, i, x, y) { sin(y/8 + t)                                                   }},
            //     { favorite: false, name: "simple triangle",                                      func: -> (t, i, x, y) { y - x                                                          }},
            //     { favorite: false, name: "quarter triangle",                                     func: -> (t, i, x, y) { (y > x) && (14-x < y)                                          }},
            //     { favorite: false, name: "pattern",                                              func: -> (t, i, x, y) { i%4 - y%4                                                      }},
            //     { favorite: false, name: "grid",                                                 func: -> (t, i, x, y) { (i%4)>0 && (y%4)>0                                             }},
            //     { favorite: false, name: "square",                                               func: -> (t, i, x, y) { x>3 && y>3 && x<12 && y<12                                     }},
            //     { favorite: false, name: "animated square",                                      func: -> (t, i, x, y) { (x>t && y>t && x<15-t && y<15-t) ? -1 : 0                      }},
            //     { favorite: false, name: "mondrian squares",                                     func: -> (t, i, x, y) { (y-6) * (x-6)                                                  }},
            //     { favorite: true,  name: "moving cross",                                         func: -> (t, i, x, y) { (y-4*t) * (x-2-t)                                              }},
            //     { favorite: false, name: "sierpinski",                                           func: -> (t, i, x, y) { (4*t).to_i & i.to_i & x.to_i & y.to_i                          }},
            //     { favorite: false, name: "binary clock",                                         func: -> (t, i, x, y) { y==8 && (t*10).to_i & (1<<x)                                   }},
            //     { favorite: false, name: "random noise",                                         func: -> (t, i, x, y) { rand(-1.0..1.0)                                                }},
            //     { favorite: false, name: "static smooth noise",                                  func: -> (t, i, x, y) { sin(i**2)                                                      }},
            //     { favorite: true,  name: "animated smooth noise",                                func: -> (t, i, x, y) { cos(t + i + x * y)                                             }},
            //     { favorite: true,  name: "waves",                                                func: -> (t, i, x, y) { sin(x/2) - sin(x-t) - y+6                                      }},
            //     { favorite: true,  name: "bloop bloop bloop by @v21",                            func: -> (t, i, x, y) { (x-8)*(y-8) - sin(t)*64                                        }},
            //     { favorite: true,  name: "fireworks by @p_malin and @aemkei",                    func: -> (t, i, x, y) { -0.4/(hypot(x-t%10,y-t%8)-t%2*9)                               }},
            //     { favorite: true,  name: "ripples by @thespite",                                 func: -> (t, i, x, y) { sin(t-sqrt(x*x+y*y))                                           }},
            //     { favorite: true,  name: "scrolling TIXY font by @atesgoral",                    func: -> (t, i, x, y) { [5463,2194,2386][y.to_i + (t*9).to_i & 7] & (1 << x-1)         }},
            //     { favorite: true,  name: "3d checker board by @p_malin",                         func: -> (t, i, x, y) { y>0 && (((x-8) / y + t*5).to_i & 1 ^ (1/y*8).to_i & 1) * y / 5 }},
            //     { favorite: false, name: "sticky blood by @joeytwiddle",                         func: -> (t, i, x, y) { y-t*3+9+3*cos(x*3-t)-5*sin(x*7)                                }},
            //     { favorite: true,  name: "3d starfield by @p_malin",                             func: -> (t, i, x, y) { d=y*y%5.9+1;(((x+t*50/d).to_i&15).zero? ? 1/d : 0)             }},
            //     { favorite: false, name: "dialogue with an alien by @chiptune",                  func: -> (t, i, x, y) { 1.0/32.0*tan(t/64.0*x*tan(i-x))                                }},
            //     { favorite: true,  name: "space invader by @keithclarkcouk + @zozuar",           func: -> (t, i, x, y) { 'p}¶¼<¼¶}p'.codepoints[x] & 2**y.to_i                        }},
            //     { favorite: true,  name: "hungry pac man by @p_malin and @aemkei",               func: -> (t, i, x, y) { hypot(x-=t%4*5,y-=8)<6 && (x<y || y<-x)                        }},
            //     { favorite: false, name: "spectrum analyser by @joeytwiddle",                    func: -> (t, i, x, y) { x.to_i.even? && y < 9 && y > (4 + sin(8*t+x*x) + x / 4)        }},
            //     { favorite: false, name: "diagonals",                                            func: -> (t, i, x, y) { y == x || ((15-x == y) ? -1 : 0 )                              }},
            //     { favorite: false, name: "frame",                                                func: -> (t, i, x, y) { x==0 || x==15 || y==0 || y==15                                 }},
            //     { favorite: true,  name: "drop",                                                 func: -> (t, i, x, y) { 8*t%13 - hypot(x-7.5, y-7.5)                                   }},
            //     { favorite: true,  name: "rotation",                                             func: -> (t, i, x, y) { sin(2*atan((y-7.5)/(x-7.5))+5*t)                               }},
            //     { favorite: true,  name: "wipe",                                                 func: -> (t, i, x, y) { (x-y) - sin(t) * 16                                            }},
            //     { favorite: false, name: "soft wipe",                                            func: -> (t, i, x, y) { (x-y)/24 - sin(t)                                              }},
            //     { favorite: false, name: "disco",                                                func: -> (t, i, x, y) { sin(t*5) * tan(t*7)                                            }},
            //     { favorite: false, name: "input is limited to 32 characters!",                   func: -> (t, i, x, y) { (x-5)**2 + (y-5)**2 - 99*sin(t)                                }},

        ],
    };

    model.selected_index = (model.patterns.len() - 1) as isize;
    model
}

fn event(app: &App, model: &mut Model, event: Event) {
    if let Event::WindowEvent { simple: Some(KeyPressed(key)), .. } = event {
        match key {
            Key::Z => model.preset_change(1),
            Key::X => model.preset_change(-1),
            Key::Q => app.quit(),
            _ => {},
        }
    }
}

fn mouse_pressed(_app: &App, model: &mut Model, button: MouseButton) {
    match button {
        MouseButton::Left  => { model.preset_change(1)  },
        MouseButton::Right => { model.preset_change(-1) },
        _ => {},
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    // let win = app.window_rect();
    // if model.counter == 0 {
    model.view_rect    = app.window_rect().pad(PADDING);
    model.cell_wh      = model.view_rect.wh() * ((1.0 / SIDE_N) * VIEW_SIZE_RATE); // 画面の大きさから1つのセルのサイズを求める
    model.half_cell_wh = model.cell_wh * 0.5 * vec2(1.0, -1.0);                 // 扱いやすいように半分バージョンも作っておく
    // }

    if app.keys.down.contains(&Key::Return) {
        model.preset_change(1);
    }

    // app.keys.down
    //     if frame.nth() == 0 || app.keys.down.contains(&Key::R) {
    //     draw.background().color(BLACK);
    // }
    model.counter += 1;
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(BLACK);
    // frame.clear(BLACK);

    let mut cell_index = 0;

    let idx = model.selected_index as usize % model.patterns.len();
    let func = model.patterns[idx as usize];
    for y in 0..SIDE_N as usize {
        for x in 0..SIDE_N as usize {
            let xy = vec2(x as f32, y as f32);
            let retval = func(app.time, cell_index as f32, xy.x, xy.y);
            if retval != 0.0 {
                let retval = retval.clamp(-1.0, 1.0);
                let v = model.cell_wh * xy * vec2(1.0, -1.0); // セルの右上
                let v = model.view_rect.top_left() + v;       // セルの集合の右上を足す
                let v = v + model.half_cell_wh;               // セルの中心
                let color = model.retval_to_color(retval);
                let diameter = model.cell_diameter(retval);   // 直径
                draw.rect().xy(v).wh(diameter).color(color);  // xy は右上ではなく中心の座標(使いやすい)
            }
            cell_index += 1;
        }
    }

    let win = app.window_rect();
    let r = Rect::from_w_h(win.w(), 15.0).top_left_of(win.pad(0.0));
    draw.rect()
        .xy(r.xy())
        .wh(r.wh())
        .rgba(0.0, 0.0, 0.0, 0.9);
    let text = format!("{} {} FPS:{:.0}", model.counter, frame.nth(), app.fps());
    draw.text(&text)
        .xy(r.xy())
        .wh(r.wh())
        .left_justify()
        .align_text_top()
        .color(WHITE);

    draw.to_frame(app, &frame).unwrap();
}

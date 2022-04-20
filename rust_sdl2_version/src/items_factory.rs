use super::*;
use rand::Rng;

pub fn items_factory() -> Vec<Item> {
    return vec![
        Item {
            name: "default",
            favorite: true,
            func: |_t, _i, _x, _y| (_y / 8.0 + _t).sin(),
        },
        Item {
            name: "for every dot return 0 or 1 to change the visibility",
            favorite: true,
            func: |_t, _i, _x, _y| bool_to_float(rand::random::<f32>() < 0.1),
        },
        Item {
            name: "use a float between 0 and 1 to define the size",
            favorite: false,
            func: |_t, _i, _x, _y| rand::random::<f32>(),
        },
        Item {
            name: "parameter `t` is the time in seconds",
            favorite: false,
            func: |_t, _i, _x, _y| _t.sin(),
        },
        Item {
            name: "parameter `i` is the index of the dot (0..255)",
            favorite: false,
            func: |_t, _i, _x, _y| _i / 256.0,
        },
        Item {
            name: "`x` is the column index from 0 to 15",
            favorite: false,
            func: |_t, _i, _x, _y| _x / 16.0,
        },
        Item {
            name: "`y` is the row also from 0 to 15",
            favorite: false,
            func: |_t, _i, _x, _y| _y / 16.0,
        },
        Item {
            name: "positive numbers are white, negatives are red",
            favorite: false,
            func: |_t, _i, _x, _y| _y - 7.5,
        },
        Item {
            name: "use the time to animate values",
            favorite: false,
            func: |_t, _i, _x, _y| _y - _t,
        },
        Item {
            name: "multiply the time to change the speed",
            favorite: false,
            func: |_t, _i, _x, _y| _y - _t * 4.0,
        },
        Item {
            name: "create patterns using different color",
            favorite: true,
            func: |_t, _i, _x, _y| [1.0, 0.0, -1.0][(_i % 3.0) as usize],
        },
        Item {
            name: "pow, sqrt example",
            favorite: true,
            func: |_t, _i, _x, _y| (_t - ((_x - 7.5).powi(2) + (_y - 6.0).powi(2)).sqrt()).sin(),
        },
        Item {
            name: "more examples",
            favorite: false,
            func: |_t, _i, _x, _y| (_y / 8.0 + _t).sin(),
        },
        Item {
            name: "simple triangle",
            favorite: false,
            func: |_t, _i, _x, _y| _y - _x,
        },
        Item {
            name: "quarter triangle",
            favorite: false,
            func: |_t, _i, _x, _y| bool_to_float((_y > _x) && ((14.0 - _x) < _y)),
        },
        Item {
            name: "item",
            favorite: false,
            func: |_t, _i, _x, _y| _i % 4.0 - _y % 4.0,
        },
        Item {
            name: "grid",
            favorite: false,
            func: |_t, _i, _x, _y| {
                if (_i % 4.0) > 0.0 && (_y % 4.0) > 0.0 {
                    1.0
                } else {
                    0.0
                }
            },
        },
        Item {
            name: "square",
            favorite: false,
            func: |_t, _i, _x, _y| {
                if _x > 3.0 && _y > 3.0 && _x < 12.0 && _y < 12.0 {
                    1.0
                } else {
                    0.0
                }
            },
        },
        Item {
            name: "animated square",
            favorite: false,
            func: |_t, _i, _x, _y| {
                if _x > _t && _y > _t && _x < 15.0 - _t && _y < 15.0 - _t {
                    -1.0
                } else {
                    0.0
                }
            },
        },
        Item {
            name: "mondrian squares",
            favorite: false,
            func: |_t, _i, _x, _y| (_y - 6.0) * (_x - 6.0),
        },
        Item {
            name: "moving cross",
            favorite: true,
            func: |_t, _i, _x, _y| (_y - 4.0 * _t) * (_x - 2.0 - _t),
        },
        Item {
            name: "sierpinski",
            favorite: false,
            func: |_t, _i, _x, _y| {
                if ((4.0 * _t) as isize & _i as isize & _x as isize & _y as isize) != 0 {
                    1.0
                } else {
                    0.0
                }
            },
        },
        Item {
            name: "binary clock",
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
        Item {
            name: "random noise",
            favorite: false,
            func: |_t, _i, _x, _y| rand::thread_rng().gen_range(-1.0..=1.0),
        },
        Item {
            name: "static smooth noise",
            favorite: false,
            func: |_t, _i, _x, _y| _i.powi(2).sin(),
        },
        Item {
            name: "animated smooth noise",
            favorite: true,
            func: |_t, _i, _x, _y| (_t + _i + _x * _y).cos(),
        },
        Item {
            name: "waves",
            favorite: true,
            func: |_t, _i, _x, _y| (_x / 2.0).sin() - (_x - _t).sin() - _y + 6.0,
        },
        Item {
            name: "bloop bloop bloop by @v21",
            favorite: true,
            func: |_t, _i, _x, _y| (_x - 8.0) * (_y - 8.0) - _t.sin() * 64.0,
        },
        Item {
            name: "fireworks by @p_malin and @aemkei",
            favorite: true,
            func: |_t, _i, _x, _y| -0.4 / ((_x - _t % 10.0).hypot(_y - _t % 8.0) - _t % 2.0 * 9.0),
        },
        Item {
            name: "ripples by @thespite",
            favorite: true,
            func: |_t, _i, _x, _y| (_t - (_x * _x + _y * _y).sqrt()).sin(),
        },
        Item {
            name: "3d checker board by @p_malin",
            favorite: true,
            func: |_t, _i, _x, _y| {
                if _y > 0.0 {
                    (((_x - 8.0) / _y + _t * 5.0) as usize & 1 ^ (1.0 / _y * 8.0) as usize & 1)
                        as f32
                        * _y
                        / 5.0
                } else {
                    0.0
                }
            },
        },
        Item {
            name: "scrolling TIXY font by @atesgoral",
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
        Item {
            name: "sticky blood by @joeytwiddle",
            favorite: false,
            func: |_t, _i, _x, _y| {
                _y - _t * 3.0 + 9.0 + 3.0 * (_x * 3.0 - _t).cos() - 5.0 * (_x * 7.0).sin()
            },
        },
        Item {
            name: "3d starfield by @p_malin",
            favorite: true,
            func: |_t, _i, _x, _y| {
                let d = _y * _y % 5.9 + 1.0;
                if ((_x + _t * 50.0 / d) as usize & 15) == 0 {
                    1.0 / d
                } else {
                    0.0
                }
            },
        },
        Item {
            name: "dialogue with an alien by @chiptune",
            favorite: true,
            func: |_t, _i, _x, _y| 1.0 / 32.0 * (_t / 64.0 * _x * (_i - _x).tan()).tan(),
        },
        Item {
            name: "space invader by @keithclarkcouk + @zozuar",
            favorite: true,
            func: |_t, _i, _x, _y| {
                if let Some(ch) = "p}¶¼<¼¶}p".chars().nth(_x as usize) {
                    if (ch as i32 & 2_i32.pow(_y as u32)) != 0 {
                        1.0
                    } else {
                        0.0
                    }
                } else {
                    0.0
                }
            },
        },
        Item {
            name: "hungry pac man by @p_malin and @aemkei",
            favorite: true,
            func: |_t, _i, _x, _y| {
                let x = _x - _t % 4.0 * 5.0;
                let y = _y - 8.0;
                if x.hypot(y) < 6.0 && (x < y || y < -x) {
                    1.0
                } else {
                    0.0
                }
            },
        },
        Item {
            name: "spectrum analyser by @joeytwiddle",
            favorite: false,
            func: |_t, _i, _x, _y| {
                if (_x as usize) % 2 == 0
                    && _y < 9.0
                    && _y > (4.0 + (8.0 * _t + _x * _x).sin() + _x / 4.0)
                {
                    1.0
                } else {
                    0.0
                }
            },
        },
        Item {
            name: "diagonals",
            favorite: false,
            func: |_t, _i, _x, _y| {
                if _y == _x {
                    1.0
                } else if 15.0 - _x == _y {
                    -1.0
                } else {
                    0.0
                }
            },
        },
        Item {
            name: "frame",
            favorite: false,
            func: |_t, _i, _x, _y| {
                if _x == 0.0 || _x == 15.0 || _y == 0.0 || _y == 15.0 {
                    1.0
                } else {
                    0.0
                }
            },
        },
        Item {
            name: "drop",
            favorite: true,
            func: |_t, _i, _x, _y| 8.0 * _t % 13.0 - (_x - 7.5).hypot(_y - 7.5),
        },
        Item {
            name: "rotation",
            favorite: true,
            func: |_t, _i, _x, _y| (2.0 * ((_y - 7.5) / (_x - 7.5)).atan() + 5.0 * _t).sin(),
        },
        Item {
            name: "wipe",
            favorite: true,
            func: |_t, _i, _x, _y| (_x - _y) - _t.sin() * 16.0,
        },
        Item {
            name: "soft wipe",
            favorite: true,
            func: |_t, _i, _x, _y| (_x - _y) / 24.0 - _t.sin(),
        },
        Item {
            name: "disco",
            favorite: false,
            func: |_t, _i, _x, _y| (_t * 5.0).sin() * (_t * 7.0).tan(),
        },
        Item {
            name: "input is limited to 32 characters!",
            favorite: false,
            func: |_t, _i, _x, _y| (_x - 5.0).powi(2) + (_y - 5.0).powi(2) - 99.0 * _t.sin(),
        },
    ];
}

fn bool_to_float(v: bool) -> f32 {
    if v {
        1.0
    } else {
        0.0
    }
}

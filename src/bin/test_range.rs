// https://docs.rs/nannou/latest/nannou/prelude/geom/struct.Range.html

use nannou::geom::Range;

fn main() {
    println!("{:?}", Range { start: 2, end: 5 });                                    // Range { start: 2, end: 5 }
    println!("{:?}", Range::new(2, 5));                                              // Range { start: 2, end: 5 }
    println!("{:?}", Range::from_pos_and_len(5.0, 4.0));                             // Range { start: 3.0, end: 7.0 }
    println!("{:?}", Range::new(1, 5).magnitude());                                  // 4
    println!("{:?}", Range::new(1, 5).len());                                        // 4
    println!("{:?}", Range::new(1.0, 2.0).middle());                                 // 1.5
    println!("{:?}", Range::new(1, 5).invert());                                     // Range { start: 5, end: 1 }
    println!("{:?}", Range::new(1.0, 5.0).lerp(1.0));                                // 5.0
    println!("{:?}", Range::new(2.0, 3.0).shift(1.0));                               // Range { start: 3.0, end: 4.0 }
    println!("{:?}", Range::new(2.0, 2.0).direction());                              // 0.0
    println!("{:?}", Range::new(-2.0, -2.0).absolute());                             // Range { start: -2.0, end: -2.0 }
    println!("{:?}", Range::new(2.0, 3.0).max(Range::new(2.0, 3.0)));                // Range { start: 2.0, end: 3.0 }
    println!("{:?}", Range::new(2.0, 3.0).overlap(Range::new(2.0, 3.0)));            // Some(Range { start: 2.0, end: 3.0 })
    println!("{:?}", Range::new(2.0, 3.0).max_directed(Range::new(2.0, 3.0)));       // Range { start: 2.0, end: 3.0 }
    println!("{:?}", Range::new(2.0, 3.0).contains(2.0));                            // true
    println!("{:?}", Range::new(2.1, 3.1).round());                                  // Range { start: 2.0, end: 3.0 }
    println!("{:?}", Range::new(2.1, 3.1).floor());                                  // Range { start: 2.0, end: 3.0 }
    println!("{:?}", Range::new(0.0, 5.0).pad_start(1.0));                           // Range { start: 1.0, end: 5.0 }
    println!("{:?}", Range::new(0.0, 5.0).pad_end(1.0));                             // Range { start: 0.0, end: 4.0 }
    println!("{:?}", Range::new(0.0, 5.0).pad(1.0));                                 // Range { start: 1.0, end: 4.0 }
    println!("{:?}", Range::new(0.0, 5.0).pad_ends(1.0, 1.0));                       // Range { start: 1.0, end: 4.0 }
    println!("{:?}", Range::new(0.0, 5.0).clamp_value(4.0));                         // 4.0
    println!("{:?}", Range::new(0.0, 5.0).stretch_to_value(6.0));                    // Range { start: 0.0, end: 6.0 }
    println!("{:?}", Range::new(0.0, 5.0).has_same_direction(Range::new(0.0, 5.0))); // true
    println!("{:?}", Range::new(0.0, 10.0).closest_edge(4.0));                       // Start
    // println!("{:?}", Range::new(0.0, 5.0).align_to(Align::Middle, Range::new(0.0, 5.0)));

    let a = Range::new(2.5, 7.5);
    let b = Range::new(0.0, 10.0);
    println!("{:?}", a.align_start_of(b));  // Range { start: 0.0, end: 5.0 }
    println!("{:?}", b.align_start_of(a));  // Range { start: 2.5, end: 12.5 }

    let a = Range::new(2.5, 7.5);
    let b = Range::new(0.0, 10.0);
    println!("{:?}", a.align_end_of(b));    // Range { start: 5.0, end: 10.0 }
    println!("{:?}", b.align_end_of(a));    // Range { start: -2.5, end: 7.5 }

    let a = Range::new(0.0, 5.0);
    let b = Range::new(0.0, 10.0);
    println!("{:?}", a.align_middle_of(b)); // Range { start: 2.5, end: 7.5 }
    println!("{:?}", b.align_middle_of(a)); // Range { start: -2.5, end: 7.5 }

    let a = Range::new(2.5, 7.5);
    let b = Range::new(0.0, 10.0);
    println!("{:?}", a.align_after(b));     // Range { start: 10.0, end: 15.0 }
    println!("{:?}", b.align_after(a));     // Range { start: 7.5, end: 17.5 }

    let a = Range::new(2.5, 7.5);
    let b = Range::new(0.0, 10.0);
    println!("{:?}", a.align_before(b));    // Range { start: -5.0, end: 0.0 }
    println!("{:?}", b.align_before(a));    // Range { start: -7.5, end: 2.5 }

    // let a = Range::new(0.0, 5.0);
    // let b = Range::new(0.0, 10.0);
    // println!("{:?}", a.map_value(2.5, &b));
}

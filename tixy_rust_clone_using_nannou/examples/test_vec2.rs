// https://docs.rs/nannou/latest/nannou/glam/struct.Vec2.html#impl-Mul%3CVec2%3E
// rustxmp_playground: "nannou"

use nannou::prelude::*;
use nannou::geom::Vertex2d;

fn main() {
    println!("{:?}", Vec2::ZERO);                                             // => Vec2(0.0, 0.0)
    println!("{:?}", Vec2::ONE);                                              // => Vec2(1.0, 1.0)
    println!("{:?}", Vec2::X);                                                // => Vec2(1.0, 0.0)
    println!("{:?}", Vec2::Y);                                                // => Vec2(0.0, 1.0)
    println!("{:?}", Vec2::AXES);                                             // => [Vec2(1.0, 0.0), Vec2(0.0, 1.0)]
    println!("{:?}", vec2(2f32, 3f32));                                       // => Vec2(2.0, 3.0)
    println!("{:?}", vec2(2.0, 3.0).extend(4.0));                             // => Vec3(2.0, 3.0, 4.0)
    println!("{:?}", vec2(2.0, 3.0).to_array());                              // => [2.0, 3.0]
    println!("{:?}", Vec2::splat(2.0));                                       // => Vec2(2.0, 2.0)
    println!("{:?}", vec2(2.0, 3.0).dot(vec2(2.0, 3.0)));                     // => 13.0
    println!("{:?}", vec2(2.0, 3.0).min(vec2(1.0, 4.0)));                     // => Vec2(1.0, 3.0)
    println!("{:?}", vec2(2.0, 3.0).max(vec2(1.0, 4.0)));                     // => Vec2(2.0, 4.0)
    println!("{:?}", vec2(2.0, 5.0).clamp(vec2(3.0, 3.0), vec2(4.0, 4.0)));   // => Vec2(3.0, 4.0)
    println!("{:?}", vec2(2.0, 3.0).min_element());                           // => 2.0
    println!("{:?}", vec2(2.0, 3.0).max_element());                           // => 3.0
    println!("{:?}", vec2(2.0, 3.0).cmpeq(vec2(2.0, 3.0)));                   // => BVec2(0xffffffff, 0xffffffff)
    println!("{:?}", vec2(2.0, 3.0).cmpne(vec2(2.0, 3.0)));                   // => BVec2(0x0, 0x0)
    println!("{:?}", vec2(2.0, 3.0).cmpge(vec2(2.0, 3.0)));                   // => BVec2(0xffffffff, 0xffffffff)
    println!("{:?}", vec2(2.0, 3.0).cmpgt(vec2(2.0, 3.0)));                   // => BVec2(0x0, 0x0)
    println!("{:?}", vec2(2.0, 3.0).cmple(vec2(2.0, 3.0)));                   // => BVec2(0xffffffff, 0xffffffff)
    println!("{:?}", vec2(2.0, 3.0).cmplt(vec2(2.0, 3.0)));                   // => BVec2(0x0, 0x0)
    println!("{:?}", Vec2::from_slice(&[2.0, 3.0]));                          // => Vec2(2.0, 3.0)
    println!("{:?}", vec2(-2.0, -3.0).abs());                                 // => Vec2(2.0, 3.0)
    println!("{:?}", vec2(-2.0, 3.0).signum());                               // => Vec2(-1.0, 1.0)
    println!("{:?}", vec2(2.0, 3.0).perp());                                  // => Vec2(-3.0, 2.0)
    println!("{:?}", vec2(2.0, 3.0).perp_dot(vec2(2.0, 3.0)));                // => 0.0
    println!("{:?}", vec2(2.0, 3.0).is_finite());                             // => true
    println!("{:?}", vec2(2.0, f32::NAN).is_nan());                           // => true
    println!("{:?}", vec2(2.0, f32::NAN).is_nan_mask());                      // => BVec2(0x0, 0xffffffff)
    println!("{:?}", vec2(2.0, 3.0).length());                                // => 3.6055512
    println!("{:?}", vec2(2.0, 3.0).length_squared());                        // => 13.0
    println!("{:?}", vec2(2.0, 3.0).recip());                                 // => Vec2(0.5, 0.33333334)
    println!("{:?}", vec2(2.0, 3.0).distance(vec2(4.0, 5.0)));                // => 2.828427
    println!("{:?}", vec2(2.0, 3.0).distance_squared(vec2(4.0, 5.0)));        // => 8.0
    println!("{:?}", vec2(2.0, 3.0).normalize());                             // => Vec2(0.5547002, 0.8320503)
    println!("{:?}", vec2(2.0, 3.0).try_normalize());                         // => Some(Vec2(0.5547002, 0.8320503))
    println!("{:?}", vec2(2.0, 3.0).normalize_or_zero());                     // => Vec2(0.5547002, 0.8320503)
    println!("{:?}", vec2(2.0, 3.0).is_normalized());                         // => false
    println!("{:?}", vec2(2.0, 3.0).normalize().is_normalized());             // => true
    println!("{:?}", vec2(2.0, 3.0).project_onto(vec2(2.0, 3.0)));            // => Vec2(2.0, 3.0)
    println!("{:?}", vec2(2.0, 3.0).reject_from(vec2(1.0, 1.0)));             // => Vec2(-0.5, 0.5)
    println!("{:?}", vec2(2.0, 3.0).project_onto_normalized(vec2(1.0, 1.0))); // => Vec2(5.0, 5.0)
    println!("{:?}", vec2(2.0, 3.0).reject_from_normalized(vec2(1.0, 1.0)));  // => Vec2(-3.0, -2.0)
    println!("{:?}", vec2(2.4, 3.5).round());                                 // => Vec2(2.0, 4.0)
    println!("{:?}", vec2(2.4, 3.5).floor());                                 // => Vec2(2.0, 3.0)
    println!("{:?}", vec2(2.4, 3.5).ceil());                                  // => Vec2(3.0, 4.0)
    println!("{:?}", vec2(2.4, 3.5).fract());                                 // => Vec2(0.4000001, 0.5)
    println!("{:?}", vec2(2.0, 3.0).exp());                                   // => Vec2(7.389056, 20.085537)
    println!("{:?}", vec2(2.0, 3.0).powf(2.0));                               // => Vec2(4.0, 9.0)
    println!("{:?}", vec2(2.0, 3.0).recip());                                 // => Vec2(0.5, 0.33333334)
    println!("{:?}", vec2(2.0, 3.0).lerp(vec2(4.0, 5.0), 2.0));               // => Vec2(6.0, 7.0)
    println!("{:?}", vec2(2.0, 3.0).abs_diff_eq(vec2(4.0, 5.0), 2.0));        // => true
    println!("{:?}", vec2(2.0, 3.0).clamp_length(1.0, 2.0));                  // => Vec2(1.1094004, 1.6641006)
    println!("{:?}", vec2(2.0, 3.0).clamp_length_min(1.0));                   // => Vec2(2.0, 3.0)
    println!("{:?}", vec2(2.0, 3.0).clamp_length_max(2.0));                   // => Vec2(1.1094004, 1.6641006)
    println!("{:?}", vec2(2.0, 3.0).angle_between(vec2(4.0, 5.0)));           // => -0.08673801
    println!("{:?}", vec2(2.0, 3.0).as_f64());                                // => DVec2(2.0, 3.0)
    println!("{:?}", vec2(2.0, 3.0).as_i32());                                // => IVec2(2, 3)
    println!("{:?}", vec2(2.0, 3.0).as_u32());                                // => UVec2(2, 3)
    println!("{:?}", vec2(2.0, 3.0) + vec2(2.0, 3.0));                        // => Vec2(4.0, 6.0)
    println!("{:?}", vec2(2.0, 3.0) - vec2(2.0, 3.0));                        // => Vec2(0.0, 0.0)
    println!("{:?}", { let mut v = vec2(2.0, 3.0); v += v; v });              // => Vec2(4.0, 6.0)
    println!("{:?}", { let mut v = vec2(2.0, 3.0); v -= v; v });              // => Vec2(0.0, 0.0)
    println!("{:?}", { let mut v = vec2(2.0, 3.0); v *= v; v });              // => Vec2(4.0, 9.0)
    println!("{:?}", { let mut v = vec2(2.0, 3.0); v /= v; v });              // => Vec2(1.0, 1.0)
    println!("{:?}", { let mut v = vec2(2.0, 3.0); v = -v; v });              // => Vec2(-2.0, -3.0)
    println!("{:?}", vec2(2.0, 3.0).as_mut());                                // => [2.0, 3.0]
    println!("{:?}", vec2(2.0, 3.0).as_ref());                                // => [2.0, 3.0]
    println!("{:?}", vec2(2.0, 3.0).clone());                                 // => Vec2(2.0, 3.0)
    println!("{:?}", Vec2::default());                                        // => Vec2(0.0, 0.0)
    println!("{:?}", Vec2::from((2.0, 3.0)));                                 // => Vec2(2.0, 3.0)
    println!("{:?}", Vec2::from([2.0, 3.0]));                                 // => Vec2(2.0, 3.0)
    println!("{:?}", Vec2::from(vec2(2.0, 3.0)));                             // => Vec2(2.0, 3.0)
    println!("{:?}", Vec2::from(Vec3::new(2.0, 3.0, 4.0)));                   // => Vec2(2.0, 3.0)
    println!("{:?}", vec2(2.0, 3.0) == vec2(2.0, 3.0));                       // => true
    println!("{:?}", vec2(2.0, 3.0) != vec2(2.0, 3.0));                       // => false
    println!("{:?}", vec2(2.0, 3.0).angle());                                 // => 0.9827937
    println!("{:?}", vec2(2.0, 3.0).rotate(1.0));                             // => Vec2(-1.4438083, 3.3038487)
    println!("{:?}", vec2(2.0, 3.0).point2());                                // => [2.0, 3.0]
    println!("{:?}", vec2(2.0, 3.0).to_string());                             // => "[2, 3]"

    let v1 = vec2(2.0, 3.0);
    let mut v2 = vec2(4.0, 5.0);
    v2.clone_from(&v1);
    println!("{:?}", v2);       // => Vec2(2.0, 3.0)

    let mut v = [0.0, 0.0];
    vec2(2.0, 3.0).write_to_slice(&mut v);
    println!("{:?}", v);        // => [2.0, 3.0]
}

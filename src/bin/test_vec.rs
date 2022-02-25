// https://moshg.github.io/rust-std-ja/std/vec/struct.Vec.html

struct A {
    a: isize,
}

const FOO: Vec<A> = vec![
    A { a: 1 },
];

fn main() {
    // let v = vec![A { a: 1 }];
    // println!("{:?}", v[0].a);
}

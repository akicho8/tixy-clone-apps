fn main() {
    let v: &str = "foo";
    println!("{:?}", v);

    let v: &'static str = "foo";
    println!("{:?}", v);
}

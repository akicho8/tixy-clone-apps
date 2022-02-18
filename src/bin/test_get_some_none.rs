fn main() {
    let ary = ["a"];
    if let Some(v) = ary.get(1) {
        println!("{:?}", v);
    } else {
        println!("None");
    }
}

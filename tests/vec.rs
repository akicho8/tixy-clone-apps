#[test]
fn test_vec() {
    let mut v: Vec<fn(u32) -> u32> = Vec::new();
    v.push(|x| x);
    println!("{:?}", v);
}

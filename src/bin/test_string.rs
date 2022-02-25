fn main() {
    let v: &str = "foo";
    println!("{:?}", v);

    let v: &'static str = "foo";
    println!("{:?}", v);

    println!("{:?}", "p}¶¼<¼¶}p".bytes().nth(0));
    println!("{:?}", "p}¶¼<¼¶}p".bytes().nth(1));
    println!("{:?}", "p}¶¼<¼¶}p".bytes().nth(2));
    println!("{:?}", "p}¶¼<¼¶}p".chars().nth(0));
    println!("{:?}", "p}¶¼<¼¶}p".chars().nth(1));
    println!("{:?}", "p}¶¼<¼¶}p".chars().nth(2));
}

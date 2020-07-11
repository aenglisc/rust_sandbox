pub fn run() {
    let mut hello = String::from("hello");
    // let mut hello = "hello"; won't work
    println!("{}", hello);
    println!("{}", hello.len());
    hello.push(' ');
    hello.push_str("world!");
    println!("{}", hello);
    println!("{}", hello.len());
    println!("{}", hello.capacity());
    println!("{}", hello.is_empty());
    println!("{}", hello.contains("hello"));
    println!("{}", hello.replace("world", "there"));
    println!("{}", hello);
    for word in hello.split_whitespace() {
        println!("{}", word);
    }
    let mut kek = String::with_capacity(10);
    kek.push('a');
    kek.push('b');

    println!("{}", kek);

    assert_eq!(2, kek.len());
    assert_eq!(10, kek.capacity());
}

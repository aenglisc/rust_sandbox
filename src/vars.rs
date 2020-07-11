pub fn run() {
    let name = "Kek";
    let mut lulz = 42;
    println!("My name is {}, lulz: {}", name, lulz);

    lulz = 69;
    println!("My name is {}, lulz: {}", name, lulz);

    const ID: i32 = 001;
    println!("ID: {}", ID);

    let (foo, bar) = (1, 2);
    println!("foo: {}, bar: {}", foo, bar);
}

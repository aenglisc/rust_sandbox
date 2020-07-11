pub fn run() {
    println!("Hello print.rs");
    println!("Number: {}", 1);
    println!("{0} {1} {0}", "Axe", "is");
    println!("{name} {activity} {name}", name = "Axe", activity = "is");
    println!("Binary: {:b}", 42);
    println!("{:?}", (12, true, "hello"));
    println!("10 + 10 = {}", 10 + 10);
}

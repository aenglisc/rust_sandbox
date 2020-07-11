pub fn run() {
    greeting("Hello", "Pepe");
    let get_sum = add(69, 420);
    println!("{}", get_sum);

    let solo: i32 = 322;
    let add_nums = |x: i32, y: i32| x + y + solo;
    println!("{}", add_nums(228, 282));
}

fn greeting(greet: &str, name: &str) {
    println!("{}, {}", greet, name);
}

fn add(x: i32, y: i32) -> i32 {
    x + y
}

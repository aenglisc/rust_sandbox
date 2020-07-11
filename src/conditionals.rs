pub fn run() {
    let age = 42;
    let check_id: bool = false;
    let knows_age: bool = true;

    if age >= 21 && check_id || knows_age {
        println!("Bartender: drink?")
    } else if age < 21 && check_id {
        println!("Bartender: go away kid")
    } else {
        println!("Bartender: id?")
    }

    let is_of_age = if age >= 21 { true } else { false };
    println!("{}", is_of_age);
}

use std::env;

pub fn run() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let command = args[1].clone();
        let name = "Kek";

        if command == "hello" {
            println!("Hi, {}", name);
        } else if command == "status" {
            println!("All systems nominal");
        } else {
            println!("Invalid command");
        }
    } else {
        println!("No command");
    }
}

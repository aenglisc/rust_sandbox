enum Movement {
    Up,
    Down,
    Left,
    Right,
}

fn move_character(m: Movement) {
    match m {
        Movement::Up    => println!("Moving up"),
        Movement::Down  => println!("Moving down"),
        Movement::Left  => println!("Moving left"),
        Movement::Right => println!("Moving right"),
    }
}

pub fn run() {
    let character0 = Movement::Up;
    let character1 = Movement::Down;
    let character2 = Movement::Left;
    let character3 = Movement::Right;

    move_character(character0);
    move_character(character1);
    move_character(character2);
    move_character(character3);
}

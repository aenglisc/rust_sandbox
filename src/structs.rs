struct Colour {
    red: u8,
    green: u8,
    blue: u8,
}

struct SColour(u8, u8, u8);

struct Person {
    full_name: String,
    address_as: String,
}

impl Person {
    fn new(full_name: &str, address_as: &str) -> Person {
        Person {
            full_name: full_name.to_string(),
            address_as: address_as.to_string(),
        }
    }

    fn present(&self) -> String {
        format!("{} aka \"{}\"", self.full_name, self.address_as)
    }

    fn update_address_as(&mut self, address_as: &str) {
        self.address_as = address_as.to_string();
    }

    fn to_tuple(self) -> (String, String) {
        (self.full_name, self.address_as)
    }
}

pub fn run() {
    let mut c = Colour{
        red: 255,
        green: 0,
        blue: 0,
    };

    c.red = 200;
    println!("{}, {}, {}", c.red, c.green, c.blue);

    let mut sc = SColour(255, 0, 0);

    sc.0 = 200;
    println!("{}, {}, {}", sc.0, sc.1, sc.2);

    let mut p = Person::new("Topias Taavitsainen", "Topson");
    println!("{}, {}", p.full_name, p.address_as);
    println!("{}", p.present());
    p.update_address_as("Godson");
    println!("{}", p.present());
    println!("{:?}", p.to_tuple());
}

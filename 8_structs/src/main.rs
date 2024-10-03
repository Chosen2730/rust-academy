struct Composite {
    o_level: i32,
    utme: i32,
    name: String,
    number_of_sittings: i32,
}

impl Composite {
    fn check_o_level(&self) -> bool {
        if self.o_level > 30 {
            return false;
        } else {
            return true;
        }
    }
}

impl Composite {
    fn calculate_utme_value(&self) -> f32 {
        self.utme as f32 / 400 as f32 * 60 as f32
    }
}

impl Composite {
    fn get_sittings_score(&self) -> i32 {
        if self.number_of_sittings == 1 {
            10
        } else {
            0
        }
    }
}

fn main() {
    println!("Hello, world!");
    let user1 = Composite {
        o_level: 23,
        utme: 200,
        name: String::from("John Doe"),
        number_of_sittings: 1,
    };
    let user2 = Composite {
        o_level: 35,
        utme: 250,
        name: String::from("Jane Doe"),
        number_of_sittings: 2,
    };
    calculate_composite(&user1);
    println!("{}", user1.name);
    calculate_composite(&user2);
}

fn calculate_composite(user: &Composite) {
    let utme_value: f32 = user.calculate_utme_value();
    let sittings_point = user.get_sittings_score();
    let is_ok = user.check_o_level();
    if !is_ok {
        return println!("Your O level calculation is wrong");
    }
    let total_composite: f32 = sittings_point as f32 + utme_value + user.o_level as f32;

    println!(
        "Hello {}, your total composite is {}",
        user.name, total_composite
    )
}

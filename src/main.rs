trait Sellable {
    fn price(&self) -> u16;
    fn description(&self) -> String;
}

struct Sword {
    name: String,
    damage: u16,
    swing_time_ms: u16,
}

impl Sellable for Sword {
    fn price(&self) -> u16 {
        (self.damage * 1000_u16) / self.swing_time_ms * 10_u16
    }

    fn description(&self) -> String {
        format!(
            "{}, damage is {}, swing time: {}ms",
            self.name, self.damage, self.swing_time_ms
        )
    }
}

struct Shield {
    name: String,
    armor: u16,
    block: u16,
}

impl Sellable for Shield {
    fn price(&self) -> u16 {
        self.armor + self.block
    }

    fn description(&self) -> String {
        format!(
            "{}, armor: {}, block: {}ms",
            self.name, self.armor, self.block
        )
    }
}

fn vendor_text_static<T: Sellable>(item: &T) -> String {
    format!("I offer you {}, [{}g]", item.description(), item.price())
}

fn main() {
    let sword = Sword {
        name: "Sword of cowardise".into(),
        damage: 10,
        swing_time_ms: 1500,
    };

    let shield = Shield {
        name: "Golden Barrier".into(),
        armor: 50,
        block: 35,
    };

    println!("{}", vendor_text_static(&sword));
    println!("{}", vendor_text_static(&shield));
}

struct Redfox {
    enemy: bool,
    life: u32,
}

impl Noisy for Redfox {
    fn get_noise(&self) -> &str {
        "Meow?"
    }
}

impl Noisy for u8 {
    fn get_noise(&self) -> &str {
        "BYTE!"
    }
}

trait Noisy {
    fn get_noise(&self) -> &str;
}

fn print_noise<T: Noisy>(item: T) {
    println!("{}", item.get_noise());
}

fn main() {
    let fox = Redfox {
        enemy: true,
        life: 100,
    };
    print_noise(fox);
    print_noise(5_u8);
}

struct RedFox {
    enemy: bool,
    life: u8,
}

/* Implementation block */
impl RedFox {
    fn new() -> Self {
        Self {
            enemy: true,
            life: 70,
        }
    }
}

fn main() {
    let fox = RedFox {
        enemy: true,
        life: 70,
    };
    let another_fox = RedFox::new();
}

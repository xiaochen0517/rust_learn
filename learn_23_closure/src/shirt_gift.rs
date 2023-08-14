#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Green,
    Blue,
}

pub struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }
    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_green = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Green => num_green += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }

        if num_red > num_blue && num_red > num_green {
            ShirtColor::Red
        } else if num_blue > num_red && num_blue > num_green {
            ShirtColor::Blue
        } else {
            ShirtColor::Green
        }
    }
}

pub fn test_shirt_gift() {
    println!("====== test_shirt_gift ======");
    let store = Inventory {
        shirts: vec![ShirtColor::Red, ShirtColor::Red, ShirtColor::Blue, ShirtColor::Green],
    };

    let user_preference = Some(ShirtColor::Blue);
    let shirt = store.giveaway(user_preference);
    println!("shirt: {:?}", shirt);

    let user_preference = None;
    let shirt = store.giveaway(user_preference);
    println!("shirt: {:?}", shirt);
}
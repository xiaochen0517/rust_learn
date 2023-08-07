mod front_of_house;

use rand::Rng;

fn main() {
    for _ in 1..=10 {
        let random_number = rand::thread_rng().gen_range(1..=100);
        println!("Random number: {}", random_number);
    }
    front_of_house::hosting::add_to_waitlist();
}

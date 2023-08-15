use add_one;

fn main() {
    let num = 10;
    println!("Hello, world! {} plus one is {}!", num, add_one::add_one(num));
    let rand_num = add_one::get_rand_num();
    println!("Random number: {}", rand_num);
}

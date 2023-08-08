use std::collections::HashMap;

fn main() {
    basic_hashmap();
    hashmap_used_var();
    hashmap_get_var();
    hashmap_check_insert();
}

fn basic_hashmap() {
    println!("====== basic_hashmap ======");
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    scores.insert(String::from("Blue"), 25); // overwrite

    println!("map: {:?}", scores);
}

fn hashmap_used_var() {
    println!("====== hashmap_used_var ======");
    let mut map = HashMap::new();
    let mut team_name = String::from("Blue");
    let score = 10;
    map.insert(&team_name, &score);
    println!("map: {:?}", map);

    println!("team_name: {} | score: {}", team_name, score);
    team_name.push_str(" test");
    println!("team_name: {} | score: {}", team_name, score);
    // println!("map: {:?}", map); // error borrow of moved value: `team_name`
}

fn hashmap_get_var() {
    println!("====== hashmap_get_var ======");
    let mut map = HashMap::new();
    map.insert(String::from("Blue"), 10);
    map.insert(String::from("Yellow"), 50);
    let color_value = map.get("Blue").copied().unwrap_or(0);
    println!("color_value: {}", color_value);
    // let none_value = map.get("Red").expect("Red is not exist");
    // println!("none_value: {}", none_value);
}

fn hashmap_check_insert() {
    println!("====== hashmap_check_insert ======");
    let mut map = HashMap::new();
    map.insert(String::from("Blue"), 10);
    map.entry(String::from("Yellow")).or_insert(50);
    map.entry(String::from("Blue")).or_insert(25);
    println!("map: {:?}", map);
}

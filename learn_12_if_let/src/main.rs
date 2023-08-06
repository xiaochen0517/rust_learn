fn main() {
    let some_u8_value: Option<u8> = Some(1u8);
    // let some_u8_value: Option<u8> = Some(1u8);
    match some_u8_value {
        Some(max) => println!("value: {}", max),
        _ => (),
    }
    // use if let
    if let Some(max) = some_u8_value {
        println!("value: {}", max);
    }
}

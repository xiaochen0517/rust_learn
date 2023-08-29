use crate::button::Button;
use crate::screen::Screen;
use crate::text_field::TextField;

mod averaged_collection;
mod draw;
mod screen;
mod button;
mod text_field;

fn main() {
    test_averaged_collection();
    test_draw();
    test_while_let();
    test_range();
}

fn test_averaged_collection() {
    let mut ac = averaged_collection::AveragedCollection::new();
    ac.add(1);
    ac.add(2);
    ac.add(3);
    println!("average: {}", ac.average());
    ac.remove();
    println!("average: {}", ac.average());
    ac.remove();
    println!("average: {}", ac.average());
    ac.remove();
    println!("average: {}", ac.average());
}

fn test_draw() {
    println!("====== test_draw ======");
    let screen = Screen {
        components: vec![
            Box::new(Button::new(
                10,
                10,
                "test button".to_string(),
            )),
            Box::new(TextField::new(
                "text field".to_string(),
            )),
        ]
    };

    screen.run();
}

fn test_while_let() {
    println!("====== test_while_let ======");
    let mut arr: Vec<Option<i32>> = vec![
        Some(1),
        None,
        Some(2),
        Some(3),
    ];
    while let Some(Some(value)) = arr.pop() {
        println!("value = {:?}", value);
    }
}

fn test_range() {
    println!("====== test_range ======");
    for index in 0..=5 {
        println!("index = {}", index);
    }
    println!("----- none five ------");
    for index in 0..5 {
        println!("index = {}", index);
    }
}

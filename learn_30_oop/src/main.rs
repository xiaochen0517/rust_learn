mod averaged_collection;

fn main() {
    test_averaged_collection();
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

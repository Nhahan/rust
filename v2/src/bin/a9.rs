// Topic: Data management using tuples

fn coordinate() -> (i32, i32) {
    (1, 7)
}

fn main() {
    let (x, y) = coordinate();

    if y > 5 {
        println!("y > 5");
    } else if y < 5 {
        println!("y < 5");
    } else {
        println!("y = 5");
    }
}

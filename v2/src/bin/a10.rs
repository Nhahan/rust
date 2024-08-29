// Topic: Working with expression

fn print_message(gt_100: bool) {
    match gt_100 {
        true => println!("value is greater than 100"),
        false => println!("value is not greater than 100"),
    }
}

fn main() {
    let value = 100;
    let is_gt_100 = value > 100;
    print_message(is_gt_100);
}

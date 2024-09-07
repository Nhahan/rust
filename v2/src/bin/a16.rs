// Topic: Option

struct Student {
    name: String,
    locker: Option<i32>,
}

fn main() {
    let mary = Student {
        name: "Mary".to_string(),
        locker: Some(3),
    };
    println!("student: {:?}", mary.name);
    match mary.locker {
        Some(num) => println!("locker number: {}", num),
        None => println!("no locker assigned"),
    }
}

// Topic: Working with on enum

enum Color {
    Red,
    Yellow,
    Blue,
}

fn print_color(color: Color) {
    match color {
        Color::Red => println!("Red"),
        Color::Yellow => println!("Yellow"),
        Color::Blue => println!("Blue"),
    }
}

fn main() {
    print_color(Color::Red);
}

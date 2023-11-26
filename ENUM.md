## Enum

```rust
#[derive(Debug, PartialEq)]
enum Color {
    Red,
    Green,
    Blue,
}

fn main() {
    let red: Color = Color::Red;
    let green: Color = Color::Green;
    
    println("red = {:?}", red);
    println("red === green => {}", red == green); // false
    println!("red == red => {}", red == Color::Red); // true
}
```

## Option

```rust
fn main() {
    let some_number: Option<i32> = Some(5);
    let some_string: Option<&str> = Some("a string");
    
    println!("some_number: {:?}", some_number);
    println!("some_string: {:?}", some_string);
}
```
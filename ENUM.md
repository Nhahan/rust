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
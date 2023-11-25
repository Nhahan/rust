## 구조체 Struct

```rust
struct User {
    name: String,
    email: String,
    active: bool,
}

fn main() {
    let user = User {
        name: String::from("Hong"),
        email: String::from("gildong@example.com"),
        active: true,
    };
    
    println("유저 이름: {}", user.name); // Hong

    let mut user2 = User {
        name: String::from("Kim"),
        email: String::from("hello@example.com"),
        active: true,
    };

    user2.name = String::from("Park");

    println("유저 이름: {}", user2.name); // Park

    let user3 = {...user2}; // spread 사용 가능
}
```

## 구조체 Struct 예제

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect = Rectangle {
        width: 20,
        height: 30,
    };
    
    prntln!("사각형의 면적은 {}", area(*rect));
    
    println!("Debug / 사각형: {:?}", rect);
}

fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
    
}
```

## 구조체 Struct

```rust
struct User {
    username: String,
    email: String,
    active: bool,
}

fn main() {
    let user1 = User {
        username: String::from("hong"),
        email: String::from("홍길동"),
        active: true,
    };
    
    println("유저 이름: {}", user.username);
    
```
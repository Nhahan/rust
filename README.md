# Rust

## Installation
```bash
# https://rustup.rs/
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# After run curl
source "$HOME/.cargo/env"
```

## Uninstall
```bash
rustup self uninstall
```

## Single File Compilation
```bash
rustc filename.rs
```

## Cargo (Package Manager & Build System)
```bash
cargo build
cargo run
```

- `cargo run` result example

<img width="471" alt="image" src="https://github.com/Nhahan/rust/assets/81916648/dd6b7be6-ea47-4d8c-a398-3ea3c47dadf7">

## Variables
```rust
fn main() {
    let x = 3;
    println!("x의 값은 {x}입니다."); // 3
    // x = 7; // let 은 불변
    
    let mut y = 1; // mutable 변수
    println!("y의 값은 {y}입니다."); // 1
    y = 7;
    println!("y의 값은 {y}입니다."); // 7
    
    // let -> 타입추론 o, 런타임 초기화
    // const -> 타입추론 x, 컴파일타임 초기화, 상수이므로 mut 불가
    const PI: f32 = 3.141592;
    println!("PI 상수값은 {PI}입니다.")
}
```

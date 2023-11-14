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

#### shadowing
```rust
let x = 3;
println!("x의 값은 {x}입니다."); // 3
let x = x + 1;
println!("x의 값은 {x}입니다."); // 4
{
    let x = x * 2;
    println!("x의 값은 {x}입니다."); // 8 
}
println!("x의 값은 {x}입니다."); // 4
```

## Data Type

- Rust에서 모든 값은 특정 데이터 타입의 값 (기본값Scalar 또는 복합값 Compound)
- Rust는 정적 타입 언어 (컴파일 시점에 모든 변수의 타입을 알아야 함)

#### 정수 integer 타입 (대부분은 i32, u32일 것이다)

|       | 부호 있는 | 부호 없는 |
|-------|-------|-------|
| 8비트   | i8    | u8    |
| 16비트  | i16   | u16   |
| 32비트  | i32   | u32   |
| 64비트  | i64   | u64   |
| 128비트 | i128  | u128  |
| 아키텍쳐  | isize | usize |
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

### 정수 integer 타입 (대부분은 i32, u32일 것이다)

|       | 부호 있는 | 부호 없는 |
|-------|-------|-------|
| 8비트   | i8    | u8    |
| 16비트  | i16   | u16   |
| 32비트  | i32   | u32   |
| 64비트  | i64   | u64   |
| 128비트 | i128  | u128  |
| 아키텍쳐  | isize | usize |

### 정수 리터럴 표현

|         | 예          |
|---------|------------|
| 10진수    | 19_384     |
| 16진수    | 0xff       |
| 8진수     | 0o77       |
| 2진수     | 0b1111_0010|
| 바이트 u8  | b'A'       |

### 부동소수점 floating-point 타입

| | 부호 있는 |
|---|---|
| 32비트 | f32 |
| 64비트 | f64 |

```rust
// 부동소수점 floating-point 타입 추가 설명
// 타입 지정을 해주지 않는다면 f64가 default 타입
let x = 3.14; // f64
let y: f32 = 3.14; // f32
```

### 불린 boolean 타입

```rust
let t = true;
let f: bool = false;
```

### char 타입

```rust
let c = 'A';
let z: char = '가';
```

## Compound 타입

### 튜플 tuple 타입

```rust
fn main() {
    let t: (i32, bool, f64) = (32, true, 3.14);

    let (x, y, z) = t; // 튜플 구조분해 tuple destructuring
    
    println!("x, y, z = {x}, {y}, {z}"); // x=32, y=true, z=3.14
}
```

#### 특별한 튜플 unit

| 타입 | 값(유일) |
|---|---|
| unit | () |

- 빈 튜플. 다른 언어의 void와 비슷하다.

### 배열 array 타입

```rust
fn main() {
    let x = [1, 2, 3, 4, 5]; // let x: [i32: 5] = [1, 2, 3, 4, 5];

    let threes = [3; 100];
    let last = threes[99]; // 3
}
```

- Rust의 배열은 길이가 고정이다. 크기가 가변적인 배열을 원한다면 벡터를 사용해야 한다.

## 함수 Functions

```rust
fn main() {
    println!("Hello, world!");
    another_function();
}

fn another_function() {
    println!("Another function.");
}
```

## if

```rust
fn main() {
    let number = 3;

    if number % 2 == 0 {
        println!("짝수입니다.");
    } else {
        println!("홀수입니다.");
    }
}
```

### let에 쓰는 if

```rust
fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {}", number);
}
```

## loop

```rust
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);
}
```

## while

```rust
fn main() {
    let mut number = 0;

    while number != 10 {
        println!("number = {}", number);
        number += 1;
    }
}
```

## for

```rust
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {}", element);
    }
}
```

---

# [소유권과 임대](OWNERSHIP.md)

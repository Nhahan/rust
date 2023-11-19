# Rust의 메모리 관리 규칙

## 변수의 범위 scope가 끝나면 메모리 해제 가능

- 블록 안에서삼 변수가 유효, 블록을 벗어나면 무효.

```rust
fn main() {
    {
        let s = "hello";
    }
}
```

## 문자열 리터럴과 String

- 기본 데이터 타입은 Stack에서 쉽게 관리 가능
- 문자열 리터럴 string literal은 프로그램에 고정으로 확보(불변)
- 사용자가 입력하는 문자열은 고정으로 확보 불가능(미리 크기를 알 수 없다)

## 문자열 String 타입

- 변경 가능 mutable - 길이가 변함
- 즉, 컴파일 시점에 미리 크기를 알 수 없음
- 그러므로 힙 heap 메모리에 저장

## 문자열 리터럴 값으로부터 String 값 만들기

```rust
fn main() {
    let s = String::from("hello"); // String::from 함수 사용
    println!("{}, world!", s);
}
```

### 메모리의 소유권

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1; // s1의 메모리 소유권이 s2로 이동
    println!("{s2}, world!");
    // println!("{s1}, world!"); // s1의 메모리 소유권은 박탈되었으므로 컴파일 에러
}
```

### 새로운 메모리 할당

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone(); // s1의 메모리를 복제하여 s2에 할당
    println!("{s1}, world!");
    println!("{s2}, world!");
}
```

## 함수 호출시 소유권 이동

```rust
fn main() {
    let s = String::from("hello");
    string_length(s);
    // println!("{s}, world!"); // 소유권이 string_length() 함수로 넘어갔으므로 컴파일 에러
}

fn string_length(s: String) {
    println!("The length of '{}' is {}.", s, s.len());
}
```

## 함수 반환값의 소유권 이동

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = string_length(s1);
    println!("s2 = {}", s2);
}

fn string_length(s: String) -> String {
    println!("The length of '{}' is {}.", s, s.len());
    s // 반환값의 소유권을 호출한 함수로 이동
}
```

## 소유권 임대

```rust
fn main() {
    let s = String:: from("헬로");
    
    let len = calc_length(&s);
    
    println!("{}의 길이는 {}입니다.", s, len);
}

fn calc_length(s: &String) -> usize {
    let length = s.len();
    
    length
}
```
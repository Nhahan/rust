## Error

두 가지 오류를 구분해서 처리

- 복구 가능 -> `Result<T, E>`
- 복구 불가능 -> `panic!`

```rust
fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => panic!("파일 열기 실패: {:?}", error),
    };
}
```
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

```rust
fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("파일 생성 실패: {:?}", e),
            },
            other_error => panic!("파일 열기 실패: {:?}", other_error),
        },
    };
}
```

## 에러 전파

```rust
use std::io;
use std::fs::File;
use std::io::Read;

fn read_username() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}
```

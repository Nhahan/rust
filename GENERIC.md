## Generic

```rust
fn smallest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut smallest = &list[0];

    for item in list {
        if item < smallest {
            smallest = item;
        }
    }

    smallest
}
```

## Struct with Generic

```rust
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let p1: Point<i32> = Point { x: 5, y: 10 };
    let p2: Point<f64> = Point { x: 1.0, y: 4.0 };
    println!("p1 = {:?}, p2 = {:?}", p1, p2);
}

// Method with Generic
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
```
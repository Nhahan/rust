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

## Traits

```rust
trait Greet {
    fn greet(&self) -> String;
}

enum Pet {
    Cat,
    Dog,
    Tiger,
}

impl Greet for Pet {
    fn greet(&self) -> String {
        match self {
            Pet::Cat => String::from("Meow!"),
            Pet::Dog => String::from("Woof!"),
            Pet::Tiger => String::from("Roar!"),
        }
    }
}

struct Person {
    name: String,
}

impl Greet for Person {
    fn greet(&self) -> String {
        format!("Hello, my name is {}", self.name)
    }
}

fn meet(one: &impl Greet, another: &impl Greet) {
    println!("{}", one.greet());
    println!("{}", another.greet());
}

fn main() {
    let cat = Pet::Cat;
    let gildong = Person {
        name: String::from("Gildong"),
    };
    
    meet(&cat, &gildong);
    meet(&gildong, &cat);
}
```

// Topic: Strings

struct Person {
    name: String,
    favorite_color: String,
    age: i32,
}

fn main() {
    let people = vec![
        Person {
            name: "Alice".to_string(),
            favorite_color: "blue".to_string(),
            age: 20,
        },
        Person {
            name: "Bob".to_string(),
            favorite_color: "green".to_string(),
            age: 25,
        },
        Person {
            name: "Carol".to_string(),
            favorite_color: "red".to_string(),
            age: 30,
        },
    ];

    for person in &people {
        println!(
            "{} ({}): {}",
            person.name, person.age, person.favorite_color
        );
    }
}

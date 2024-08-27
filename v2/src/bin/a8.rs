// Topic: Organizing similar data using structs

enum Flavor {
    Chocolate,
    Vanilla,
    Strawberry,
}

struct Drink {
    flavor: Flavor,
    fluid_oz: f64,
}

fn print_drink(drink: Drink) {
    match drink.flavor {
        Flavor::Chocolate => println!("Chocolate"),
        Flavor::Vanilla => println!("Vanilla"),
        Flavor::Strawberry => println!("Strawberry"),
    }
    println!("oz: {:?}", drink.fluid_oz);
}

fn main() {
    let milkshake = Drink {
        flavor: Flavor::Chocolate,
        fluid_oz: 16.0,
    };
    print_drink(milkshake);

    let smoothie = Drink {
        flavor: Flavor::Strawberry,
        fluid_oz: 24.0,
    };
    print_drink(smoothie);
}

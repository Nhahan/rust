// Topic: Result

struct Customer {
    age: i32
}

fn try_purchase(customer: &Customer) -> Result<(), String> {
    if customer.age < 21 {
        return Err("customer must be at least 21 years old".to_string());
    }
    println!("purchase successful");
    Ok(())
}

fn main() {
    let ashley = Customer { age: 22 };
    let purchased = try_purchase(&ashley);
    println!("purchased: {:?}", purchased);
}

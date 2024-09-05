// Topic: Advanced match

enum Ticket {
    Backstage(f64, String),
    Standard(f64),
    Vip(f64, String)
}

fn main() {
    let tickets = vec![
        Ticket::Backstage(50.0, "Alice".to_string()),
        Ticket::Standard(20.0),
        Ticket::Vip(100.0, "Bob".to_string()),
    ];

    for ticket in &tickets {
        match ticket {
            Ticket::Backstage(price, holder) => {
                println!("Backstage ticket for {} is ${}", holder, price);
            },
            Ticket::Standard(price) => {
                println!("Standard ticket is ${}", price);
            },
            Ticket::Vip(price, holder) => {
                println!("VIP ticket for {} is ${}", holder, price);
            },
        }
    }
}

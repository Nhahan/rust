// Topic: User Input

use std::io;

enum PowerState {
    Off,
    Sleep,
    Reboot,
    Shutdown,
    Hibernate
}

impl PowerState {
    fn new(state: &str) -> Option<PowerState> {
        let state = state.trim().to_lowercase();
        match state {
            "off" => Some(PowerState::Off),
            "sleep" => Some(PowerState::Sleep),
            "reboot" => Some(PowerState::Reboot),
            "shutdown" => Some(PowerState::Shutdown),
            "hibernate" => Some(PowerState::Hibernate),
            _ => None,
        }
    }
}

fn print_power_action(state: PowerState) {
    use PowerState::*;
    match state {
        Off => println!("Powering off"),
        Sleep => println!("Putting to sleep"),
        Reboot => println!("Rebooting"),
        Shutdown => println!("Shutting down"),
        Hibernate => println!("Hibernating"),
    }
}

fn main() {
}

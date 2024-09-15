// Topic: Result & the question mark operator

enum Position {
    Maintenance,
    Marketing,
    Manager,
    LineSupervisor,
    KitchenStaff,
    AssemblyTech,
}

enum Status {
    Active,
    Terminated,
}

struct Employee {
    position: Position,
    status: Status,
}

fn try_access(employee: &Employee) -> Result<(), String> {
    match employee.status {
        Status::Terminated => return Err("teminated".to_string()),
        _ => (),
    }

    match employee.position {
        Position::Maintenance => Ok(()),
        Position::Marketing => Ok(()),
        Position::Manager => Ok(()),
        _ => return Err("access denied".to_string()),
    }
}

fn print_access(employee: &Employee) -> Result<(), String> {
    try_access(employee)?;
    println!("access granted");
    Ok(())
}

fn main() {
    let manager = Employee {
        position: Position::LineSupervisor,
        status: Status::Active,
    };

    match print_access(&manager) {
        Err(err) => println!("error: {}", err),
        _ => (),
    }
}

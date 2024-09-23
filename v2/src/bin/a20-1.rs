use std::io;

fn get_input() -> io::Result<String> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer.trim().to_string())
}

fn main() {
    let mut all_input = vec![];
    let mut times_input = 0;
    while times_input < 3 {
        match get_inut() {
            Ok(input) => {
                all_input.push(input);
                times_input += 1;
            },
            Err(e) => {
                println!("error: {}", e);
                break;
            }
        }
    }
}

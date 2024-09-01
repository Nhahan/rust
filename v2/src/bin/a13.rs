// Topic: Vectors

fn main() {
    let my_numbers = vec![10, 20, 30, 40];

    for num in &my_numbers {
        match num {
            10 => println!("ten"),
            20 => println!("twenty"),
            30 => println!("thirty"),
            40 => println!("forty"),
            _ => println!("something"),
        }
    }

    println!("number of elements = {:?}", my_numbers.len());
}

// Not work: for문에서 my_numbers의 ownership을 가져가기 때문에 my_numbers를 사용할 수 없다.
// fn main() {
//     let my_numbers = vec![10, 20, 30, 40];
//
//     for num in my_numbers {
//         match num {
//             10 => println!("ten"),
//             20 => println!("twenty"),
//             30 => println!("thirty"),
//             40 => println!("forty"),
//             _ => println!("something"),
//         }
//     }
//
//     println!("number of elements = {:?}", my_numbers.len());
// }

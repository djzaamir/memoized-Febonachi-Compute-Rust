use std::collections::HashMap;
use std::io;

//Normal Febonachi function, with no memoization / no optimization
fn feb(feb_query : i32) -> i32{
    
    if feb_query <= 2{
       return 1;
    }

    return feb(feb_query -1) + feb(feb_query - 2);
}

fn main() {
    
    println!("Compute Nth Febonachi : ");

    let mut str_content = String::new();

    io::stdin()
        .read_line(&mut str_content)
        .expect("Failed to read user input");

    let str_content_trimmed = str_content.trim();

    let feb_query : i32 = str_content_trimmed.parse::<i32>().unwrap();

    let nth_feb : i32 = feb(feb_query);

}

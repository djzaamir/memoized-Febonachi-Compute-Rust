use std::collections::HashMap;
use std::io;

fn feb(feb_query : i64, use_memoization : bool) -> i64{
    if use_memoization{

        let mut memo : HashMap<i64,i64> =  HashMap::new();
        
        return feb_memoized(feb_query, &mut memo);

    }else{
        return feb_naive(feb_query);
    }
}

//Normal Febonachi function, with no memoization / no optimization
fn feb_naive(feb_query : i64) -> i64{
    
    if feb_query <= 2{
       return 1;
    }

    return feb_naive(feb_query -1) + feb_naive(feb_query - 2);
}


//Optimized function, which uses memoization to massively speed-up repetitive computations
fn feb_memoized(feb_query: i64,  memo : &mut HashMap<i64, i64>) -> i64{

    if memo.contains_key(&feb_query) {
        return memo.get(&feb_query).unwrap();
    }
    if feb_query <= 2{
        return 1;
    }

    let intermediate_result = feb_memoized(feb_query-1, &mut memo) + feb_memoized(feb_query-2, &mut memo);
    memo.insert(feb_query, intermediate_result);
    return intermediate_result;
}

fn main() {
    
    println!("Compute Nth Febonachi : ");

    let mut str_content = String::new();

    io::stdin()
        .read_line(&mut str_content)
        .expect("Failed to read user input");

    let str_content_trimmed = str_content.trim();

    let feb_query : i64 = str_content_trimmed.parse::<i64>().unwrap();

    let nth_feb : i64 = feb(feb_query, false);

    println!("{}th Febonachin Number is -> {}", feb_query, nth_feb);

}

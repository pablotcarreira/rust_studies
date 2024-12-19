pub mod postcode_validator;
use std::time::{Instant};



fn main() {
    call_validate_postcodes()
}


fn call_validate_postcodes(){
    let raw_code = "GIR 0AA";

    // Start the timer
    let start = Instant::now();

    // Pre allocated space to hold the result.
    let mut result = String::with_capacity(7);

    let executions = 10_000_000;
    for _i in 0..executions {
        let r = postcode_validator::validate_post_code(
            raw_code,
            false,
            &mut result
        );
        match r {
            Some(_) =>  continue,   //println!("OK: {}", result_post_code),
            None => println!("Invalid code: {}", result)
        }
    }

    // Stop the timer
    let duration = start.elapsed();
    let executions_per_second = executions as f64 / duration.as_secs_f64();
    println!("Executed {} iterations in {:?} seconds.", executions, duration);
    println!("Executions per second: {:.2}", executions_per_second);


    // Non strict mode:
    // Python 1.8 million /s
    // Rust release (Progressive improvements)
    //    - regex inside function:  ~ 700 /s
    //    - regex passed to function:  5,956,463 /s
    //    - Cow<str>.to_owned: 6,412,509 /s
    //    - Pre-allocated result: 5,970,170 /s
    //    - No regex, Pre-allocated: 11,190,096 /s
}
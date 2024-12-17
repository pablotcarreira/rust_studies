// pub mod brincando_com_types;
pub mod postcode_validator;
use std::time::{Instant, Duration};
use regex::Regex;

fn main() {
    // playing_with_numbers();
    // playing_with_chars_and_tuples();
    // lets_test_references();
    // brincando_com_types::brincando_com_strings();

    let raw_code = "GIR 0AA";

    // Start the timer
    let start = Instant::now();

    // Only initialized once.
    let clean_post_code: Regex = Regex::new(r"[^A-Za-z0-9]+").unwrap();
    let validate_post_code_regex: Regex = Regex::new(
        r#"^(([A-Z]{1,2}[0-9][A-Z0-9]?|ASCN|STHL|TDCU|BBND|[BFS]IQQ|PCRN|TKCA) ?[0-9][A-Z]{2}|BFPO ?[0-9]{1,4}|(KY[0-9]|MSR|VG|AI)[ -]?[0-9]{4}|[A-Z]{2} ?[0-9]{2}|GE ?CX|GIR ?0A{2}|SAN ?TA1)$"#,
    ).unwrap();
    // Pre allocated space to hold the result.
    let mut result = String::with_capacity(6);


    let executions = 1_000_000;
    for _i in 0..executions {
        let r = postcode_validator::validate_post_code(
            raw_code,
            false,
            &clean_post_code,
            &validate_post_code_regex,
            &mut result
        );
        match r {
            Some(_) =>  continue,   //println!("OK: {}", result_post_code),
            None => println!("Invalid code: {}", result)
        }
    }

    // Stop the timer
    let duration = start.elapsed();
    // Calculate executions per second
    let executions_per_second = executions as f64 / duration.as_secs_f64();
    // Print the results
    println!(
        "Executed {} iterations in {:?} seconds.",
        executions, duration
    );
    println!("Executions per second: {:.2}", executions_per_second);

    // Non strict mode:
    // Python 1.8 million /s
    // Rust release (Progressive improvements)
    //    - regex inside function:  ~ 700 /s
    //    - regex passed to function:  5,956,463 /s
    //    - Cow<str>.to_owned: 6,412,509 /s
    //    - Prealocated result: 5,970,170 /s



}



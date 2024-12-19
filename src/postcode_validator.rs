use std::collections::HashSet;
use regex::Regex;
use lazy_static::lazy_static;
use std::time::Instant;

lazy_static! {
    static ref VALID_CHARS: HashSet<char> = ('A'..='Z')
        .chain('a'..='z')
        .chain('0'..='9')
        .collect();

    static ref VALIDATE_POST_CODE_REGEX: Regex = Regex::new(r#"^(([A-Z]{1,2}[0-9][A-Z0-9]?|ASCN|STHL|TDCU|BBND|[BFS]IQQ|PCRN|TKCA) ?[0-9][A-Z]{2}|BFPO ?[0-9]{1,4}|(KY[0-9]|MSR|VG|AI)[ -]?[0-9]{4}|[A-Z]{2} ?[0-9]{2}|GE ?CX|GIR ?0A{2}|SAN ?TA1)$"#).unwrap();
}


/// Clean, format, and validate a post code. Returns the formatted post code if it is valid,
/// otherwise returns `None`.
///
/// If not in strict mode, the function checks if the post code composition is valid.
/// If in strict mode, the post code must exist in the provided cache. A set of well-formatted
/// post codes must be provided as the cache to use strict mode.
///
/// # Parameters
/// - `post_code`: The post code to validate.
/// - `strict_mode`: Whether to use strict mode.
/// - `post_code_cache`: A cache containing acceptable post codes.
///
/// # Returns
/// - `Some(String)` if the post code is valid and formatted.
/// - `None` if the post code is invalid.
pub fn validate_post_code(
    post_code: &str,
    strict_mode: bool,
    result: &mut String
    // post_code_cache: Some(set)

) -> Option<()>
{
    if strict_mode {
        return None
    }

    // Clear the buffer before writing to it
    result.clear();
    // Manually clean the post code by pushing valid characters into the buffer
    for ch in post_code.chars() {
        if VALID_CHARS.contains(&ch){
            result.push(ch);
        }
    }
    if !VALIDATE_POST_CODE_REGEX.is_match(result) {
        return None
    }
    Some(())
}


fn call_validate_postcodes(){
    let raw_code = "GIR 0AA";

    // Start the timer
    let start = Instant::now();

    // Pre allocated space to hold the result.
    let mut result = String::with_capacity(7);

    let executions = 100_000_000;
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
    // +--------------------------+--------------------+
    // | Test Case                | Executions/Second |
    // +--------------------------+--------------------+
    // | Python (my computer)     | 1,800,000         |
    // | Regex inside function    | ~700              |
    // | Regex passed to function | 5,956,463         |
    // | Cow<str>.to_owned        | 6,412,509         |
    // | Pre-allocated result     | 5,970,170         |
    // | No regex, Pre-allocated  | 11,190,096        |
    // | My computer              | 21,051,089        |
    // +--------------------------+--------------------+
}
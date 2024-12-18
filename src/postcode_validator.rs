use std::collections::HashSet;
use regex::Regex;
use lazy_static::lazy_static;

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
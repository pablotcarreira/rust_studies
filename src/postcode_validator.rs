
use regex::Regex;


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
    cleaner_regex: &Regex,
    validator_regex: &Regex,
    result: &mut String
    // post_code_cache: Some(set)

) -> Option<()>
{

    if strict_mode {
        return None
    }

    // Clear the buffer before writing to it
    result.clear();

    // Manually clean the post code by copying valid characters into the buffer
    for ch in post_code.chars() {
        if cleaner_regex.is_match(&ch.to_string()) {
            result.push(ch);
        }
    }

    // // TODO: instead of allocation new memory we could reuse an available pre-allocated space.
    // let cleaned_pc = cleaner_regex.replace_all(post_code, "");

    // The same here.
    if !validator_regex.is_match(result) {
        return None
    }
    // Pre-allocate a buffer for the result (6 characters)
    // result.clear();
    // result.push_str(&cleaned_pc);

    Some(())

}
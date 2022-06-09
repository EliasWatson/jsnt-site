use fancy_regex::{Captures, Regex};

pub fn split_by_regex<F, G, T>(
    text: &str,
    pattern: &Regex,
    match_func: F,
    no_match_func: G,
) -> Vec<T>
where
    F: Fn(&Captures) -> T,
    G: Fn(&str) -> T,
{
    let mut segments = vec![];

    let mut prev_end_index = 0;
    for cap_res in pattern.captures_iter(text) {
        let cap = cap_res.unwrap();
        let entire_match = cap.get(0).unwrap();

        if entire_match.start() > prev_end_index {
            segments.push(no_match_func(&text[prev_end_index..entire_match.start()]));
        }

        prev_end_index = entire_match.end();

        segments.push(match_func(&cap));
    }

    // This is `s.len()` not `s.len() - 1` because end_index is always one past the last variable
    if prev_end_index < text.len() {
        segments.push(no_match_func(&text[prev_end_index..]));
    }

    return segments;
}

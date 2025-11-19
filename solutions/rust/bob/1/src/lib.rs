pub fn reply(message: &str) -> &str {
    let msg = message.trim();
    if msg.is_empty() {
        return "Fine. Be that way!";
    }
    let is_question = msg.ends_with('?');
    let has_letters = msg.chars().any(|c| c.is_alphabetic());
    let is_yelling = has_letters && msg.chars().all(|c| !c.is_lowercase());
    if is_yelling && is_question {
        "Calm down, I know what I'm doing!"
    } else if is_yelling {
        "Whoa, chill out!"
    } else if is_question {
        "Sure."
    } else {
        "Whatever."
    }
}
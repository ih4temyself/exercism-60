pub fn build_proverb(list: &[&str]) -> String {
    if list.is_empty() {
        return String::new();
    }
    let mut proverb = String::new();
    for i in 0..list.len().saturating_sub(1) {
        let this = list[i];
        let next = list[i + 1];

        if !proverb.is_empty() {
            proverb.push('\n');
        }

        proverb.push_str(&format!("For want of a {this} the {next} was lost."));
    }

    if !proverb.is_empty() {
        proverb.push('\n');
    }

    let first = list[0];
    proverb.push_str(&format!("And all for the want of a {first}."));

    proverb
}
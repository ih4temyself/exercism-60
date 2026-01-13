pub fn brackets_are_balanced(string: &str) -> bool {
    let mut st : Vec<char> = Vec::new();

    for ch in string.chars() {
        match ch {
            '(' | '[' | '{' => {
                st.push(ch);
            }
            ')' | ']' | '}' => {
                let open = match st.pop() {
                    Some(x) => x,
                    None => return false
                };
                let ok = match (open , ch) {
                    ('(',')') => true,
                    ('[',']') => true,
                    ('{','}') => true,
                    _ => false
                };
                if !ok { return false }
            }
            _ => { }
        }
    }
    st.is_empty()
}
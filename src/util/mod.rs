pub fn remove_spaces(stringy: &String) -> String {
    let trim_string = stringy.trim();
    let mut new_string = String::new();
    let iterator = trim_string.split_whitespace();

    for i in iterator {
        new_string.push_str(i);
    }

    new_string
}

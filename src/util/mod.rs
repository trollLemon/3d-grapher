/*This is used once the user wants to graph, and has entered a function made of x y and z parametrics.
  The function will look through the whole string and separate the string into thrids bases on the "," in the original string.
  Any spaces are also removed.
*/




pub fn remove_spaces(stringy: &String) -> String {
    let trim_string = stringy.trim();
    let mut new_string = String::new();
    let iterator = trim_string.split_whitespace();

    for i in iterator {
        new_string.push_str(i);
    }

    new_string
}

#[cfg(test)]
mod tests {
    use super::*;

    //helper function tester

    #[test]
    fn remove_spaces_test() {
        let the_string = " hue  huehueh ehu".to_string();
        let expected_string = "huehuehuehehu".to_string();

        let new_string = remove_spaces(&the_string);

        assert_eq!(new_string, expected_string);
    }

    //parser
    #[test]
    fn parse_stuff() {
        let sample_input = "x(t)=cos(t),y(t)=4,z(t)=sqrt(t)".to_string();

        let test_array: [String; 3] = parse_input(&sample_input);

        assert_eq!(test_array[0], "x(t)=cos(t)".to_string());
        assert_eq!(test_array[1], "y(t)=4".to_string());
        assert_eq!(test_array[2], "z(t)=sqrt(t)".to_string());
    }

    #[test]
    fn parse_stuff_again_but_with_spaces() {
        let sample_input_but_with_spaces = "x(t)= cos (t), y(t) =4, z(t)= sqrt(t)".to_string();

        let test_array: [String; 3] = parse_input(&sample_input_but_with_spaces);

        assert_eq!(test_array[0], "x(t)=cos(t)".to_string());
        assert_eq!(test_array[1], "y(t)=4".to_string());
        assert_eq!(test_array[2], "z(t)=sqrt(t)".to_string());
    }
}

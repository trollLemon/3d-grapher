/*This is used once the user wants to graph, and has entered a function made of x y and z parametrics.
  The function will look through the whole string and separate the string into thrids bases on the "," in the original string.
  Any spaces are also removed.
*/

pub fn parse_input(input: &String) -> [String; 3] {
    let mut indexes_of_commas: Vec<usize> = vec![];

    //iterate over the String and find where the ',' are
    let bytes = input.as_bytes();
    let target = b',';

    find_char(bytes, target, &mut indexes_of_commas);

    //now split the String up into thirds now that we know where to slice
    let x = input[..indexes_of_commas[0]].to_string();
    let y = input[indexes_of_commas[0] + 1..indexes_of_commas[1]].to_string();
    let z = input[indexes_of_commas[1] + 1..].to_string();

    //before returning the Array, remove any spaces in the three Strings
    let target = b' ';

    let x_as_bytes = x.as_bytes();
    let y_as_bytes = y.as_bytes();
    let z_as_bytes = z.as_bytes();

    //vectors of all spaces for each string

    let mut x_spaces: Vec<usize> = vec![];
    let mut y_spaces: Vec<usize> = vec![];
    let mut z_spaces: Vec<usize> = vec![];

    //since we have three new strings, we can interate through each of them seperatly instead of the whole thing like last time
    find_char(x_as_bytes, target, &mut x_spaces);
    find_char(y_as_bytes, target, &mut y_spaces);
    find_char(z_as_bytes, target, &mut z_spaces);

    //now that we know where the spaces are, we can remove them and return the strings without spaces
    let fin_x = remove_spaces(x);
    let fin_y = remove_spaces(y);
    let fin_z = remove_spaces(z);

    //now we can return the array
    [fin_x, fin_y, fin_z]
}

/*helper functions*/

/*
  Finds the given char in a string(as bytes), and adds its index into a vector
*/
fn find_char(bytes: &[u8], target: u8, list: &mut Vec<usize>) {
    for (i, &item) in bytes.iter().enumerate() {
        if item == target {
            list.push(i);
        }
    }
}

fn remove_spaces(stringy: String) -> String {
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

        let new_string = remove_spaces(the_string);

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

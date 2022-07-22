use std::iter::zip;

/* This function removes all spaces, and then adds a space between each term.
 * We do this because the parser that will be called later splits the input
 * based on spaces.
 *
 * However, if a string that has no operators is passed in, like "4", then
 * nothing should change and the function should return that string.
 */

pub fn remove_then_add_spaces(stringy: &String) -> String {
    if !contains_ops(stringy) {
        return stringy.to_string();
    }

    let trim_string = stringy.trim();
    let mut new_string = String::new();
    let iterator = trim_string.split_whitespace();

    for i in iterator {
        new_string.push_str(i);
    }

    let mut output = String::new();

    /*  We use two iterators here. One is just the chars of new_string, and one is
     *  the chars of new_string execpt its once step farther in the iterator than the first one:
     *
     *  a,b,c,d
     *  b,c,d
     *
     *  This is so we can compare what would come next in the first iterator, without having to
     *  worry about borrowing issues.
     *
     *  Now we zip the two iterators together and go though each entry
     *  so we can put spaces in between single digits and operators, and since
     *  we are checking the next item in the iterator, we can make sure we dont put
     *  spaces in between stuff like Sin or numbers like 33
     */

    let iter_main = new_string.chars();

    let mut iter_next = new_string.chars();
    iter_next.next(); // shift the second iterator over so its one ahead of iter_main
    
    let mut include_space_switch : bool = false;

    for a in iter_main {

        if a =='('{
            include_space_switch = true;
        }


        if a==')'{

            include_space_switch = false;
        }

        if let Some(b) = iter_next.next() {

            if include_space_switch || is_part_of_function(&a) && is_part_of_function(&b) || (b.is_numeric() && a.is_numeric()){
                output.push(a);
            } else {
                output.push(a);
                output.push(' ');

            }
        } else {
            output.push(a);
        }

        dbg!(&a);

        dbg!(&output);
    }

    output.trim().to_string() //get rid of extra whitespace on the end and then return
}

fn contains_ops(string: &String) -> bool {
    string.contains("+") || string.contains("-") || string.contains("*") || string.contains("/")
}

fn is_part_of_function(c: &char) -> bool {
    contains_trig_part(c) || c == &'(' || c == &')'
}

fn contains_trig_part(c: &char) -> bool {
    match c {
        'S' => true,
        'i' => true,
        'n' => true,
        'C' => true,
        'o' => true,
        's' => true,
        'T' => true,
        'a' => true,
        _ => false,
    }
}

mod tests {

    use crate::util::remove_then_add_spaces;

    #[test]
    fn test_single() {
        let input = "4".to_string();

        let test = remove_then_add_spaces(&input);

        let expected = "4".to_string();

        assert_eq!(test, expected);
    }

    #[test]
    fn test() {
        let input = "4+3+2+1".to_string();
        let test = remove_then_add_spaces(&input);

        let expected = "4 + 3 + 2 + 1".to_string();

        assert_eq!(test, expected);

        let input = "   4+t +5 +4".to_string();
        let test = remove_then_add_spaces(&input);
        let expected = "4 + t + 5 + 4".to_string();

        assert_eq!(test, expected);

        let input = " Sin(t+3) + e + 33   + t".to_string();

        let expected = "Sin(t+3) + e + 33 + t".to_string();

        let test = remove_then_add_spaces(&input);

        assert_eq!(test, expected);
    }
}

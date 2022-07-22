mod parsing_tools {
    
    use crate::calculator::equation_handeler::expr;
    use std::collections::HashMap; 
    use crate::util;

    /*  This is used to perform substitutions for strings with trig functions in them.
     *  Substitutions are used so we can represent a trig function with a char (like u), and
     *  easily move it around or convert it to a new format without having to worry about string
     *  length. This struct also stores the converted version of what ever trig function its
     *  substituting. For example, Sin(t)+4*4*8 will end up looking like; u+4*4*8, and we can then easily
     *  replace the u with S{t}. 
     */
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Substitution {
        var: char,
        equation: String,
        replace: String,
    }
    
    /*  rewrites a given expression into a new form
     *  exampele : Sin(t) will be converted into S{t}
     *  This is so we can distinguish trig functions from 
     *  everything else, and make it easier to parse
     */ 
    #[macro_export]
    macro_rules! rewrite {
        ( $op: expr, $eq: expr ) => {{
            let mut string: String = String::from("");
            let length = $eq.len();

            let front = 4;
            string.push_str($op);
            string.push_str("{");
            string.push_str(&$eq[front..length - 1]);
            string.push_str("}");
            

            dbg!(&$eq);

            string
        }};
    }

    //implementing a sub function so we can use the substitution struct to help with trig
    impl Substitution {
        pub fn sub(&mut self, v: char, eq: String) {
            self.var = v;
            self.equation = eq;
            self.replace = match self.var {
                'S' => {
                    rewrite!("S", self.equation.as_str())
                }
                'C' => {
                    rewrite!("C", self.equation.as_str())
                }
                'T' => {
                    rewrite!("T", self.equation.as_str())
                }
                _ => {
                    panic!("could not find required substitution variable");
                }
            }


                }

    
            pub fn init() -> Self{
            
                Self{
                var: 'i',
                equation: "NAN".to_string(),
                replace: "NAN".to_string(),

                }


            }
    }

        
    
    /* This function take in the user's inputed equation, and
     * converts it to an S expression:
     *  3*4+Sin(t) will be converted into (+ (* 3 4) Sin(t))
     */
    pub fn convert_input(string: &String) -> String {
       
        let mut eq_parts: HashMap<i32, &str>= HashMap::new();//this will hold our terms 
        
        let eq_iter =  string.split_whitespace();
       
        let mut counter = 0; //we will use this counter to represent parts of the function
        
        /* put all of the terms into eq_parts, with the counter being the key
         * for example in 3+4+5/Sin(t), the hashmap will look like this:
         * 0 -> 3
         * 1 -> 4
         * 2 -> 5
         * 3 -> Sin(t)
         */
        for term in eq_iter {
            if is_term(term) {
            eq_parts.insert(counter, term);
            
            counter +=1;
            }
        }

        /* before we put the string into the pratt parser
         * we need to replace all the terms in the string with 
         * their keys from the hashmap. This is because
         * the pratt parser we are using only supports single digit 
         * numbers.
         * This is what should happen:
         *
         * 3+4+Cos(t*3) will turn into 0+1+2, where 0 is the 
         * key for 3, 1 is the key for 4, and 2 is the key for Cos(t*3)
         *
         * our subsitution string willl be put into the pratt parser,
         * and we will get:
         * (+ (+ 1 2) 0)
         *
         * and after that we can replace those numbers with our actuall terms
         * (+ (+ 3 4) Cos(t*3))
         */
        
        let mut new_counter: i64 = 0;

        let mut  substitution_string = String::new();

        let iter = string.split_whitespace(); //we will iterate through each item in the string
                                              //with this

        /*  Go through each item in the string, and determine if it is a term or an operator.
         *  If the item is a term (like 3, or Sin(40000000)), we will add the value of new_counter.
         *  If the item is an operator (like + or -), we will add the item as it is.
         *  
         *
         */
        for i in iter {
            if is_term(i){
            substitution_string.push_str(new_counter.to_string().as_str());
            new_counter+=1;
            } else {
            substitution_string.push_str(i);
            } 
        }

       

        let pratt_string = expr(substitution_string.as_str());//lets get our new string into
                                                              //S-expression form 


        //now we can replace the numbers from the previous loop with our actual terms and numbers,
        //and then we are done and have the inputed string in the form we need
       

        let mut final_string = String::new();

        for i in pratt_string.to_string().chars() {
            
            let cur_part = String::from(i);


            //see if the current char is an operator or not
            if is_term(cur_part.as_str()) {
                
                //TODO: determine what to tell the user if this panics
                
                let key = cur_part.parse::<i32>().unwrap();
                          
                assert!(eq_parts.contains_key(&key), "");

                final_string.push_str( eq_parts.get(&key).unwrap());
            } else {
        
                final_string.push_str(cur_part.as_str());

            }

        }
        final_string

    }
    
    pub fn is_term(string : &str)-> bool{
        string != "+" && string != "-" && string != "*" && string != "/" && string != "(" && string != ")" && string != " "   
    }
    

    /*  This function is for converting any trig functions present into a new format.
     *  Im not sure what to call the format, but it looks like this
     *
     *  f{expr}
     *
     *  where f is a capital letter denoting which trig function to use (i.e S for Sin),
     *  and expr is the expression (i.e t + 4)
     *  S{t+4}
     *
     */
    pub fn trig_convert(string: &String) -> String {
        
       if !string.contains('S') && !string.contains('C') && !string.contains('T') {

            return string.to_string()
       } else  {

        //if the program reaches here, then we need to find where the trig functions are located

           let sin_location = string.chars().position(|c| c == 'S');
           let cos_location = string.chars().position(|c| c == 'C');
           let tan_location = string.chars().position(|c| c == 'T');
            
           
           let mut result : String = String::new();
            
           match sin_location {
            
               Some(s) =>{
                
               result = sin_rewrite(&string, s); 

               },
               None =>{}

            }
            
              match cos_location {
            
               Some(c) =>{
                
               result = cos_rewrite(&string, c); 

               },
               None =>{}

            }
            
              match tan_location {
            
               Some(t) =>{
                
               result = tan_rewrite(&string, t); 

               },
               None =>{}

            }



        result
       } 




    }


    fn sin_rewrite(eq: &String, index: usize)-> String {
    
        let end_of_trig = look_for_end(eq);
        let string_slice = eq[index..end_of_trig+1].to_string();

        let mut sin_sub = Substitution::init();

        sin_sub.sub('S',string_slice);

        let new_string = eq.replace(sin_sub.equation.as_str(), sin_sub.replace.as_str());
        

        new_string
    }
    
   
    fn cos_rewrite(eq: &String, index: usize)-> String {
    
        let end_of_trig = look_for_end(eq);
        let string_slice = eq[index..end_of_trig+1].to_string();

        let mut cos_sub = Substitution::init();

        cos_sub.sub('C',string_slice);

        let new_string = eq.replace(cos_sub.equation.as_str(), cos_sub.replace.as_str());
        

        new_string
    }
    
        fn tan_rewrite(eq: &String, index: usize)-> String {
    
        let end_of_trig = look_for_end(eq);
        let string_slice = eq[index..end_of_trig+1].to_string();

        let mut tan_sub = Substitution::init();

        tan_sub.sub('T',string_slice);

        let new_string = eq.replace(tan_sub.equation.as_str(), tan_sub.replace.as_str());
        

        new_string
    }

    pub fn look_for_end(string: &String) -> usize {
        //we can assume that there will always be a pair of parenthesis, so 
        //we dont need to worry about this panicing 
        string.chars().position(|c| c == ')').unwrap() 
    

    }

    //determines if the current string slice is the last sub-expression: (+ 1 (* 24 4)) would be false,
    //but (+ 1 2) would be true
    //also, if there are no parentheses, like with C{t+3}, then it will also return true

    pub fn is_last_expr(string: &String) -> bool {
        let mut left_paren = 0;
        let mut right_paren = 0;

        for b in string.as_str().as_bytes() {
            match b {
                b'(' => {
                    left_paren = left_paren + 1;
                }
                b')' => {
                    right_paren = right_paren + 1;
                }
                _ => {}
            }
        }
        if left_paren <= 1 && right_paren <= 1 {
            true
        } else {
            false
        }
    }
    pub fn get_operator(stringy: &String) -> u8 {
        let data = &stringy.as_str()[1..].as_bytes();

        data[0]
    }

    pub fn get_numbers_out_of_string(string: String) -> Vec<String> {
        let length = string.len();

        let the_bytes = string[2..length - 1].split_whitespace(); //remvove the first two characters because they are not numbers: "(' and an Operator (+ _ * /), and trim the outer spaces away,
        let mut numbers: Vec<String> = vec![];
            

        for i in the_bytes {
            let len = &i.len();
            if !&i.contains("(") && !&i.contains(")") {
                numbers.push(i.to_string());
            } else if i.contains("(") || i.contains(")") {
                numbers.push(i[..len - 1].to_string());
            }
        }

        numbers
    }
}

pub mod math_functions {
    use crate::calculator::math::parsing_tools::*;
     

    //replaces the variable with the number we want to calculate at, and runs the calculation
    pub fn calculate(string: &String, t: i64) -> f64 {
            

        //if the inputed string doesnt have any operators in it and does not contain 't', we can
        //just return it

        if dont_math(&string) && !string.contains('t'){
            return string.parse::<f64>().unwrap();
        }
            

        let new = convert_input(&trig_convert(&string));


        let equation = str::replace(new.as_str(), "t", t.to_string().as_str());
        
        parse(equation).parse::<f64>().unwrap()

    }     
    

    /* This function takes in the string that calculate() give it, and evaluates all the 
     * expressions in the string, starting from the center of the string to the outer parts.
     * For example, the function will take (+ (* 3 (* 3 2)) 4), and start with (* 3 2) before
     * moving outwards
     */
    pub fn parse(string: String) -> String {
        
        if is_last_expr(&string) {
            do_some_math(string).to_string()
        } else {
            let terms = get_terms(&string);

            if terms.1 == "(".to_string() {
                let mut new_string: String = "".to_string();

                let first = get_op_and_first_term(&string);
                let second = parse(cut_out_second_term(&string));

                new_string.push_str(first.as_str());
                new_string.push_str(second.as_str());
                new_string.push_str(")");

                do_some_math(new_string).to_string()
            } else if terms.0 == "(".to_string() {
                let mut new_string: String = "".to_string();

                let first = parse(cut_out_first_term(&string));

                let second = cut_out_second_term(&string);

                let op = get_op_and_first_term(&string);
                new_string.push_str(&op[..3]);
                new_string.push_str(first.as_str());
                new_string.push_str(" ");
                new_string.push_str(second.as_str());
                new_string.push_str(")");

                do_some_math(new_string).to_string()
            } else {
                panic!("bad input:{}", string);
            }
        }

    
    }

    pub fn get_terms(string: &String) -> (String, String) {
        let len = string.len() - 1;
        let target: String = string[2..len].to_string(); //we dont need the first ( and the operator since we just need the terms
        let mut terms = target.split_whitespace();

        let first = terms.next().unwrap();

        let second = terms.next().unwrap();

        if first.contains("(") {
            ("(".to_string(), second.to_string())
        } else if second.contains("(") {
            (first.to_string(), "(".to_string())
        } else {
            (first.to_string(), second.to_string())
        }
    }

    pub fn get_op_and_first_term(string: &String) -> String {
        string[..5].to_string()
    }

    pub fn cut_out_second_term(string: &String) -> String {
        if string.chars().nth(3).unwrap() != '(' {
            string[5..string.len() - 1].to_string()
        } else {
            let len = string.len();
            let target = string[3..len-1].to_string();
            
            let mut end_index = target.len();
               
            let mut new_string: String = String::from("");
           
            //look at the string starting from the end
            for c in target.chars().rev() {
                if c.is_whitespace() {
                break;
                } else {
                new_string.push(c);
                end_index= end_index -1;

                }


            }
            
            //the string we want is backwards, so reverse it before returning it
            new_string.chars().rev().collect()
            }

            
        }
    

    pub fn cut_out_first_term(string: &String) -> String {
        if string.chars().nth(3).unwrap() != '(' {
            string[3..4].to_string()
        } else {
            let target = string[2..].as_bytes();

            let mut right_paren = 0;
            let mut left_paren = 1;
            let mut index = 0;

            for b in target {
                index = index + 1;

                match b {
                    b'(' => left_paren = left_paren + 1,

                    b')' => {
                        right_paren = right_paren + 1;
                    }

                    _ => {}
                }

                if left_paren - right_paren == 0 {
                    break;
                }
            }

            string[3..index - 1].to_string()
        }
    }

    enum Tasks {
        Add,
        Sub,
        Mult,
        Div,
    }

    use crate::calculator::math::math_functions::Tasks::{Add, Div, Mult, Sub};
    use crate::util::remove_then_add_spaces;
    
    //determines if a string has an operator in it, which means we need to do math with it (true),
    //or if the string has no operator in it, which means we dont need to do anything with it
    //(false)
   pub fn dont_math(string :&String) -> bool {
        
        if !string.contains('+') && !string.contains('-') && !string.contains('*') && !string.contains('/') && !string.contains('{') && !string.contains('}'){
            true
         } else {
            false
         }
    } 

    //determines if the given input is a single trig function: like Sin(t), or if its not
    fn is_not_single_trig(string : &String) -> bool {
        if string.starts_with('S') || string.starts_with('C') || string.starts_with('T') {
            true

        } else {
        false
        }

    }
    
    
    fn trig_func(string: &String)-> String {
        
        let op = string.as_str().chars().nth(0).unwrap();

        let index_0 = 2;
        let index_1 = string.len()-1;

        let expr = string[index_0..index_1].to_string();
        
        let converted_expr= remove_then_add_spaces(&expr);
    
        
        
        let answer : f64 = calculate(&converted_expr,0);

        let result: String;

        match op {

            'S'=>{

                result = answer.sin().to_string();
            },
            'C'=>{
                 result = answer.cos().to_string();
            },
            'T'=>{
                 result = answer.tan().to_string();

            },
            _=>{

                panic!();
            },

        }
    

        result
    }

    pub fn do_some_math(parsed_string: String) -> f64 {
            
        if dont_math(&parsed_string){
            
            return parsed_string.parse::<f64>().unwrap();
        } 

        if is_not_single_trig(&parsed_string) {

            return trig_func(&parsed_string).parse::<f64>().unwrap();
        }

        let operator_in_string = get_operator(&parsed_string);
        let operator: Tasks = match operator_in_string {
            b'+' => Add,
            b'-' => Sub,
            b'*' => Mult,
            b'/' => Div,
            _ => {
                panic!("Bad operator:{}", parsed_string);
            }
        };
        let mut the_numbers = get_numbers_out_of_string(parsed_string);
        
        //if there are things like Sin functions we need to do for any of the terms, do them now
        dbg!(&the_numbers);
        if the_numbers[0].contains("S")
            || the_numbers[0].contains("C")
            || the_numbers[0].contains("T")
        {
            the_numbers[0]= trig_func(&the_numbers[0]);
        }
        
        let first_term = the_numbers[0].parse::<f64>().unwrap();


        let second_term = the_numbers[1].parse::<f64>().unwrap();
        

        match operator {
            Add => first_term + second_term,
            Sub => first_term - second_term,
            Mult => first_term * second_term,
            Div => first_term / second_term,
        }
    }

    }


#[cfg(test)]
mod tests {
    use crate::calculator::math::math_functions::*;
    use crate::calculator::math::parsing_tools::*;
    use  crate::rewrite;
    
    #[test]
    fn test_trig_convert(){
        let string = String::from("(+ Sin(3+t) t)");
        let expected = String::from("(+ S{3+t} t)");
        let actual = trig_convert(&string);   

        assert_eq!(actual, expected);

    }

    #[test]
    fn test_rewrite() {

        let string = String::from("Sin(4+t)");
        let expected = String::from("S{4+t}");
        let actual = rewrite!("S",string);

        assert_eq!(actual, expected);

    }

    #[test]
    fn test_is_term(){

        let string = "+";
        let bool_test = is_term(string);
        assert_eq!(bool_test, false);
    }

    #[test]
    fn test_converter(){
    
    //single values should just be returned as they are
    let string = String::from("0");
    let expected = String::from("0");
    let acutal = convert_input(&string);
    assert_eq!(acutal,expected);
    
    let string = String::from("Sin(t)");
    let expected = String::from("Sin(t)");
    let acutal = convert_input(&string);
    assert_eq!(acutal,expected);

    
    //now test with operators involved
    
    let string = String::from("10 + 3");
    let expected = String::from("(+ 10 3)");
    let acutal = convert_input(&string);
    assert_eq!(acutal,expected);
    
   let string = String::from("1000 * 10");
   let expected = String::from("(* 1000 10)");
   let acutal = convert_input(&string);
   assert_eq!(acutal,expected);
    
   let string = String::from("Sin(t+3) * 48");
   let expected = String::from("(* Sin(t+3) 48)");
   let acutal = convert_input(&string);
   assert_eq!(acutal,expected);
    }

    #[test]
    fn tests() {
        let parsed = "(- 2 3)";

        let math = do_some_math(parsed.to_string());

        assert_eq!(math, -1.0);

        let parsed = "(+ 2 2)";

        let math = do_some_math(parsed.to_string());

        assert_eq!(math, 4.0);
    }

    #[test]
    fn test_get_operator() {
        let parsed = "(- 1 2)";

        let expected_op = b'-';

        let the_op = get_operator(&parsed.to_string());

        assert_eq!(the_op, expected_op);
    }

    #[test]
    fn test_parse() {
        let parsed = "(- 2 3)".to_string();

        let answer = parse(parsed);

        let expected = "-1".to_string();

        assert_eq!(answer, expected);
    }

    #[test]
    fn test_recursive_parser() {
        let parsed = "(+ 1 (* 2 3))".to_string();

        let answer = parse(parsed);

        let expected = "7".to_string();

        assert_eq!(answer, expected);
        
       

	let parsed = "(* (* 10 10) 10)".to_string();

        let answer = parse(parsed);

        let expected = "1000".to_string();

        assert_eq!(answer, expected);


        
    }

    #[test]
    fn test_recursive_parser_large_input() {
        let parsed = "(+ (+ 1 (* (* 2 5) 3)) 2)".to_string();

        let answer = parse(parsed);

        let expected = "33".to_string();

        assert_eq!(answer, expected);
    }

    #[test]
    fn test_with_variables() {
        let var = 1;

        let parsed = "5 * 2 * 3 + t + 2".to_string();
        let answer = calculate(&parsed, var);

        let expected = 33.0;

        assert_eq!(answer, expected);
    }

    #[test]
    fn test_calculate() {
        let parsed = "10".to_string();
        let var = 0;
        let answer = calculate(&parsed, var);
        let expected = 10.0;
        assert_eq!(answer, expected);

        let parsed = "1 + 2 * 3".to_string();

        let var = 8;

        let expected = 7.0;
        let answer = calculate(&parsed, var);
        assert_eq!(answer, expected);
    }

    #[test]
    fn test_op_and_first_term() {
        let parsed = "(+ 1 (* 2 3))".to_string();

        let expected = "(+ 1 ".to_string();

        let test = get_op_and_first_term(&parsed);

        assert_eq!(test, expected);
    }

    #[test]
    fn test_get_terms() {
        let parsed = "(* 10 10)".to_string();

        let expected = ("10", "10");

        let test = get_terms(&parsed);

        assert_eq!(test.0, expected.0);
        assert_eq!(test.1, expected.1);

        let parsed = "(* 2 (- 2 3))".to_string();

        let expected = ("2", "(");

        let test = get_terms(&parsed);

        assert_eq!(test.0, expected.0);
        assert_eq!(test.1, expected.1)
    }

    #[test]
    fn test_cut_out_first_term() {
        let parsed = "(+ (+ a (* (* b c) d)) e)".to_string();
        let expected = "(+ a (* (* b c) d))".to_string();

        let test = cut_out_first_term(&parsed);

        assert_eq!(test, expected);

        let parsed = "(+ 1 (* 2 3))".to_string();
        let test = cut_out_first_term(&parsed);

        let expected = "1".to_string();
        assert_eq!(test, expected);

        let parsed = "(* (* 2 5) 3)".to_string();
        let test = cut_out_first_term(&parsed);
        let expected = "(* 2 5)".to_string();

        assert_eq!(test, expected);
    }

    #[test]
    fn test_cut_out_second_term() {
        let parsed = "(+ (+ a (* (* b c) d)) e)".to_string();

        let expected = "e".to_string();

        let test = cut_out_second_term(&parsed);

        assert_eq!(test, expected);

        let parsed = "(* (* 10 10) 10)".to_string();

        let expected = "10".to_string();

        let test = cut_out_second_term(&parsed);

        assert_eq!(test, expected);
    }

    #[test]
    fn test_number_fetcher() {
        let parsed = "(- 1 3)";

        let numbers = get_numbers_out_of_string(parsed.to_string());

        assert_eq!(numbers[0], "1");
        assert_eq!(numbers[1], "3");

        let parsed = "(+ 100 10)";

        let numbers = get_numbers_out_of_string(parsed.to_string());

        assert_eq!(numbers[0], "100");
        assert_eq!(numbers[1], "10");
    }


    #[test]
    fn test_find_end_of_trig() {
        let before = "Sin(3*t)+4";
        let expected: usize = 7;
        let test = look_for_end(&before.to_string());
        assert_eq!(test, expected);

        let before = "Tan(357385950*t)+4";
        let expected: usize = 15;
        let test = look_for_end(&before.to_string());
        assert_eq!(test, expected);
    }
        

    #[test]
    fn trig_formater(){
    let input = String::from("Sin(2 * t)");
    let string = String::from("S{2 * t}");

    let test = trig_convert(&input);
    assert_eq!(test, string);
    }

    #[test]
    fn calculate_with_trig_sin() {
        let input = "Sin(3+t) + t".to_string();
        let var = 3;

        let answer = calculate(&input, var);
        
        let expected: f64 = 2.720584501801074;
        assert_eq!(answer, expected);
    }

    #[test]
    fn calculate_with_trig_cos() {
        let input = "Cos(3+t) + t".to_string();
        let var = 3;
        let answer = calculate(&input, var);
        let expected: f64 = 3.960170286650366;
        assert_eq!(answer, expected);
    }

    #[test]
    fn calculate_with_trig_tan() {
        let input = "Tan(3) + t".to_string();
        let var = 3;
        let answer = calculate(&input, var);
        let expected: f64 = 2.857453456925722;
        assert_eq!(answer, expected);
    }
}

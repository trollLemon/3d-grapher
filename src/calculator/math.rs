use crate::calculator::math::tasks::Add;
use crate::calculator::math::tasks::Div;
use crate::calculator::math::tasks::Mult;
use crate::calculator::math::tasks::Sub;
use crate::calculator::equation_handeler::expr;













//we will substitute the trig functions with u, j and k for sin, cos, and tan respectivly, and
//after the pratt parser puts it in S expression form, we will then replace the substitution with
//the functions
pub struct substitution {

    var: char,
    equation: String,
    replace_with: String    
}



impl substitution {
    
  pub  fn sub(&mut self, v: char, eq: String) {
        self.var = v;
        self.equation = eq;
        self.replace_with = match self.var {
        'u'=>{
        
            let mut string : String = String::from("");
            let length = self.equation.len();

            string.push_str("S{");
            string.push_str(&self.equation[4..length-1]);
            string.push_str("}");
            string


        },
        'j' =>{
           let mut string : String = String::from("");
            let length = self.equation.len();

            string.push_str("C{");
            string.push_str(&self.equation[4..length-1]);
            string.push_str("}");
            string


        },
        'k' =>{ 
           let mut string : String = String::from("");
            let length = self.equation.len();

            string.push_str("T{");
            string.push_str(&self.equation[4..length-1]);
            string.push_str("}");
            string

        },
        _ => {
    
            panic!("could not find required substitution variable");
        },
        
        
        }
  } 
}



pub fn convert_to_parsed_input (string: String) -> String{
    

      let mut  sin_sub = substitution {
        var: 'i',
        equation: "init".to_string(),
        replace_with: "NAN".to_string(),

      };
      
      let mut  cos_sub = substitution {
        var: 'i',
        equation: "init".to_string(),
          replace_with: "NAN".to_string(),


      };
      
      let mut  tan_sub = substitution {
        var: 'i',
        equation: "init".to_string(),
        replace_with: "NAN".to_string(),

      };



      let the_bytes = string.as_bytes();
      let mut counter: usize = 0;  
      for b in the_bytes {
        
          let curr_slice = &string[counter..].to_string();
          match b {
            
              b'S' => {
                let end_of_slice = look_for_end(curr_slice); //find the end of the trig function
                sin_sub.sub('u', string[counter..end_of_slice].to_string());                   
              },
              b'C' => {
                let end_of_slice = look_for_end(curr_slice);
                cos_sub.sub('j', string[counter..end_of_slice].to_string());

              },
              b'T' => {
                let end_of_slice = look_for_end(curr_slice);
                tan_sub.sub('k', string[counter..end_of_slice].to_string());

             
              },
              _    =>{}

          }
        counter = counter +1;
      }

    //ok so now we have the positions of the trig function(s), so we can now replace stuff with an
    //arbituary  letter,u for sin, j for tan. we will also make sure we arent replacing stuff that
    //doesnt exist

    let mut new_string = String::from("");
    
    let do_sin : bool = sin_sub.var != 'i';
    let do_cos : bool = cos_sub.var != 'i';
    let do_tan : bool = tan_sub.var != 'i';


    
    if !do_sin && !do_cos && !do_tan {
        
     string
    } else {

    if do_sin {
    new_string.push_str(string.replace(sin_sub.equation.as_str(), sin_sub.var.to_string().as_str()).as_str());
    }
    if do_cos {
    new_string.push_str(string.replace(cos_sub.equation.as_str(), cos_sub.var.to_string().as_str()).as_str());

    }    
    if do_tan {
    new_string.push_str(string.replace(tan_sub.equation.as_str(), tan_sub.var.to_string().as_str()).as_str());

    }    
    

   let ready = expr(new_string.as_str()).to_string();
    let mut output = String::from("");

    //ok now replace the substituted variable with the trig stuff
    if do_sin {

        output.push_str(ready.replace(sin_sub.var.to_string().as_str(), sin_sub.replace_with.as_str()).as_str());
    }
    if do_cos {
        output.push_str(ready.replace(cos_sub.var.to_string().as_str(), cos_sub.replace_with.as_str()).as_str());

    }    
    if do_tan {
        output.push_str(ready.replace(tan_sub.var.to_string().as_str(), tan_sub.replace_with.as_str()).as_str());


    }    
    
    
    	
    output
    
    }
    
}




pub fn look_for_end(string:&String) -> usize {

    let bytes = string.as_bytes();
    let mut counter = 0;

    for b in bytes {
    counter = counter +1;
    if *b == b')'{  
    break;
        }
     

    }
    counter
}



//replaces the variable with the number we want to calculate at, and runs the calculation
pub fn calculate(string: String, t: i64) -> i64 {
   
   let new =  convert_to_parsed_input(string);
    let equation = str::replace(new.as_str(),"t", t.to_string().as_str() );
    parse(equation).parse::<i64>().unwrap()

} 


fn parse(string: String)-> String {
    
    
   
   

            if string.chars().nth(0).unwrap() == '(' && string.chars().nth(6).unwrap() == ')'{
        	
        
        do_some_math(string).to_string()



    } else {
		
	 let terms = get_terms(&string);	
	
	if terms.1 == '(' {
	
	let mut new_string: String ="".to_string();
	
	let first = get_op_and_first_term(&string);
	let second = parse(cut_out_second_term(&string));


        new_string.push_str(first.as_str());
	new_string.push_str(second.as_str());
	new_string.push_str(")");
       
        do_some_math(new_string).to_string()

	} else if terms.0 == '(' {
	 
	
	let mut new_string: String ="".to_string();
	
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
	
	panic!();
	
	}
       
    } 
    }









fn get_terms(string:& String) -> (char, char)  {
	
    let target: String = string[2..].to_string(); //we dont need the first ( and the operator since we just need the terms 
    
    let term_one : usize = 1;
    let term_two : usize = 3;

(target.chars().nth(term_one).unwrap(), target.chars().nth(term_two).unwrap()) //indexes 1 and 3 will have the terms 


}

fn get_op_and_first_term(string: &String) ->String {

	
     string[..5].to_string()
    
    }



fn cut_out_second_term(string: &String)-> String {
    
   

    if string.chars().nth(3).unwrap() != '(' {
       
       string[5..string.len()-1].to_string()

   } else {
    
     let target = string[3..].as_bytes();

     let mut right_paren = 0;
     let mut left_paren = 1;
     let mut index = 0;

     for b in target {
      
         index = index + 1;

         match b {

             b'(' => {
                left_paren = left_paren + 1
             },

             b')' => {
                right_paren = right_paren +1;
             },

             _=>{}

         }

        


         if left_paren - right_paren == 0 {
            break;
         } 
         


     }

     string[index+1..string.len()-1].to_string()

    }

}


fn cut_out_first_term(string: &String) -> String {
    
  
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

             b'(' => {
                left_paren = left_paren + 1
             },

             b')' => {
                right_paren = right_paren +1;
             },

             _=>{}

         }

        


         if left_paren - right_paren == 0 {
            break;
         } 
         


     }

     string[3..index-1].to_string()

    }


    

}





enum tasks {
    Add,
    Sub,
    Mult,
    Div,
}

pub  fn do_some_math(parsed_string: String) -> i64 {


	

            let operator_in_string = get_operator(&parsed_string);
            let operator: tasks = match operator_in_string {
                b'+' => Add,
                b'-' => Sub,
                b'*' => Mult,
                b'/' => Div,
                _ => {
                    panic!();
                }
            };

            let the_numbers = get_numbers_out_of_string(parsed_string);
		

            let first_term = the_numbers[0].parse::<i64>().unwrap();
            let second_term = the_numbers[1].parse::<i64>().unwrap();

            match operator {
                Add => first_term + second_term,
                Sub => first_term - second_term,
                Mult => first_term * second_term,
                Div => first_term / second_term,
            }
        
}


fn get_operator(stringy: &String) -> u8{
    let data = &stringy.as_str()[1..].as_bytes();
    
    data[0]


    



}


fn get_numbers_out_of_string(string: String) -> Vec<String> {
    let length = string.len();

    let the_bytes = string[2..length - 1].split_whitespace(); //remvove the first two characters because they are not numbers: "(' and an Operator (+ _ * /), and trim the outer spaces away,

    let mut numbers: Vec<String> = vec![];

    for i in the_bytes {
        numbers.push(i.to_string());
    }

    numbers
}







#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests() {
        let parsed = "(- 2 3)";

        let math = do_some_math(parsed.to_string());

        assert_eq!(math, -1);

        let parsed = "(+ 2 2)";

        let math = do_some_math(parsed.to_string());

        assert_eq!(math, 4);

    }

    #[test]
    fn test_get_operator(){
    let parsed = "(- 1 2)";
    
    let expected_op = b'-';

    let the_op = get_operator(&parsed.to_string());

    assert_eq!(the_op, expected_op);

    




}

    #[test]
    fn test_parse(){
    
        let parsed = "(- 2 3)".to_string();
        let var = 1;

        let answer = parse(parsed);
    
        let expected = "-1".to_string();

        assert_eq!(answer, expected);
        

    }


    #[test]
    fn test_recursive_parser() {
    
        let var = 1;

        let parsed = "(+ 1 (* 2 3))".to_string();
        
        let answer = parse(parsed);
        
         let expected = "7".to_string();

         assert_eq!(answer,expected);


    }
    
    

    
      #[test]
    fn test_recursive_parser_large_input() {
    
        let var = 1;

        let parsed = "(+ (+ 1 (* (* 2 5) 3)) 2)".to_string();
        
        let answer = parse(parsed);
        
         let expected = "33".to_string();

         assert_eq!(answer,expected);


    }
    
          #[test]
    fn test_with_variables() {
    
        let var = 1;

        let parsed = "(+ (+ t (* (* 2 5) 3)) 2)".to_string();
        
        let answer = calculate(parsed, var);
        
         let expected = 33;

         assert_eq!(answer,expected);
         
         let var = 1;

        let parsed = "(+ (+ t (* (* 2 5) t)) 2)".to_string();
        
        let answer = calculate(parsed, var);
        
         let expected = 13;

         assert_eq!(answer,expected);


    }
    
    


    #[test]
    fn test_calculate(){
    
        let parsed = "(- 2 3)".to_string();
        let var = 1;
        let answer =  calculate(parsed, var);
        let expected = -1;
        assert_eq!(answer, expected );

        let parsed = "(+ 1 (* 2 3))".to_string();

        let var = 8;
        
        let expected = 7;
        let answer = calculate(parsed, var);
        assert_eq!(answer, expected);


    }

    
    #[test]
    fn test_op_and_first_term(){
    
        let parsed = "(+ 1 (* 2 3))".to_string();

        let expected = "(+ 1 ".to_string();

        let test = get_op_and_first_term(&parsed);

        assert_eq!(test, expected);
        
    } 

    
    #[test]
    fn test_get_terms(){
    
        let parsed = "(* 2 3)".to_string();
        
        let expected = ('2', '3');

        let test = get_terms(&parsed);
    
        assert_eq!(test.0, expected.0);
        assert_eq!(test.1, expected.1);
        
        
        let parsed = "(* 2 (- 2 3))".to_string();
        
        let expected = ('2', '(');

        let test = get_terms(&parsed);
    
        assert_eq!(test.0, expected.0);
        assert_eq!(test.1, expected.1)




    }


    #[test]
    fn test_cut_out_first_term(){

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
    fn test_cut_out_second_term(){
    
        let parsed = "(+ (+ a (* (* b c) d)) e)".to_string();

        let expected = "e".to_string();

        let test = cut_out_second_term(&parsed);

        assert_eq!(test, expected);
        
        
         let parsed = "(+ 1 (* 2 3))".to_string();

        let expected = "(* 2 3)".to_string();

        let test = cut_out_second_term(&parsed);

        assert_eq!(test, expected);
        

    }


    #[test]
    fn test_number_fetcher() {
        let parsed = "(- 1 3)";

        let numbers = get_numbers_out_of_string(parsed.to_string());

        assert_eq!(numbers[0], "1");
        assert_eq!(numbers[1], "3");

        let parsed = "(+ 3 x)";

        let numbers = get_numbers_out_of_string(parsed.to_string());   
        
        assert_eq!(numbers[0], "3");
        assert_eq!(numbers[1], "x");


    }



    #[test]
    fn test_init_sin() {
    let before = "Sin(3*t) + 4";
    let after = "(+ S{3*t} 4)";

    let test = convert_to_parsed_input(before.to_string());

    assert_eq!(test, after);
    
   
 

    }
    
    #[test]
    fn test_init_cos(){
     let before = "Cos(t) - 3";
    let after = "(- C{t} 3)";

    let test = convert_to_parsed_input(before.to_string());

    assert_eq!(test, after);
    } 	
    
    #[test]
    fn test_init_tan(){
    
       
    let before = "Tan(t/4) * 8";
    let after = "(* T{t/4} 8)";

    let test = convert_to_parsed_input(before.to_string());

    assert_eq!(test, after);

    
    }  	

    #[test]
    fn test_find_end_of_trig(){
    
    let before = "Sin(3*t) + 4";
    let expected : usize = 8;
    let test = look_for_end(&before.to_string());
    assert_eq!(test, expected);
    
        let before = "Tan(357385950*t) + 4";
    let expected : usize = 16;
    let test = look_for_end(&before.to_string());
    assert_eq!(test, expected);
    
    
    }	


}

mod equation_handeler;
mod math;

use crate::util::remove_then_add_spaces;

pub fn vector_function_calculator(parametric:&String, t: i64) -> f64 {
    math::math_functions::calculate(parametric, t)


}



pub fn get_some_data(parametric: String) ->Vec<f64> {
    
    let mut  vector = vec![];
    
    let init = remove_then_add_spaces(&parametric);

    let lim = 50;

    let mut count = 0;

    while count < lim {
    let val = vector_function_calculator(&init, count);
    
    count +=1;
    vector.push(val);
    
    }

    vector
}


mod equation_handeler;
mod math;

pub fn vector_function_calculator(parametric:&String, t: i64) -> f64 {
    math::math_functions::calculate(parametric, t)


}



pub fn get_some_data(parametric: String) ->Vec<f64> {
    
    let mut  vector = vec![];

    let lim = 25;

    let mut count = 0;

    while count < lim {
    let val = vector_function_calculator(&parametric, count);
    
    println!("{},{}", &val, &count);

    count +=1;
    vector.push(val);
    
    }

    vector
}


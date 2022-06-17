mod equation_handeler;
mod math;

pub async fn vector_function_calculator(parametric: String) -> Vec<[f64; 2]> {
    let parsed_eq = equation_handeler::expr(&parametric);

    let mut values: Vec<[f64; 2]> = Vec::new();

    let mut counter = 0.0;

    while counter <= 1000.0 {
        let output = math::do_some_math(parsed_eq.to_string());

        values.push([counter, output]);

        counter += 1.0;
    }

    values
}

#[cfg(test)]
mod tests {}

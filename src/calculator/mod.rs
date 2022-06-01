mod equation_handeler;
mod math;

use std::collections::BTreeMap;

enum EquationSlice {
    Part(),
    EquationSlice,
    End,
}

pub async fn vector_function_calculator(parametric: String) -> Vec<[i64; 2]> {
    let parsed_eq = equation_handeler::expr(&parametric);

    let mut values: Vec<[i64; 2]> = Vec::new();

    let mut counter = 0;

    while counter <= 1000 {
        let output = math::do_some_math(parsed_eq.to_string());

        values.push([counter, output]);

        counter += 1;
    }

    values
}

#[cfg(test)]
mod tests {}

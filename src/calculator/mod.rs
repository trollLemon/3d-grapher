mod equation_handeler;
mod math;

pub fn vector_function_calculator(parametric: String, t: i64) -> f64 {
    math::calculate(parametric, t)


}

#[cfg(test)]
mod tests {}

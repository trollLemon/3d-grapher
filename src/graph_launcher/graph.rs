use crate::calculator;
use fastcurve_3d::fast_curve_3d;
use gnuplot::{Caption, Color, Figure};

pub fn run(x_eq: String, y_eq: String, z_eq: String) -> Result<(), String> {
    //get some data for all three equations
    let x = calculator::get_some_data(x_eq);
    let y = calculator::get_some_data(y_eq);
    let z = calculator::get_some_data(z_eq);
    

    let n = 3;

    let (xn, yn, zn) = fast_curve_3d(&x, &y, &z, n);

    // gnu plot Figure
    let mut fg = Figure::new();
    fg.axes3d()
        .lines(&xn, &yn, &zn, &[Caption("equation"), Color("red")]);
    fg.set_scale(1.0, 1.0);
    fg.show().unwrap();

    Ok(())
}

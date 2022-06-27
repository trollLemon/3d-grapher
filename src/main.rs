#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // this will hide the console window on Windows

mod app;
mod calculator;

mod graph_launcher;
mod util;
use app::UserInput;
use eframe::{egui::vec2, run_native, NativeOptions};
use tokio;
use fastcurve_3d::fast_curve_2d;
use gnuplot::Figure;
use gnuplot::Caption;
use gnuplot::Color;
 fn main() {
    let app = UserInput::default();
    let mut window_options = NativeOptions::default();
    window_options.initial_window_size = Some(vec2(700.0, 400.0));
    window_options.resizable = true;


    //following is a visual test and will be removed in the future
    //TODO: incrememnt by small values 
    let a = calculator::vector_function_calculator("Sin(t)+t".to_string(), 0);
    let b = calculator::vector_function_calculator("Sin(t)+t".to_string(), 1);
    let c = calculator::vector_function_calculator("Sin(t)+t".to_string(), 2);
    let d = calculator::vector_function_calculator("Sin(t)+t".to_string(), 3);
    let e = calculator::vector_function_calculator("Sin(t)+t".to_string(), 4);
    let x = vec![0.0,1.0,2.0,3.0,4.0];
    let y = vec![a,b,c,d,e];
    let n: u8 = 4;


     let (xn,yn) = fast_curve_2d(&x, &y, n);


    let mut fg = Figure::new();
    fg.axes2d()
    .lines(&xn, &yn, &[Caption("Smoothed"), Color("red")]);
    
    fg.set_scale(1.0, 1.0);
    fg.show().unwrap();


    run_native(Box::new(app), window_options);
}

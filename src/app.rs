use crate::graph_launcher::spawn;
use eframe::{
    egui::{self, FontData},
    egui::{Context, FontDefinitions},
    epaint::FontFamily,
    epi::App,
};

use crate::util::remove_spaces;
use std::io;
use std::thread;
pub struct UserInput {
    input_x: String,
    input_y: String,
    input_z : String,
}

impl Default for UserInput {
    fn default() -> Self {
        Self {
            input_x: "t".to_owned(),
            input_y: "t*2".to_owned(),
            input_z: "t/2".to_owned(),
        }
    }
}

impl App for UserInput {
    fn update(&mut self, ctx: &egui::Context, _frame: &eframe::epi::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_space(30.0);
            ui.style_mut().override_text_style = Some(egui::TextStyle::Heading);
            ui.visuals_mut().override_text_color = Some(egui::Color32::LIGHT_GREEN);
            ui.label("equation");
            ui.add_space(10.0);
            ui.label("r(t)={");
            ui.label("x(t)=");
            ui.text_edit_singleline(&mut self.input_x);
            ui.label("y(t)=");  
            ui.text_edit_singleline(&mut self.input_y);
            ui.label("x(t)=");  
            ui.text_edit_singleline(&mut self.input_z);  
            ui.add_space(30.0);
            ui.label("}");

        });

        egui::SidePanel::right("side_panel").show(ctx, |ui| {
            ui.add_space(40.0);
            if ui.button("Graph").clicked() {
            let x_eq = remove_spaces(&self.input_x);        
            let y_eq = remove_spaces(&self.input_y);  
            let z_eq = remove_spaces(&self.input_z);
            
                thread::spawn(move || {

                    spawn(x_eq, y_eq, z_eq).expect("error");

                });


            }

            }
            );
    
    }        
    

    fn name(&self) -> &str {
        "3D Grapher"
    }
}

fn fill_bar(ui: &mut eframe::egui::Ui, steps: i32) {
    match steps {
        0 => {
            ui.add(egui::widgets::ProgressBar::new(0.0).desired_width(300.0));
        }
        1 => {
            ui.add(egui::widgets::ProgressBar::new(0.5).desired_width(300.0));
        }
        2 => {
            ui.add(egui::widgets::ProgressBar::new(1.0).desired_width(300.0));
        }
        _ => {
            ui.add(egui::widgets::ProgressBar::new(0.0).desired_width(300.0));
        }
    }
}

fn see_what_steps_for_graphing_are_done(
    calculation: &Option<poll_promise::Promise<Result<(), io::Error>>>,
    plotting: &Option<poll_promise::Promise<Result<(), io::Error>>>,
) {
    match calculation {
        Some(calc) => match calc.poll() {
            std::task::Poll::Ready(_) => {}
            _ => {}
        },
        None => {
            //do nothing
        }
    }

    match plotting {
        Some(plot) => match plot.poll() {
            std::task::Poll::Ready(_) => {}
            _ => {}
        },
        None => {
            //do nothing
        }
    }
}

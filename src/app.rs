use crate::graph_launcher::spawn;
use eframe::{
    egui::{self},
    epi::App,
};

use std::thread;
pub struct UserInput {
    input_x: String,
    input_y: String,
    input_z: String,
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
            ui.label("z(t)=");
            ui.text_edit_singleline(&mut self.input_z);
            ui.add_space(30.0);
            ui.label("}");
        });

        egui::SidePanel::right("side_panel").show(ctx, |ui| {
            ui.add_space(40.0);
            if ui.button("Graph").clicked() {
                let x_eq = self.input_x.to_string();
                let y_eq = self.input_y.to_string();
                let z_eq = self.input_z.to_string();

               thread::spawn(move || {
                    spawn(x_eq, y_eq, z_eq).expect("error");
                });

            }
        });

    }

    fn name(&self) -> &str {
        "3D Grapher"
    }
}

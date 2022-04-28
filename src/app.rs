
use crate::graph_launcher::spawn;
use std::thread;
use eframe::{
    egui::{self, FontData},
    egui::{Context, FontDefinitions},
    epaint::FontFamily,
    epi::App,
};

use std::io;

pub struct UserInput {
    input: String,
    message: String,
    calculation: Option<poll_promise::Promise<Result<(),io::Error>>>,
    plotting: Option<poll_promise::Promise<Result<(),io::Error>>>,
    steps: i32,
   
}

impl Default for UserInput {
    fn default() -> Self {
        Self {
            input: "equation".to_owned(),
            message: "".to_owned(),
            calculation: None,
            plotting: None,
            steps: 0
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
            ui.label("r(t)=");
            ui.text_edit_singleline(&mut self.input);
            ui.add_space(30.0);
            

            see_what_steps_for_graphing_are_done(&self.calculation, &self.plotting);

            fill_bar(ui, self.steps);
        
            
            
        });

        

        egui::SidePanel::right("side_panel").show(ctx, |ui| {
          ui.add_space(40.0);
          if ui.button("Graph").clicked() {

            //allocate another thread for the graph so the main thread can still deal with the egui stuff
            thread::spawn(|| {
                spawn().expect("bruh");
            });

              
          } 

          

          ui.add_space(40.0);
          if ui.button("Choose Color").clicked() {
              println!("f");
          } 


        });
    }

    fn name(&self) -> &str {
        "3D Grapher"
    }

    
    
}



fn fill_bar(ui: &mut eframe::egui::Ui, steps: i32) {
    match steps {
        0 =>{

            ui.add(egui::widgets::ProgressBar::new(0.0).desired_width(300.0));

        },
        1 =>{
            ui.add(egui::widgets::ProgressBar::new(0.5).desired_width(300.0));

        },
        2 => {
            ui.add(egui::widgets::ProgressBar::new(1.0).desired_width(300.0));

        },
        _ =>{
            ui.add(egui::widgets::ProgressBar::new(0.0).desired_width(300.0));

        }
    }


}

fn see_what_steps_for_graphing_are_done(calculation: &Option<poll_promise::Promise<Result<(),io::Error>>>,
plotting: &Option<poll_promise::Promise<Result<(),io::Error>>>){

    match calculation {
        Some(calc) =>{
            match calc.poll(){
                std::task::Poll::Ready(_) =>{
               
                },
                _=>{},

            }    

        },
        None => {
            //do nothing
        }
    }

    match plotting {
        Some(plot) =>{
            match plot.poll(){
                std::task::Poll::Ready(_) =>{},
                _=>{},

            }   
        },
        None => {
            //do nothing
        }
    }

}

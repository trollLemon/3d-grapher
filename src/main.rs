#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // this will hide the console window on Windows





mod app;
mod graph_launcher;
mod graph;
use app::UserInput;
use eframe::{egui::vec2, run_native, NativeOptions};
use tokio;
use graph_launcher::spawn;

#[tokio::main]
async fn main() {



     let app = UserInput::default();
     let mut window_options = NativeOptions::default();
     window_options.initial_window_size = Some(vec2(700.0, 400.0));
     window_options.resizable = true;
     run_native(Box::new(app), window_options);
}


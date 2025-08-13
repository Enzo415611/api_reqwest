mod ui;
mod test;
mod methods_http;

use std::sync::mpsc::{Receiver, Sender};
use eframe::{NativeOptions, App};
use egui::Style;

#[tokio::main]
async fn main() -> Result<(), eframe::Error> {
    eframe::run_native(
        "reqwest",
        NativeOptions::default(),
        Box::new(|_| Ok(Box::new(MyApp::default()))),
    )
}


#[derive()]
pub struct MyApp{

    // comunicação entre threads
    pub tx: Sender<String>,
    pub rx: Receiver<String>,

    pub url: String,
    types: Types,
    response_body: Option<Result<String, String>>
}

#[derive(PartialEq)]
pub enum Types{
    Get,
    Post
}


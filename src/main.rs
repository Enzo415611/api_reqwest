mod ui;
mod methods_http;

use std::sync::mpsc::{Receiver, Sender};
use eframe::{NativeOptions, App};
use serde_json::Value;

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
    
    
    pub input_body_str: String,
    pub body: Value,
    pub method_is_active: Types,
    pub url: String,
    response_body: Option<Result<String, String>>,
}

#[derive(PartialEq)]
pub enum Types{
    Get,
    Post,
    Delete,
    Put,
}


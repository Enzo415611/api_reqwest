use std::sync::mpsc::{channel};
use eframe::{App, Frame};
use eframe::epaint::{FontFamily, FontId};
use egui::{Color32, Context, Style, TextStyle};
use crate::{MyApp, Types};

impl Default for MyApp {
     fn default() -> Self {
        let (tx, rx) = channel();
        Self {
            tx,
            rx,
            style: Default::default(),
            url: String::new(),
            types: Types::Get,
            response_body: None
        }
    }
}



impl App for MyApp {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {

        match self.rx.try_recv() {
            Ok(result) => {
                self.response_body = Some(Ok(result));
            }
            Err(_) => {}
        }
        egui::CentralPanel::default().show(&ctx, |ui| {
            ui.menu_button("Methods", |ui| {
                if ui.button("Get").clicked(){}
                if ui.button("Post").clicked(){}
            });

            ui.vertical_centered(|ui|{
               ui.heading("reqwest");
               ui.add_space(100.0);
               ui.text_edit_singleline(&mut self.url);
               ui.add_space(10.0);
               ui.add_space(10.0);
                    if ui.button("check").clicked() {
                        let tx = self.tx.clone();
                        let url_to_fetch = self.url.clone();
                        self.method_get();
                        self.response_body = Some(Ok("Carregando...".to_string()));
                    }
            });

            if let Some(result) = &self.response_body {
                match result {
                    Ok(body) => {
                        ui.label(body);
                    }
                    Err(err_msg) => {
                        ui.colored_label(Color32::RED,err_msg);
                    }
                }
            }
        });

        // pede ao egui para redesenhar a Ui continuamente
        ctx.request_repaint();
    }
}


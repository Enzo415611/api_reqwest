use std::sync::mpsc::{channel};
use eframe::{App, Frame};
use egui::{vec2, Color32, Context, TextEdit, FontDefinitions, Visuals, RichText};
use serde_json::{Value};
use crate::{MyApp, Types};

impl Default for MyApp {
     fn default() -> Self {
        let (tx, rx) = channel();
        Self {
            tx,
            rx,
            method_is_active: Types::Get,
            url: String::new(),
            response_body: None,
            input_body_str: String::new(),
            body: Value::Null,
        }
    }
}



impl App for MyApp {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        let mut visuals = Visuals::dark();
        match self.rx.try_recv() {
            Ok(result) => {
                self.response_body = Some(Ok(result));
            }
            Err(_) => {}
        }


        egui::CentralPanel::default().show(&ctx, |ui| {
            ui.style_mut().spacing.item_spacing = vec2(15.0,15.0);

            ui.vertical_centered(|ui| {
                ui.heading(RichText::new("reqwest").size(30.0));
                ui.add_space(100.0);
                ui.menu_button(RichText::new("Methods").size(15.0), |ui| {
                    if ui.button(RichText::new("Get").size(15.0)).clicked(){
                        self.method_is_active = Types::Get;
                    }
                    if ui.button(RichText::new("Post").size(15.0)).clicked(){
                        self.method_is_active = Types::Post;
                    }
                    if ui.button(RichText::new("Delete").size(15.0)).clicked(){
                        self.method_is_active = Types::Delete;
                    }
                    if ui.button(RichText::new("Put").size(15.0)).clicked(){
                        self.method_is_active = Types::Put;
                    }
                });
                ui.label(RichText::new("URL").size(15.0));
                ui.add_sized([600.0, 30.0], TextEdit::singleline(&mut self.url));
                ui.label(RichText::new("JSON").size(15.0));
                ui.code_editor(&mut self.input_body_str);
                if ui.button(RichText::new("Send").size(15.0)).clicked() {
                    let tx = self.tx.clone();
                    let url_to_fetch = self.url.clone();
                    self.verifier_method();
                    self.response_body = Some(Ok("Err".to_string()));
                }
                if let Some(result) = &self.response_body {
                    match result {
                        Ok(body) => {
                            ui.text_edit_multiline(&mut body.to_string());
                        }
                        Err(err_msg) => {
                            ui.colored_label(Color32::RED,err_msg);
                        }
                    }
                }

            });

        });

        // pede ao egui para redesenhar a Ui continuamente
        ctx.request_repaint();
    }


}


use std::sync::mpsc::{channel};
use eframe::{App, Frame};
use egui::{vec2, Color32, Context, TextStyle, TextEdit};
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
        match self.rx.try_recv() {
            Ok(result) => {
                self.response_body = Some(Ok(result));
            }
            Err(_) => {}
        }


        egui::CentralPanel::default().show(&ctx, |ui| {
            ui.style_mut().spacing.item_spacing = vec2(10.0, 10.0);
            ui.style_mut().spacing.interact_size = vec2(80.0, 30.0);
            ui.style_mut().text_styles.get_mut(&TextStyle::Button).unwrap().size = 50.0;

            ui.vertical_centered(|ui| {

                ui.heading("reqwest");
                    ui.add_space(100.0);
                    ui.menu_button("Methods", |ui| {
                       if ui.button("Get").clicked(){
                           self.method_is_active = Types::Get;
                           println!("get");
                       }
                       if ui.button("Post").clicked(){
                           println!("post");
                           self.method_is_active = Types::Post;
                       }
                    });
                    ui.add_space(10.0);
                    ui.label("URL");
                    ui.add_sized([600.0, 30.0], TextEdit::singleline(&mut self.url));
                    ui.label("JSON");
                    ui.code_editor(&mut self.input_body_str);
                    ui.add_space(10.0);
                    if ui.button("send").clicked() {
                       let tx = self.tx.clone();
                       let url_to_fetch = self.url.clone();
                       self.verifier_method();
                       self.response_body = Some(Ok("Erro...".to_string()));
                    }
                ui.add_space(20.0);

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


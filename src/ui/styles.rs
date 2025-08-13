use eframe::epaint::FontFamily;
use egui::style::{StyleModifier, Style};
use egui::{FontId, TextStyle};
use crate::MyApp;

impl MyApp{
    pub fn styles(&mut self){
        self.style.text_styles = [

            (TextStyle::Heading, egui::FontId::new(30.0, FontFamily::Proportional)),
            (TextStyle::Body, FontId::new(20.0, FontFamily::Proportional))
        ].into();
    }
}
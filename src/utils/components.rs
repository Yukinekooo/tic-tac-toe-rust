use egui::{Color32, RichText, Ui};

pub fn new_label(ui: &mut Ui, text: &str) {
    ui.label(
        RichText::new(text)
            .color(Color32::from_rgb(255, 255, 255))
            .strong()
            .size(20.0),
    );
}

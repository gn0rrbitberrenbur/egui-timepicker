use egui_timepicker::{TimePickerWatch, TimePickerSimple};

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "TimePicker Demo",
        options,
        Box::new(|_| Box::new(App::default())),
    )
}

#[derive(Default)]
struct App {
    picker_watch: TimePickerWatch,
    picker_simple: TimePickerSimple
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        let mut style = (*ctx.style()).clone();

        style.text_styles = [
            (egui::TextStyle::Heading, egui::FontId::proportional(28.0)),
            (egui::TextStyle::Body, egui::FontId::proportional(18.0)),
            (egui::TextStyle::Button, egui::FontId::proportional(18.0)),
            (egui::TextStyle::Small, egui::FontId::proportional(14.0)),
            (egui::TextStyle::Monospace, egui::FontId::monospace(16.0)),
        ]
        .into();

        ctx.set_style(style);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add(&mut self.picker_watch);

            let (hour_watch, minute_watch) = self.picker_watch.time();

            ui.add_space(10.0);

            ui.label(format!(
                "Chosen time: {:02}:{:02}",
                hour_watch, minute_watch
            ));

            ui.add(&mut self.picker_simple);

            let (hour_simple, minute_simple) = self.picker_simple.time();

            ui.add_space(10.0);

            ui.label(format!(
                "Chosen time: {:02}:{:02}",
                hour_simple, minute_simple 
            ));
        });
    }
}

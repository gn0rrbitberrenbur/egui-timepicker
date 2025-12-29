use egui::*;

#[derive(Clone, Debug)]
pub struct TimePicker {
    hour: u8,
    minute: u8,
    open: bool,
    dragging: DragTarget,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum DragTarget {
    None,
    Hour,
    Minute,
}

impl Default for TimePicker {
    fn default() -> Self {
        Self::new(12, 0)
    }
}

impl TimePicker {
    pub fn new(hour: u8, minute: u8) -> Self {
        Self {
            hour: hour.min(23),
            minute: minute.min(59),
            open: false,
            dragging: DragTarget::None,
        }
    }

    pub fn time(&self) -> (u8, u8) {
        (self.hour, self.minute)
    }
}

impl Widget for &mut TimePicker {
    fn ui(self, ui: &mut Ui) -> Response {
        let response = ui.button(format!("{:02}:{:02}", self.hour, self.minute));

        if response.clicked() {
            self.open = true;
        }

        if self.open {
            Window::new("Timepicker")
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                .show(ui.ctx(), |ui| {

                    ui.vertical_centered(|ui| {
                        draw_clock(ui, self);
                    });

                    ui.add_space(8.0);

                    ui.horizontal(|ui| {
                        ui.add(egui::DragValue::new(&mut self.hour).clamp_range(0..=23));
                        ui.label(":");
                        ui.add(egui::DragValue::new(&mut self.minute).clamp_range(0..=59));
                    });

                    ui.add_space(8.0);

                    if ui.button("OK").clicked() {
                        self.open = false;
                    }
                });
        }

        response
    }
}

fn draw_clock(ui: &mut Ui, picker: &mut TimePicker) {
    let size = 220.0;
    let (rect, response) =
        ui.allocate_exact_size(vec2(size, size), Sense::drag());
    let painter = ui.painter();
    let center = rect.center();
    let radius = size * 0.45;

    painter.circle_filled(
        center,
        radius,
        ui.visuals().widgets.inactive.bg_fill,
    );

    for i in 1..=12 {
        let angle = (i as f32 / 12.0) * std::f32::consts::TAU
            - std::f32::consts::FRAC_PI_2;
        let pos =
            center + vec2(angle.cos(), angle.sin()) * radius * 0.82;

        painter.text(
            pos,
            Align2::CENTER_CENTER,
            i.to_string(),
            FontId::proportional(16.0),
            ui.visuals().text_color(),
        );
    }

    draw_hand(
        painter,
        center,
        radius * 0.55,
        picker.hour as f32 / 12.0,
        Color32::WHITE,
        4.0,
    );

    draw_hand(
        painter,
        center,
        radius * 0.75,
        picker.minute as f32 / 60.0,
        Color32::LIGHT_BLUE,
        3.0,
    );

    if response.dragged() {
        if let Some(pos) = response.interact_pointer_pos() {
            let delta = pos - center;
            let angle = delta.y.atan2(delta.x)
                + std::f32::consts::FRAC_PI_2;
            let norm = (angle + std::f32::consts::TAU)
                % std::f32::consts::TAU;

            match picker.dragging {
                DragTarget::Hour => {
                    picker.hour =
                        ((norm / std::f32::consts::TAU) * 12.0)
                            .round() as u8
                            % 24;
                }
                DragTarget::Minute => {
                    picker.minute =
                        ((norm / std::f32::consts::TAU) * 60.0)
                            .round() as u8
                            % 60;
                }
                DragTarget::None => {
                    picker.dragging = if delta.length() < radius * 0.6 {
                        DragTarget::Hour
                    } else {
                        DragTarget::Minute
                    };
                }
            }
        }
    }

    if response.drag_released() {
        picker.dragging = DragTarget::None;
    }
}

fn draw_hand(
    painter: &Painter,
    center: Pos2,
    length: f32,
    value: f32,
    color: Color32,
    width: f32,
) {
    let angle =
        value * std::f32::consts::TAU - std::f32::consts::FRAC_PI_2;
    let end = center + vec2(angle.cos(), angle.sin()) * length;

    painter.line_segment(
        [center, end],
        Stroke::new(width, color),
    );
}
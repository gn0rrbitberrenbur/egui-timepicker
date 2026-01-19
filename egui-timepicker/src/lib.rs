use egui::*;

#[derive(Clone, Debug)]
pub struct TimePickerWatch {
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

impl Default for TimePickerWatch {
    fn default() -> Self {
        Self::new(12, 0)
    }
}

impl TimePickerWatch {
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

impl Widget for &mut TimePickerWatch {
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

fn draw_clock(ui: &mut Ui, picker: &mut TimePickerWatch) {
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

#[derive(Clone, Debug)]
pub struct TimePickerSimple {
    hour: u8,
    minute: u8,
    open: bool,
}

impl Default for TimePickerSimple {
    fn default() -> Self {
        Self {
            hour: 12,
            minute: 0,
            open: false,
        }
    }
}

impl TimePickerSimple {
    pub fn time(&self) -> (u8, u8) {
        (self.hour, self.minute)
    }

    fn inc_hour(&mut self) {
        self.hour = (self.hour + 1) % 24;
    }

    fn dec_hour(&mut self) {
        self.hour = if self.hour == 0 { 23 } else { self.hour - 1 };
    }

    fn inc_minute(&mut self) {
        self.minute = (self.minute + 1) % 60;
    }

    fn dec_minute(&mut self) {
        self.minute = if self.minute == 0 { 59 } else { self.minute - 1 };
    }
}

const COL_WIDTH: f32 = 40.0;
const ROW_HEIGHT: f32 = 32.0;


impl Widget for &mut TimePickerSimple {
    fn ui(self, ui: &mut Ui) -> Response {
        let response = ui.button(format!("{:02}:{:02}", self.hour, self.minute));

        if response.clicked() {
            self.open = true;
        }

        if self.open {
            Window::new("Timepicker")
                .collapsible(false)
                .resizable(false)
                .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
                .show(ui.ctx(), |ui| {
                    ui.vertical_centered(|ui| {
                        ui.horizontal(|ui| {
                            // Hour column
                            ui.vertical(|ui| {
                                if ui.add_sized(
                                    [COL_WIDTH, ROW_HEIGHT],
                                    egui::Button::new("^")).clicked() {
                                    self.inc_hour();
                                }

                                ui.label(
                                    RichText::new(format!("{:02}", self.hour))
                                        .size(24.0),
                                );

                                if ui.add_sized(
                                    [COL_WIDTH, ROW_HEIGHT],
                                    egui::Button::new("v")).clicked() {
                                    self.dec_hour();
                                }
                            });

                            ui.add_space(8.0);
                            ui.label(RichText::new(":").size(24.0));
                            ui.add_space(8.0);

                            // Minute column
                            ui.vertical(|ui| {
                                if ui.add_sized(
                                    [COL_WIDTH, ROW_HEIGHT],
                                    egui::Button::new("^")).clicked() {
                                    self.inc_minute();
                                }

                                ui.label(
                                    RichText::new(format!("{:02}", self.minute))
                                        .size(24.0),
                                );

                                if ui.add_sized(
                                    [COL_WIDTH, ROW_HEIGHT],
                                    egui::Button::new("v")).clicked() {
                                    self.dec_minute();
                                }
                            });
                        });
                    });

                    ui.add_space(10.0);

                    if ui.button("OK").clicked() {
                        self.open = false;
                    }
                });
        }

        response
    }
}

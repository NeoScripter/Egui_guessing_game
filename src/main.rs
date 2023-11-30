
use egui::{FontFamily, FontId, RichText, TextStyle, Color32};
use rand::Rng;
use std::cmp::Ordering;
use eframe::{
    egui::{self, Event, ViewportCommand},
};
use image::GenericImageView;
fn set_global_text_color(ctx: &egui::Context) {
    let mut style = (*ctx.style()).clone();

    // Set the global text color override to white
    style.visuals.override_text_color = Some(egui::Color32::from_rgb(127, 127, 127));
        let white_stroke = egui::Stroke::new(2.0, egui::Color32::from_rgb(127, 127, 127)); // You can adjust the width (2.0) as needed
    style.visuals.widgets.active.bg_stroke = white_stroke;
    style.visuals.widgets.hovered.bg_stroke = white_stroke;
    style.visuals.widgets.noninteractive.bg_stroke = white_stroke;

    // Set the text cursor color
    style.visuals.text_cursor = white_stroke;

    // Apply the modified style back to the context
    ctx.set_style(style);
}
fn configure_text_styles(ctx: &egui::Context) {
    use FontFamily::{Monospace, Proportional};

    let mut style = (*ctx.style()).clone();
    style.text_styles = [
        (TextStyle::Heading, FontId::new(25.0, Proportional)),
        (TextStyle::Body, FontId::new(25.0, Proportional)),
        (TextStyle::Monospace, FontId::new(12.0, Monospace)),
        (TextStyle::Button, FontId::new(25.0, Proportional)),
        (TextStyle::Small, FontId::new(8.0, Proportional)),
    ]
    .into();
    ctx.set_style(style);
}
fn custom_window_frame(ctx: &egui::Context, title: &str, add_contents: impl FnOnce(&mut egui::Ui)) {
    use egui::*;

    let panel_frame = egui::Frame {
        fill: Color32::from_rgb(27, 27, 27),
        rounding: 10.0.into(),
        stroke: ctx.style().visuals.widgets.noninteractive.fg_stroke,
        outer_margin: 0.5.into(), // so the stroke is within the bounds
        ..Default::default()
    };

    CentralPanel::default().frame(panel_frame).show(ctx, |ui| {
        let app_rect = ui.max_rect();

        let title_bar_height = 32.0;
        let title_bar_rect = {
            let mut rect = app_rect;
            rect.max.y = rect.min.y + title_bar_height;
            rect
        };
        title_bar_ui(ui, title_bar_rect, title);

        // Add the contents:
        let content_rect = {
            let mut rect = app_rect;
            rect.min.y = title_bar_rect.max.y;
            rect
        }
        .shrink(4.0);
        let mut content_ui = ui.child_ui(content_rect, *ui.layout());
        add_contents(&mut content_ui);
    });
}
fn title_bar_ui(ui: &mut egui::Ui, title_bar_rect: eframe::epaint::Rect, title: &str) {
    use egui::*;

    let painter = ui.painter();

    let title_bar_response = ui.interact(title_bar_rect, Id::new("title_bar"), Sense::click());

    // Paint the title:
    painter.text(
        title_bar_rect.center(),
        Align2::CENTER_CENTER,
        title,
        FontId::proportional(20.0),
        ui.style().visuals.text_color(),
    );

    // Paint the line under the title:
    painter.line_segment(
        [
            title_bar_rect.left_bottom() + vec2(1.0, 0.0),
            title_bar_rect.right_bottom() + vec2(-1.0, 0.0),
        ],
        ui.visuals().widgets.noninteractive.bg_stroke,
    );

    // Interact with the title bar (drag to move window):
    if title_bar_response.double_clicked() {
        let is_maximized = ui.input(|i| i.viewport().maximized.unwrap_or(false));
        ui.ctx()
            .send_viewport_cmd(ViewportCommand::Maximized(!is_maximized));
    } else if title_bar_response.is_pointer_button_down_on() {
        ui.ctx().send_viewport_cmd(ViewportCommand::StartDrag);
    }

    ui.allocate_ui_at_rect(title_bar_rect, |ui| {
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.spacing_mut().item_spacing.x = 0.0;
            ui.visuals_mut().button_frame = false;
            ui.add_space(8.0);
            close_maximize_minimize(ui);
        });
    });
}
fn close_maximize_minimize(ui: &mut egui::Ui) {
    use egui::{Button, RichText};

    let button_height = 20.0;

    let close_response = ui
        .add(Button::new(RichText::new("❌").size(button_height)));
    if close_response.clicked() {
        ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
    }

    let is_maximized = ui.input(|i| i.viewport().maximized.unwrap_or(false));
    if is_maximized {
        let maximized_response = ui
            .add(Button::new(RichText::new("🗗").size(button_height)));
        if maximized_response.clicked() {
            ui.ctx()
                .send_viewport_cmd(ViewportCommand::Maximized(false));
        }
    } else {
        let maximized_response = ui
            .add(Button::new(RichText::new("🗗").size(button_height)));
        if maximized_response.clicked() {
            ui.ctx().send_viewport_cmd(ViewportCommand::Maximized(true));
        }
    }

    let minimized_response = ui
        .add(Button::new(RichText::new("🗕").size(button_height)));
    if minimized_response.clicked() {
        ui.ctx().send_viewport_cmd(ViewportCommand::Minimized(true));
    }
}
struct Game {
    user_input: String,
    secret_number: u32,
    message: String,
    last_guess: Option<u32>,
    display_win_buttons: bool,
    message_color: Color32,
    texture: Option<(egui::Vec2, egui::TextureId)>,
}

impl Game {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        set_global_text_color(&cc.egui_ctx);
        Self {
            user_input: String::new(),
            secret_number: rand::thread_rng().gen_range(1..=100),
            message: String::from("Guess the number!"),
            last_guess: None,
            display_win_buttons: false,
            message_color: Color32::from_rgb(127, 127, 127),
            texture: None,
        }
    }
}
impl Default for Game {
    fn default() -> Self {
        Self {
            user_input: String::new(),
            secret_number: rand::thread_rng().gen_range(1..=100),
            message: String::from("Guess the number!"),
            last_guess: None,
            display_win_buttons: false,
            message_color: Color32::from_rgb(127, 127, 127),
            texture: None,
        }
    }
}

impl eframe::App for Game {
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        egui::Rgba::TRANSPARENT.to_array() // Make sure we don't paint anything behind the rounded corners
    }
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        ctx.set_pixels_per_point(2.0);
        custom_window_frame(ctx, "", |ui| {
            ui.vertical_centered(|ui| ui.heading("Guessing Game"));
            let image = image::open("crab.png").expect("Failed to open image");
            let image_buffer = image.to_rgba8(); // Convert image to RGBA format
            let dimensions = image.dimensions();
            let texture_options = egui::TextureOptions {
                magnification: egui::TextureFilter::Linear,
                minification: egui::TextureFilter::Linear,
            };
            
            let texture_handle = ctx.load_texture(
                "unique_image_name",
                egui::ColorImage::from_rgba_unmultiplied(
                    [dimensions.0 as _, dimensions.1 as _],
                    &image_buffer.into_raw(),
                ),
                texture_options,
            );
            
            ui.image(&texture_handle);
            ui.add_space(ui.available_height() / 3.5);
            ui.vertical_centered(|ui| {
                ui.set_max_width(200.0);
                //ui.label(&self.message);
                ui.style_mut().visuals.extreme_bg_color = egui::Color32::BLACK; // Set the desired dark background color
                let response = ui.add(
                    egui::TextEdit::singleline(&mut self.user_input)
                        .desired_width(370.0)
                );
                if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                    match self.user_input.trim().parse::<u32>() {
                        Ok(guess) => {
                            self.last_guess = Some(guess);
                            self.user_input.clear();
                            match guess.cmp(&self.secret_number) {
                                Ordering::Less => {
                                    self.message = "Too small!".to_string();
                                    self.message_color = Color32::RED;  // Set color to red
                                },
                                Ordering::Greater => {
                                    self.message = "Too big!".to_string();
                                    self.message_color = Color32::RED;  // Default color
                                },
                                Ordering::Equal => {
                                    self.message = "You win! Congratulations!".to_string();
                                    self.message_color = Color32::GREEN;
                                    self.display_win_buttons = true; 
                                },
                            }
                        }
                        Err(_) => {
                            self.message = "Please enter a valid number!".to_string();
                            self.message_color = Color32::RED;
                            self.user_input.clear();
                        }
                    }
                }
                ui.colored_label(self.message_color, &self.message);
                if !self.display_win_buttons {
                    response.request_focus();
                }
                if let Some(guess) = self.last_guess {
                    ui.label(format!("You guessed: {}", guess));
                }
                if self.display_win_buttons {
                    ui.horizontal(|ui| {
                        if ui.button("Quit").clicked() {
                            ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                        }
    
                        if ui.button("Play again").clicked() {
                            self.user_input.clear();
                            self.secret_number = rand::thread_rng().gen_range(1..=100);
                            self.message = "Guess the number!".to_string();
                            self.display_win_buttons = false; 
                        }
                    });
                }
            });
        });
    }    
}

fn main() -> eframe::Result<()>  {  
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_decorations(false) // Hide the OS-specific "chrome" around the window
            .with_inner_size([500.0, 500.0])
            .with_min_inner_size([500.0, 500.0])
            .with_transparent(true), // To have rounded corners we need transparency
    
        ..Default::default()
    };
    eframe::run_native(
        "Guessing Game", 
        options,
        Box::new(|cc| Box::new(Game::new(cc))),
    )
}  
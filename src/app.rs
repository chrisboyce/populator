use egui::{Color32, RichText};
use hex::FromHex;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct Populator {
    /// Persist UI mode and settings
    settings: Settings,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Settings {
    equation_settings: EquationSettings,
    color_settings: ColorSettings,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct ColorSettings {
    #[serde(skip)]
    show_color_picker: bool,
    color: Color32,
    color_as_string: String,
}
#[derive(serde::Deserialize, serde::Serialize)]
pub struct EquationSettings {
    #[serde(skip)]
    show_keypad: bool,
    intput: String,
    output: String,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub enum ViewMode {}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct ViewSettings {
    show_colors: bool,
}

impl Default for Populator {
    fn default() -> Self {
        Self {
            // user_input: "1+2".to_string(),
            // result: "3".to_string(),
            // Example stuff:
            settings: Settings {
                equation_settings: EquationSettings {
                    intput: "1+2".to_string(),
                    output: "3".to_string(),
                    show_keypad: false,
                },
                color_settings: ColorSettings {
                    color_as_string: "0xff0000".to_string(),
                    color: Color32::from_rgb(255, 128, 64),
                    show_color_picker: false,
                },
            },
        }
    }
}

impl Populator {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for Populator {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        let mut visual = ctx.style().visuals.clone();
        visual.panel_fill = Color32::from_rgb(0, 0, 0);
        ctx.set_visuals(visual);

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // ui.style_mut().visuals.panel_fill = Color32::from_rgb(255, 0, 0);
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    let mut visual = ctx.style().visuals.clone();
                    visual.panel_fill = Color32::from_rgb(255, 0, 0);
                    ctx.set_visuals(visual);

                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    // let mut visuals = ctx.style().visuals.clone();
                    // visuals.window_fill = Color32::RED;
                    // ctx.set_visuals(visuals);

                    ui.menu_button("View", |ui| {
                        if ui
                            .checkbox(&mut self.settings.equation_settings.show_keypad, "Keypad")
                            .changed()
                        {
                            if self.settings.equation_settings.show_keypad {
                                ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(
                                    egui::vec2(400.0, 400.0),
                                ));
                            }
                        }
                        ui.checkbox(
                            &mut self.settings.color_settings.show_color_picker,
                            "Color Picker",
                        );
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_dark_light_mode_buttons(ui);
            });
        });

        let mut visual = ctx.style().visuals.clone();
        visual.panel_fill = Color32::from_rgb(190, 190, 190); //Medium Gray
        ctx.set_visuals(visual);

        egui::SidePanel::left("my_left_panel")
            .exact_width(10.0)
            .show(ctx, |ui| {});

        egui::SidePanel::right("my_right_panel")
            .exact_width(25.0)
            .show(ctx, |ui| {});

        egui::TopBottomPanel::bottom("my_bottom_panel").show(ctx, |ui| {});

        egui::TopBottomPanel::top("my_top_panel").show(ctx, |ui| {});

        let mut visual = ctx.style().visuals.clone();
        visual.panel_fill = Color32::from_rgb(211, 211, 211); //Light Gray
        ctx.set_visuals(visual);

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's

            ui.spacing_mut().item_spacing.y = 15.0;   

            ui.heading(
                RichText::new("                     SnArKn Calculator                      ")
                    .size(20.0)
                    .color(Color32::WHITE)
                    .background_color(Color32::from_rgb(0, 0, 0)),
                    

                    
            );
            
            ui.spacing_mut().item_spacing.y = 10.0;

            ui.label(
                RichText::new("  Enter Expression Below:  ")
                    .size(15.0)
                    .color(Color32::WHITE)
                    .background_color(Color32::from_rgb(0, 0, 0)),
            );
            //ui.heading("Evaluate Expression");
            // Move to the next line }
            ui.vertical(|ui| {
                //ui.horizontal(|ui| {
                let user_input =
                    ui.text_edit_singleline(&mut self.settings.equation_settings.intput);

                if user_input.changed() {
                    let new_result = calculate_result(&self.settings.equation_settings.intput);
                    if let Some(new_result) = new_result {
                        self.settings.equation_settings.output = new_result;
                    }
                }

                //ui.label(format!("= {}", self.settings.equation_settings.output));

                ui.label(
                    RichText::new(format!("= {}", self.settings.equation_settings.output))
                        .size(15.0)
                        .color(Color32::WHITE)
                        .background_color(Color32::from_rgb(0, 0, 0)),
                );
            });

            if self.settings.equation_settings.show_keypad {
                ui.separator();
                let mut buttons = vec![];
                // We want to create a number pad in the following format:
                //
                // 7 8 9
                // 4 5 6
                // 1 2 3
                ui.vertical(|ui| {
                    // Create 3 rows, with `row` having a value of 0..=2
                    for row in 0..3 {
                        ui.horizontal(|ui| {
                            // Though we are counting 0..=2, iwe want the first row
                            // to will  contain the "7 8 9" , ie we need to reverse
                            // the row order. To do that, we subtract 2 from the
                            // current row index to count backwards from the "end"
                            // index of 2
                            let rev_row = 2 - row;

                            for i in (rev_row * 3)..(rev_row * 3 + 3) {
                                // We actually want to start counting from 1 instead
                                // of 0, so we add 1 here
                                let value = i + 1;
                                let value_as_string = format!("{}", value);
                                let button = ui.button(value_as_string.clone());

                                // Save the button in an array so that we can iterate
                                // over all of them at once to perform common
                                // behavior.
                                buttons.push((button, value_as_string))
                            }
                        });
                    }

                    ui.horizontal(|ui| buttons.push((ui.button("0"), "0".to_string())));
                    ui.horizontal(|ui| {
                        buttons.push((ui.button("+"), "+".to_string()));
                        buttons.push((ui.button("-"), "+".to_string()))
                    });
                });

                for (button, value) in buttons {
                    if button.clicked() {
                        self.settings
                            .equation_settings
                            .intput
                            .push_str(&format!("{value}"));
                        let new_result = calculate_result(&self.settings.equation_settings.intput);
                        if let Some(new_result) = new_result {
                            self.settings.equation_settings.output = new_result;
                        }
                    }
                }
            }

            if self.settings.color_settings.show_color_picker {
                ui.separator();
                ui.heading("Color Picker");
                ui.horizontal(|ui| {
                    let color_picker =
                        ui.color_edit_button_srgba(&mut self.settings.color_settings.color);
                    if color_picker.changed() {
                        self.settings.color_settings.color_as_string =
                            format!("{}", self.settings.color_settings.color.to_hex())
                    }

                    let color_text =
                        ui.text_edit_singleline(&mut self.settings.color_settings.color_as_string);
                    ui.label(format!(
                        "({},{},{})",
                        self.settings.color_settings.color.r(),
                        self.settings.color_settings.color.g(),
                        self.settings.color_settings.color.b(),
                    ));
                    if color_text.changed() {
                        // The user has updated the text of the color string.
                        // Try to parse it as one of the various types of inputs
                        println!("Parsing:{} ", self.settings.color_settings.color_as_string);
                        if !self.settings.color_settings.color_as_string.is_empty() {
                            // try to parse #RRGGBBxx format by skipping the first
                            // character (#) and sending the rest to the `hex`
                            // crate to try to parse.
                            let color_as_string =
                                &self.settings.color_settings.color_as_string.as_str()[1..];

                            if let Ok([r, g, b, a]) = <[u8; 4]>::from_hex(color_as_string) {
                                self.settings.color_settings.color =
                                    Color32::from_rgba_unmultiplied(r, g, b, a);
                            } else if let Ok([r, g, b]) = <[u8; 3]>::from_hex(color_as_string) {
                                self.settings.color_settings.color = Color32::from_rgb(r, g, b);
                            }

                            // try to parse #RRGGBB format by skipping the first
                            // character and sending the rest to the `hex` crate
                        }
                    }
                });
            }

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                // powered_by_egui_and_eframe(ui);
                egui::warn_if_debug_build(ui);
            });
        });
    }
}

fn calculate_result(user_input: &String) -> Option<String> {
    let mut context = fend_core::Context::new();
    if let Ok(result) = fend_core::evaluate(user_input, &mut context) {
        Some(result.get_main_result().to_string())
    } else {
        None
    }
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}

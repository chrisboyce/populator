#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    
    use egui::{Color32, Visuals};
    

    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 500.0])
            .with_min_inner_size([300.0, 220.0])
            .with_icon(
                // NOE: Adding an icon is optional
                eframe::icon_data::from_png_bytes(&include_bytes!("../assets/icon-256.png")[..])
                    .unwrap(),
            ),
        ..Default::default()
    };
    eframe::run_native(
        "populator",
        native_options,
        Box::new(|cc| {
            // Use the "dark" colors as the basis for modification
            let mut visuals = Visuals::dark();

            // [Panels](https://docs.rs/egui/latest/egui/containers/panel/index.html)
            // "take up space" in the UI, e.g. `CentralPanel`, `SidePanel`,
            // and `TopBottomPanel`. This color appears to be the background
            // for most of the UI, notably not including input/menu elements.
            // Text inputs, chekboxes, drop-down menus, etc. have background
            // colors overridden in other places, e.g. [Widgets](https://docs.rs/egui/latest/egui/style/struct.Widgets.html)
            // which have an entire [WidgetVisual](https://docs.rs/egui/latest/egui/style/struct.WidgetVisuals.html)
            // style defined for 5 "states": noninteractive, inactive, hovered,
            // active, and open. This means to make colors consisent, we'll need
            // to change the "background" color in multiple places (if we want
            // them to be the same).
            visuals.panel_fill = Color32::DARK_RED;

            // [Windows](https://docs.rs/egui/latest/egui/containers/struct.Window.html)
            // are floating UI elements. [Menus](https://docs.rs/egui/latest/egui/menu/index.html)
            // appear to be implemented in some part as a `Window` since this
            // color affects menu background color
            visuals.window_fill = Color32::DARK_GREEN;

            // From [the docs](https://docs.rs/egui/latest/egui/style/struct.Visuals.html#structfield.override_text_color)
            // > If text_color is None (default), then the text color will be the
            // > same as the foreground stroke color (WidgetVisuals::fg_stroke)
            // > and will depend on whether or not the widget is being interacted
            // > with.
            visuals.override_text_color = Some(Color32::WHITE);

            // This changes the [background color](https://docs.rs/egui/latest/egui/style/struct.WidgetVisuals.html#structfield.bg_fill)
            // of inactive widgets. This makes the checkbox background appear
            // dark blue until it is interacted  with in some way.
            visuals.widgets.inactive.bg_fill = Color32::DARK_BLUE;

            // Burried in [the docs](https://docs.rs/egui/latest/egui/widgets/text_edit/struct.TextEdit.html#other),
            // we can change the text input background color by changing the
            // `extreme_bg_color`
            visuals.extreme_bg_color = Color32::GRAY;
            //visuals.extreme_bg_color = Color32::DARK_RED;
    

            cc.egui_ctx.set_visuals(visuals);
            Box::new(populator::Populator::new(cc))
        }),
    )
}

// When compiling to web using trunk:
#[cfg(target_arch = "wasm32")]
fn main() {
    // Redirect `log` message to `console.log` and friends:
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        eframe::WebRunner::new()
            .start(
                "the_canvas_id", // hardcode it
                web_options,
                Box::new(|cc| Box::new(populator::Populator::new(cc))),
            )
            .await
            .expect("failed to start eframe");
    });
}

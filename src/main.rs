fn main() -> eframe::Result {
    env_logger::init();

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([300.0, 300.0])
            .with_min_inner_size([250.0, 250.0])
            .with_titlebar_buttons_shown(false)
            .with_title_shown(false)
            .with_titlebar_shown(false)
            .with_fullsize_content_view(true)
            .with_icon(
                eframe::icon_data::from_png_bytes(&include_bytes!("../assets/icon.png")[..])
                    .expect("Failed to load icon"),
            ),
        follow_system_theme: true,
        ..Default::default()
    };

    eframe::run_native(
        "nanotuber",
        native_options,
        Box::new(|cc| Ok(Box::new(nanotuber::NanoTuber::new(cc)))),
    )

}
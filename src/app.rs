use eframe::App;
use egui::style::HandleShape;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Stream, StreamConfig};
use std::sync::{Arc, Mutex};

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct NanoTuber {
    label: String,
    #[serde(skip)]
    value: f32,
    #[serde(skip)]
    stream: Option<Stream>,
    #[serde(skip)]
    shared_value: Arc<Mutex<f32>>, // Shared value between audio thread and UI
}

impl Default for NanoTuber {
    fn default() -> Self {
        Self {
            label: "Hello World!".to_owned(),
            value: 2.7,
            stream: None,
            shared_value: Arc::new(Mutex::new(2.7)),
        }
    }
}

impl NanoTuber {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut app: NanoTuber = if let Some(storage) = cc.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            Default::default()
        };

        // Initialize the microphone stream
        app.initialize_microphone_stream();

        app
    }

    fn initialize_microphone_stream(&mut self) {
        let shared_value = Arc::clone(&self.shared_value);
    
        let host = cpal::default_host();
        let device = host.default_input_device().expect("Failed to get default input device");
        let config = device.default_input_config().expect("Failed to get default input config");
    
        let stream = device
            .build_input_stream(
                &config.into(),
                move |data: &[f32], _: &cpal::InputCallbackInfo| {
                    let rms: f32 = (data.iter().map(|&v| v * v).sum::<f32>() / data.len() as f32).sqrt();
                    let mut value = shared_value.lock().unwrap();
                    *value = rms * 10000.0; // Scale RMS to the range 0-100
                },
                |err| eprintln!("Error: {:?}", err),
            )
            .expect("Failed to build input stream");
    
        stream.play().expect("Failed to start input stream");
    
        self.stream = Some(stream);
    }
    
}

impl App for NanoTuber {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        log::info!("Microphone RMS value: {}", self.value);
        // Update `self.value` from the shared value
        {
            let shared_value = self.shared_value.lock().unwrap();
            self.value = *shared_value;
        }

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            let is_web = cfg!(target_arch = "wasm32");
            if !is_web {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
                ui.add_space(16.0);
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add(
                egui::Slider::new(&mut self.value, 0.0..=100.0)
                    .text("Sound Threshold")
                    .trailing_fill(true)
                    .handle_shape(HandleShape::Rect { aspect_ratio: 0.4 }),
            );

            if ui.button("Increment").clicked() {
                self.value += 10.0;
            }

            ui.separator();
        });
    }
}

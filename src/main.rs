#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod fp_engine;

use fp_engine::{LowLatencyFloat, StandardFloat};

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([700.0, 420.0]),
        ..Default::default()
    };

    eframe::run_native(
        "IEEE754 Paweł Perek",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_zoom_factor(2.0);

            Box::<MyApp>::default()
        }),
    )
}

#[derive(PartialEq)]
enum Mode {
    ToInternal,
    ToIEEE754,
}

struct MyApp {
    input: String,
    mode: Mode,
    result: Option<u32>,
    error: Option<String>,
}

const TO_IEEE754_DEFAULT: &str = "8F7F FF00";
const TO_INTERNAL_DEFAULT: &str = "477F FF00";

impl Default for MyApp {
    fn default() -> Self {
        Self {
            input: TO_IEEE754_DEFAULT.to_owned(),
            mode: Mode::ToIEEE754,
            result: None,
            error: None,
        }
    }
}

#[derive(PartialEq, Debug)]
enum InputFormat {
    Binary,
    Hexadecimal,
    None
}

fn input_format(input: &str) -> InputFormat {
    let clean_input = input.replace(" ", "").to_lowercase();

    if clean_input.chars().all(|c| c == '0' || c == '1') {
        InputFormat::Binary
    }
    else if clean_input.starts_with("0x") || clean_input.chars().all(|c| c.is_digit(16)) {
        InputFormat::Hexadecimal
    }  else {
        InputFormat::None
    }
}

fn parse_input(input: &str, mode: &Mode, result: &mut Option<u32>, error: &mut Option<String>) {
    *error = None;
    
    let clean_input = input.replace(" ", "").to_lowercase();

    let format = input_format(input);

    let parsed_input = match format {
        InputFormat::Binary => u32::from_str_radix(&clean_input.replace(" ", ""), 2).map_err(|e| e.to_string()),
        InputFormat::Hexadecimal => u32::from_str_radix(&clean_input.trim_start_matches("0x"), 16).map_err(|e| e.to_string()),
        InputFormat::None => Err("Nieprawidłowy format wejścia".to_owned())
    };

    match parsed_input {
        Ok(num) => {
            match mode {
                Mode::ToInternal => {
                    let sf: StandardFloat = num.into();
                    let llf: LowLatencyFloat = sf.into();
                    *result = Some(llf.representation());
                },
                Mode::ToIEEE754 => {
                    let llf: LowLatencyFloat = num.into();
                    let sf: StandardFloat = llf.into();
                    *result = Some(sf.representation());
                },
            }
        },
        Err(e) => {
            *error = Some(format!("Nieprawidłowe wejście: {}", e));
            *result = None;
        }
    }
}


impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Konwerter kodu wewnętrznego/IEEE 754");
            ui.horizontal(|ui| {
                let name_label = ui.label("Bajty (");
                
                let format = input_format(&self.input);

                if format == InputFormat::Binary {
                    ui.label(egui::RichText::new("bin").color(egui::Color32::WHITE));
                } else {
                    ui.label("bin");
                }

                ui.label("/");

                if format == InputFormat::Hexadecimal {
                    ui.label(egui::RichText::new("hex").color(egui::Color32::WHITE));
                } else {
                    ui.label("hex");
                }

                ui.label("):");

                ui.text_edit_singleline(&mut self.input)
                    .labelled_by(name_label.id);
            });

            if ui.button("Oblicz").clicked() {
                parse_input(&self.input, &self.mode, &mut self.result, &mut self.error);

            }

            ui.separator();

            let to_ieee_radio = ui.radio_value(&mut self.mode, Mode::ToIEEE754, "Kod wewnętrzny => IEEE 754");
            let to_internal_radio = ui.radio_value(&mut self.mode, Mode::ToInternal, "IEEE 754 => Kod wewnętrzny");

            if to_ieee_radio.clicked() {
                self.input = TO_IEEE754_DEFAULT.to_owned();
            }

            if to_internal_radio.clicked() {
                self.input = TO_INTERNAL_DEFAULT.to_owned();
            }

            ui.separator();

            let result = self.result.clone();

            ui.horizontal(|ui| {
                ui.label("Wynik (bin): ");

                if let Some(result) = &result {
                    ui.label(format!("{:032b}", result));
                }
            });

            ui.horizontal(|ui| {
                ui.label("Wynik (hex): ");

                if let Some(result) = &result {
                    ui.label(format!("{:08X}", result));
                }
            });

            if let Some(error) = &self.error {
                ui.label(egui::RichText::new(error).color(egui::Color32::RED));
            }
        });
    }
}


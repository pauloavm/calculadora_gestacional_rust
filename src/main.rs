use chrono::{Datelike, NaiveDate, Utc};
use eframe::egui;
use std::str::FromStr;

struct ChronologicalAge {
    years: i32,
    months: i32,
    days: i32,
    total_weeks: i64,
    total_months: i64,
}

struct CorrectedAge {
    years: i32,
    months: i32,
    days: i32,
    weeks: i64,
    days_in_week: i64,
    total_months: i64,
}

struct AgeCalculatorApp {
    birth_date_str: String,
    gestational_weeks_str: String,
    gestational_days_str: String,
    result_text: Option<String>,
    error_message: Option<String>,
    clipboard: Option<arboard::Clipboard>,
}

impl Default for AgeCalculatorApp {
    fn default() -> Self {
        Self {
            birth_date_str: String::new(),
            gestational_weeks_str: String::new(),
            gestational_days_str: String::new(),
            result_text: None,
            error_message: None,
            clipboard: arboard::Clipboard::new().ok(),
        }
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([440.0, 360.0])
            .with_resizable(false),
        ..Default::default()
    };

    eframe::run_native(
        "Calculadora de Idade Gestacional do Bebê",
        options,
        Box::new(|_cc| Box::<AgeCalculatorApp>::default()),
    )
}

impl AgeCalculatorApp {
    fn calculate(&mut self) {
        self.result_text = None;
        self.error_message = None;

        let birthdate = match NaiveDate::parse_from_str(&self.birth_date_str, "%d/%m/%Y") {
            Ok(date) => date,
            Err(_) => {
                self.error_message = Some("Formato de data inválido. Use DD/MM/AAAA.".to_string());
                return;
            }
        };
        let gestational_weeks = match i32::from_str(&self.gestational_weeks_str) {
            Ok(val) => val,
            Err(_) => {
                self.error_message = Some("Idade gestacional deve ser um número.".to_string());
                return;
            }
        };
        let gestational_days = match i32::from_str(&self.gestational_days_str) {
            Ok(val) => val,
            Err(_) => {
                self.error_message =
                    Some("Dias na semana de nascimento devem ser um número.".to_string());
                return;
            }
        };

        let today = Utc::now().date_naive();
        let chronological_age = calculate_chronological_age(birthdate, today);
        let corrected_age =
            calculate_corrected_age(birthdate, today, gestational_weeks, gestational_days);

        self.result_text = Some(format!(
            "Idade Cronológica: {} semanas ({} meses)\nIdade Corrigida: {} semanas ({} meses) e {} dias\nIdade Corrigida (Anos): {} anos, {} meses e {} dias",
            chronological_age.total_weeks,
            chronological_age.total_months,
            corrected_age.weeks,
            corrected_age.total_months,
            corrected_age.days_in_week,
            corrected_age.years,
            corrected_age.months,
            corrected_age.days
        ));
    }
}

impl eframe::App for AgeCalculatorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::bottom("footer_panel").show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(5.0);
                ui.hyperlink_to(
                    "Desenvolvido em Rust por Paulo A V Munhoz",
                    "https://www.linkedin.com/in/paulomunhoz/",
                );
                ui.add_space(5.0);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(10.0);
                ui.heading("Calculadora de Idade Gestacional do Bebê");
            });
            ui.add_space(15.0);

            let birth_date_id = ui.id().with("birth_date_input");
            let weeks_id = ui.id().with("weeks_input");
            let days_id = ui.id().with("days_input");
            let mut birth_date_response = None;
            let mut weeks_response = None;
            let mut days_response = None;

            ui.vertical_centered(|ui| {
                egui::Grid::new("input_grid")
                    .num_columns(2)
                    .spacing([10.0, 12.0])
                    .show(ui, |ui| {
                        ui.label("Data de Nascimento (DD/MM/AAAA):");
                        birth_date_response = Some(ui.add(
                            egui::TextEdit::singleline(&mut self.birth_date_str).id(birth_date_id),
                        ));
                        ui.end_row();

                        ui.label("Idade Gestacional (semanas):");
                        weeks_response = Some(
                            ui.add(
                                egui::TextEdit::singleline(&mut self.gestational_weeks_str)
                                    .id(weeks_id),
                            ),
                        );
                        ui.end_row();

                        ui.label("Dias na Semana de Nascimento:");
                        days_response = Some(ui.add(
                            egui::TextEdit::singleline(&mut self.gestational_days_str).id(days_id),
                        ));
                        ui.end_row();
                    });
            });

            if birth_date_response.unwrap().lost_focus()
                && ctx.input(|i| i.key_pressed(egui::Key::Enter))
            {
                ctx.memory_mut(|m| m.request_focus(weeks_id));
            }
            if weeks_response.unwrap().lost_focus()
                && ctx.input(|i| i.key_pressed(egui::Key::Enter))
            {
                ctx.memory_mut(|m| m.request_focus(days_id));
            }
            if days_response.unwrap().lost_focus() && ctx.input(|i| i.key_pressed(egui::Key::Enter))
            {
                self.calculate();
            }

            ui.add_space(15.0);

            ui.vertical_centered(|ui| {
                ui.horizontal(|ui| {
                    let button_width = 100.0;
                    let spacing = ui.spacing().item_spacing.x;
                    let total_width = (button_width * 2.0) + spacing;
                    let left_space = (ui.available_width() - total_width).max(0.0) / 2.0;
                    ui.add_space(left_space);

                    if ui
                        .add_sized([button_width, 30.0], egui::Button::new("Calcular"))
                        .clicked()
                    {
                        self.calculate();
                    }
                    if ui
                        .add_sized([button_width, 30.0], egui::Button::new("Limpar"))
                        .clicked()
                    {
                        self.birth_date_str.clear();
                        self.gestational_weeks_str.clear();
                        self.gestational_days_str.clear();
                        self.result_text = None;
                        self.error_message = None;
                    }
                });
            });

            ui.add_space(15.0);

            ui.vertical_centered(|ui| {
                if let Some(error) = &self.error_message {
                    ui.colored_label(egui::Color32::RED, error);
                }
                if let Some(result) = &self.result_text {
                    let mut result_clone = result.clone();
                    ui.text_edit_multiline(&mut result_clone).enabled = false;
                    ui.add_space(10.0);
                    if ui
                        .add_sized([150.0, 30.0], egui::Button::new("Copiar Resultado"))
                        .clicked()
                    {
                        if let Some(clipboard) = &mut self.clipboard {
                            if let Err(e) = clipboard.set_text(result.clone()) {
                                self.error_message = Some(format!("Falha ao copiar: {}", e));
                            }
                        } else {
                            self.error_message =
                                Some("Área de transferência não disponível.".to_string());
                        }
                    }
                }
            });
        });
    }
}

fn calculate_chronological_age(birthdate: NaiveDate, today: NaiveDate) -> ChronologicalAge {
    let mut years = today.year() - birthdate.year();
    let mut months = today.month() as i32 - birthdate.month() as i32;
    let mut days = today.day() as i32 - birthdate.day() as i32;

    if days < 0 {
        months -= 1;
        let prev_month = today.with_day(1).unwrap() - chrono::Duration::days(1);
        days += prev_month.day() as i32;
    }
    if months < 0 {
        years -= 1;
        months += 12;
    }

    let total_days = today.signed_duration_since(birthdate).num_days();
    let total_weeks = total_days / 7;
    let total_months = (total_days as f64 / 30.4375).floor() as i64;

    ChronologicalAge {
        years,
        months,
        days,
        total_weeks,
        total_months,
    }
}

fn calculate_corrected_age(
    birthdate: NaiveDate,
    today: NaiveDate,
    gestational_weeks: i32,
    gestational_days: i32,
) -> CorrectedAge {
    let total_gestational_days = gestational_weeks * 7 + gestational_days;
    let full_term_days = 40 * 7;
    let prematurity_days = full_term_days - total_gestational_days;

    if prematurity_days <= 0 {
        let chronological = calculate_chronological_age(birthdate, today);
        let total_days = today.signed_duration_since(birthdate).num_days();
        return CorrectedAge {
            years: chronological.years,
            months: chronological.months,
            days: chronological.days,
            weeks: chronological.total_weeks,
            days_in_week: total_days % 7,
            total_months: chronological.total_months,
        };
    }

    let corrected_birthdate = birthdate + chrono::Duration::days(prematurity_days as i64);
    let corrected_age_as_chrono = calculate_chronological_age(corrected_birthdate, today);
    let corrected_total_days = today
        .signed_duration_since(corrected_birthdate)
        .num_days()
        .max(0);
    let corrected_weeks = corrected_total_days / 7;
    let corrected_days_in_week = corrected_total_days % 7;
    let corrected_total_months = (corrected_total_days as f64 / 30.4375).floor() as i64;

    CorrectedAge {
        years: corrected_age_as_chrono.years,
        months: corrected_age_as_chrono.months,
        days: corrected_age_as_chrono.days,
        weeks: corrected_weeks,
        days_in_week: corrected_days_in_week,
        total_months: corrected_total_months,
    }
}

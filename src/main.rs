
use chrono::{Datelike, NaiveDate, Utc};
use eframe::egui;
use std::str::FromStr;

// ----------------------------------------------------
// Estruturas de Dados para os Resultados (sem alteração)
// ----------------------------------------------------
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

// ----------------------------------------------------
// Estrutura Principal da Aplicação GUI
// ----------------------------------------------------
struct AgeCalculatorApp {
    // Campos para armazenar a entrada do usuário como texto
    birth_date_str: String,
    gestational_weeks_str: String,
    gestational_days_str: String,
    
    // Armazena o resultado formatado para exibição e cópia
    result_text: Option<String>,
    
    // Armazena mensagens de erro para o usuário
    error_message: Option<String>,

    // Acesso à área de transferência do sistema
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
            // Inicializa a área de transferência. Pode falhar, por isso usamos Option.
            clipboard: arboard::Clipboard::new().ok(),
        }
    }
}

// ----------------------------------------------------
// Função Principal - Inicializa a GUI
// ----------------------------------------------------
fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([480.0, 360.0]), // Tamanho da janela
        ..Default::default()
    };

    eframe::run_native(
        "Calculadora de Idade Corrigida",
        options,
        Box::new(|_cc| Box::<AgeCalculatorApp>::default()),
    )
}

// ----------------------------------------------------
// Implementação da Lógica da Interface Gráfica
// ----------------------------------------------------
impl eframe::App for AgeCalculatorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Calculadora de Idade Corrigida do Bebê");
            ui.add_space(10.0);

            // --- Seção de Entradas ---
            egui::Grid::new("input_grid")
                .num_columns(2)
                .spacing([10.0, 8.0])
                .show(ui, |ui| {
                    ui.label("Data de Nascimento (DD-MM-AAAA):");
                    ui.text_edit_singleline(&mut self.birth_date_str);
                    ui.end_row();

                    ui.label("Idade Gestacional (semanas):");
                    ui.text_edit_singleline(&mut self.gestational_weeks_str);
                    ui.end_row();

                    ui.label("Dias Adicionais:");
                    ui.text_edit_singleline(&mut self.gestational_days_str);
                    ui.end_row();
                });

            ui.add_space(10.0);

            // --- Botão de Calcular ---
            if ui.button("Calcular").clicked() {
                // Limpa resultados e erros anteriores
                self.result_text = None;
                self.error_message = None;

                // Validação e parsing das entradas
                let birthdate = match NaiveDate::parse_from_str(&self.birth_date_str, "%d-%m-%Y") {
                    Ok(date) => date,
                    Err(_) => {
                        self.error_message = Some("Formato de data inválido. Use DD-MM-AAAA.".to_string());
                        return;
                    }
                };
                let gestational_weeks = match i32::from_str(&self.gestational_weeks_str) {
                     Ok(val) => val,
                     Err(_) => {
                        self.error_message = Some("Idade gestacional em semanas deve ser um número.".to_string());
                        return;
                     }
                };
                 let gestational_days = match i32::from_str(&self.gestational_days_str) {
                     Ok(val) => val,
                     Err(_) => {
                        self.error_message = Some("Dias adicionais devem ser um número.".to_string());
                        return;
                     }
                };

                // Se todas as entradas são válidas, realiza os cálculos
                let today = Utc::now().date_naive();
                let chronological_age = calculate_chronological_age(birthdate, today);
                let corrected_age = calculate_corrected_age(birthdate, today, gestational_weeks, gestational_days);

                // Formata o resultado para exibição
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

            ui.add_space(15.0);

            // --- Seção de Resultados ---
            if let Some(error) = &self.error_message {
                ui.colored_label(egui::Color32::RED, error);
            }

            if let Some(result) = &self.result_text {
                ui.label("Resultados:");
                // Usa uma caixa de texto de múltiplas linhas para exibir os resultados
                ui.text_edit_multiline(&mut result.clone()).enabled = false;
                
                ui.add_space(5.0);

                // --- Botão de Copiar ---
                if ui.button("Copiar Resultados").clicked() {
                    if let Some(clipboard) = &mut self.clipboard {
                        if let Err(e) = clipboard.set_text(result.clone()) {
                           self.error_message = Some(format!("Falha ao copiar: {}", e));
                        }
                    } else {
                        self.error_message = Some("Área de transferência não disponível.".to_string());
                    }
                }
            }
            
            // --- Rodapé ---
            ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                ui.add_space(5.0);
                ui.horizontal(|ui| {
                    ui.label("Desenvolvido em RUST por Paulo A. V. Munhoz -");
                    ui.hyperlink_to("Perfil no LinkedIn", "https://www.linkedin.com/in/paulomunhoz/");
                });
            });
        });
    }
}


// ----------------------------------------------------
// Lógica de Cálculo (sem alteração da versão anterior)
// ----------------------------------------------------
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

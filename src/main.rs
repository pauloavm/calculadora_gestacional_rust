// MUDANÇA: `Application` (o trait) é importado diretamente. `application` não é mais necessário.
use chrono::{Datelike, NaiveDate, Utc};
use iced::{
    widget::{button, column, container, row, text, text_input},
    window, Alignment, Application, Command, Element, Length, Settings, Theme,
};

// ... (o enum Message e a struct BabyAgeCalculator continuam iguais)
#[derive(Debug, Clone)]
enum Message {
    BirthdateChanged(String),
    GestationalWeeksChanged(String),
    GestationalDaysChanged(String),
    CalculateButtonPressed,
    ClearButtonPressed,
    CopyButtonPressed,
}

struct BabyAgeCalculator {
    birthdate_input: String,
    gestational_weeks_input: String,
    gestational_days_input: String,
    result_text: String,
}

// ----------------------------------------------------
// Implementação da Lógica Principal
// ----------------------------------------------------
// MUDANÇA: O trait é apenas `Application`, não `application::Application`
impl Application for BabyAgeCalculator {
    // O tipo `Executor` foi removido em versões recentes, o que está correto.
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Self::Message>) {
        (
            Self {
                birthdate_input: String::new(),
                gestational_weeks_input: String::new(),
                gestational_days_input: String::new(),
                result_text: String::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Calculadora de Idade do Bebê")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::BirthdateChanged(value) => {
                self.birthdate_input = value;
            }
            Message::GestationalWeeksChanged(value) => {
                self.gestational_weeks_input = value;
            }
            Message::GestationalDaysChanged(value) => {
                self.gestational_days_input = value;
            }
            Message::CalculateButtonPressed => {
                self.calculate_age();
            }
            Message::ClearButtonPressed => {
                self.birthdate_input.clear();
                self.gestational_weeks_input.clear();
                self.gestational_days_input.clear();
                self.result_text.clear();
            }
            Message::CopyButtonPressed => {
                if !self.result_text.is_empty() && !self.result_text.starts_with("Por favor") {
                    // MUDANÇA: O módulo clipboard está em `iced::clipboard`, não `iced::widget::clipboard`
                    return iced::clipboard::write(self.result_text.clone());
                }
            }
        }
        Command::none()
    }

    // A função view() não precisa de mudanças desta vez
    fn view(&self) -> Element<Self::Message> {
        let title = text("Calculadora de Idade do Bebê").size(24);

        let birthdate_label = text("Data de Nascimento (DD-MM-AAAA):");
        let birthdate_entry = text_input("DD-MM-AAAA", &self.birthdate_input)
            .on_input(Message::BirthdateChanged)
            .width(Length::Fixed(200.0));

        let gestational_weeks_label = text("Idade Gestacional (semanas):");
        let gestational_weeks_entry = text_input("Semanas", &self.gestational_weeks_input)
            .on_input(Message::GestationalWeeksChanged)
            .width(Length::Fixed(200.0));

        let gestational_days_label = text("Dias na Semana de Nascimento:");
        let gestational_days_entry = text_input("Dias", &self.gestational_days_input)
            .on_input(Message::GestationalDaysChanged)
            .width(Length::Fixed(200.0));

        let calculate_button = button(text("Calcular")).on_press(Message::CalculateButtonPressed);
        let clear_button = button(text("Limpar")).on_press(Message::ClearButtonPressed);
        let copy_button = button(text("Copiar Resultado")).on_press(Message::CopyButtonPressed);

        let result_label = text(&self.result_text).size(16);

        let button_row = row![calculate_button, clear_button].spacing(10);

        let content = column![
            title,
            birthdate_label,
            birthdate_entry,
            gestational_weeks_label,
            gestational_weeks_entry,
            gestational_days_label,
            gestational_days_entry,
            button_row,
            result_label,
            copy_button,
        ]
        .spacing(10)
        .align_items(Alignment::Center);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(iced::alignment::Horizontal::Center)
            .align_y(iced::alignment::Vertical::Center)
            .into()
    }
}

// ... (o resto do arquivo, incluindo a lógica de cálculo e a função main, continua igual)
impl BabyAgeCalculator {
    fn calculate_age(&mut self) {
        let birthdate_str = &self.birthdate_input;
        let gestational_weeks_str = &self.gestational_weeks_input;
        let gestational_days_str = &self.gestational_days_input;

        if birthdate_str.is_empty()
            || gestational_weeks_str.is_empty()
            || gestational_days_str.is_empty()
        {
            self.result_text = "Por favor, preencha todos os campos.".to_string();
            return;
        }

        match (
            NaiveDate::parse_from_str(birthdate_str, "%d-%m-%Y"),
            gestational_weeks_str.parse::<i32>(),
            gestational_days_str.parse::<i32>(),
        ) {
            (Ok(birthdate), Ok(gestational_weeks), Ok(gestational_days)) => {
                let today = Utc::now().date_naive();
                let chronological_age = self.calculate_chronological_age(birthdate, today);
                let corrected_age = self.calculate_corrected_age(
                    birthdate,
                    today,
                    gestational_weeks,
                    gestational_days,
                );
                self.display_results(&chronological_age, &corrected_age);
            }
            _ => {
                self.result_text = "Por favor, preencha todos os campos corretamente. Use o formato DD-MM-AAAA para a data.".to_string();
            }
        }
    }

    fn calculate_chronological_age(
        &self,
        birthdate: NaiveDate,
        today: NaiveDate,
    ) -> ChronologicalAge {
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

        ChronologicalAge {
            years: years,
            months: months,
            days: days,
            total_days: total_days,
        }
    }

    fn calculate_corrected_age(
        &self,
        birthdate: NaiveDate,
        today: NaiveDate,
        gestational_weeks: i32,
        gestational_days: i32,
    ) -> CorrectedAge {
        let prematurity_days = (40 * 7) - (gestational_weeks * 7 + gestational_days);

        if prematurity_days <= 0 {
            let chronological = self.calculate_chronological_age(birthdate, today);
            return CorrectedAge {
                years: chronological.years,
                months: chronological.months,
                days: chronological.days,
            };
        }

        let corrected_birthdate = birthdate + chrono::Duration::days(prematurity_days as i64);
        let corrected_age = self.calculate_chronological_age(corrected_birthdate, today);

        CorrectedAge {
            years: corrected_age.years,
            months: corrected_age.months,
            days: corrected_age.days,
        }
    }

    fn display_results(
        &mut self,
        chronological_age: &ChronologicalAge,
        corrected_age: &CorrectedAge,
    ) {
        let result = format!(
            "Idade Cronológica: {} anos, {} meses e {} dias (Total: {} dias)\n\
             Idade Corrigida: {} anos, {} meses e {} dias",
            chronological_age.years,
            chronological_age.months,
            chronological_age.days,
            chronological_age.total_days,
            corrected_age.years,
            corrected_age.months,
            corrected_age.days
        );
        self.result_text = result;
    }
}
struct ChronologicalAge {
    years: i32,
    months: i32,
    days: i32,
    total_days: i64,
}

struct CorrectedAge {
    years: i32,
    months: i32,
    days: i32,
}

fn main() -> iced::Result {
    BabyAgeCalculator::run(Settings {
        size: (450.0, 450.0),
        window: window::Settings {
            ..Default::default()
        },
        ..Default::default()
    })
}

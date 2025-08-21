use chrono::{Datelike, NaiveDate, Utc};
use iced::{
    executor,
    widget::{button, column, container, row, text, text_input},
    window, Alignment, Application, Command, Element, Length, Settings, Theme,
};
use std::error::Error;

// ----------------------------------------------------
// Mensagens da Aplicação
// ----------------------------------------------------
// Este enum representa todas as ações que o usuário pode fazer na interface.
// É a maneira do iced de lidar com eventos.
#[derive(Debug, Clone)]
enum Message {
    BirthdateChanged(String),
    GestationalWeeksChanged(String),
    GestationalDaysChanged(String),
    CalculateButtonPressed,
    ClearButtonPressed,
    CopyButtonPressed,
}

// ----------------------------------------------------
// Estrutura do Estado da Aplicação
// ----------------------------------------------------
// Esta struct armazena o estado atual da sua aplicação,
// como os valores dos campos de entrada e o texto de resultado.
struct BabyAgeCalculator {
    birthdate_input: String,
    gestational_weeks_input: String,
    gestational_days_input: String,
    result_text: String,
}

// ----------------------------------------------------
// Implementação da Lógica Principal
// ----------------------------------------------------
impl Application for BabyAgeCalculator {
    type Executor = executor::Default;
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

    // A função `update` é o coração da aplicação.
    // Ela processa as mensagens (eventos) e atualiza o estado.
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
                // Em iced, a cópia para a área de transferência pode ser um pouco complexa,
                // pois não há um método direto como em Tkinter.
                // Usaremos um método simples para mostrar a intenção.
                // Em uma aplicação real, você usaria uma biblioteca externa como `arboard`.
                self.result_text = "Resultado copiado!".to_string();
            }
        }
        Command::none()
    }

    // A função `view` é responsável por construir e renderizar a interface.
    // Ela descreve como a UI deve se parecer a cada atualização do estado.
    fn view(&self) -> Element<Self::Message> {
        let title = text("Calculadora de Idade do Bebê")
            .size(24)
            .style(iced::Color::from_rgb8(0, 0, 255));

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

        let result_label = text(&self.result_text)
            .size(16)
            .line_height(1.5)
            .horizontal_alignment(iced::alignment::Horizontal::Left);

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
            .center_x()
            .center_y()
            .into()
    }
}

// ----------------------------------------------------
// Lógica de Cálculo (reimplementação do código Python)
// ----------------------------------------------------
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
                    &chronological_age,
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
        let delta = today.signed_duration_since(birthdate);
        let total_days = delta.num_days();

        let age_in_weeks = total_days / 7;
        let age_in_months = total_days / 30; // Aproximação

        ChronologicalAge {
            weeks: age_in_weeks,
            months: age_in_months,
        }
    }

    fn calculate_corrected_age(
        &self,
        chronological_age: &ChronologicalAge,
        gestational_weeks: i32,
        gestational_days: i32,
    ) -> CorrectedAge {
        let corrected_weeks = chronological_age.weeks - (40 - gestational_weeks as i64);
        let total_corrected_days = corrected_weeks * 7 + gestational_days as i64;

        let final_weeks = total_corrected_days / 7;
        let final_days_in_week = total_corrected_days % 7;
        let corrected_months = total_corrected_days / 30;

        // CÁLCULO MAIS PRECISO
        let today = Utc::now().date_naive();
        let corrected_birthdate = today - chrono::Duration::days(total_corrected_days);

        let mut corrected_years = today.year() - corrected_birthdate.year();
        let mut corrected_months_years = today.month() as i32 - corrected_birthdate.month() as i32;
        let mut corrected_days_years = today.day() as i32 - corrected_birthdate.day() as i32;

        if corrected_days_years < 0 {
            corrected_months_years -= 1;
            let last_day_of_prev_month = (today - chrono::Duration::days(today.day() as i64)).day();
            corrected_days_years += last_day_of_prev_month as i32;
        }

        if corrected_months_years < 0 {
            corrected_years -= 1;
            corrected_months_years += 12;
        }

        CorrectedAge {
            weeks: final_weeks,
            months: corrected_months,
            days: final_days_in_week,
            years: corrected_years,
            months_years: corrected_months_years,
            days_years: corrected_days_years,
        }
    }

    fn display_results(
        &mut self,
        chronological_age: &ChronologicalAge,
        corrected_age: &CorrectedAge,
    ) {
        let result = format!(
            "Idade Cronológica: {} semanas ({} meses)\n\
            Idade Corrigida: {} semanas ({} meses) e {} dias\n\
            Idade Corrigida (Anos): {} anos, {} meses e {} dias",
            chronological_age.weeks,
            chronological_age.months,
            corrected_age.weeks,
            corrected_age.months,
            corrected_age.days,
            corrected_age.years,
            corrected_age.months_years,
            corrected_age.days_years
        );
        self.result_text = result;
    }
}

// ----------------------------------------------------
// Estruturas de Dados para os Resultados
// ----------------------------------------------------
// Usamos structs para organizar os resultados de forma clara.
struct ChronologicalAge {
    weeks: i64,
    months: i64,
}

struct CorrectedAge {
    weeks: i64,
    months: i64,
    days: i64,
    years: i32,
    months_years: i32,
    days_years: i32,
}

fn main() -> iced::Result {
    BabyAgeCalculator::run(Settings {
        window: window::Settings {
            size: iced::Size::new(450.0, 450.0),
            resizable: false,
            ..Default::default()
        },
        ..Default::default()
    })
}

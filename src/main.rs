use chrono::{Datelike, NaiveDate, Utc};
use std::io::{self, Write};
use std::str::FromStr;

// ----------------------------------------------------
// Estruturas de Dados para os Resultados (ATUALIZADAS)
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
// Função Principal
// ----------------------------------------------------
fn main() {
    println!("--- Calculadora de Idade Corrigida do Bebê ---");

    // Coleta e valida as entradas do usuário
    let birthdate = get_date_input("Digite a data de nascimento (DD-MM-AAAA): ");
    let gestational_weeks =
        get_numeric_input::<i32>("Digite a idade gestacional em semanas (ex: 38): ");
    let gestational_days =
        get_numeric_input::<i32>("Digite os dias adicionais (ex: 2 para 38s2d): ");

    // Realiza os cálculos
    let today = Utc::now().date_naive();
    let chronological_age = calculate_chronological_age(birthdate, today);
    let corrected_age =
        calculate_corrected_age(birthdate, today, gestational_weeks, gestational_days);

    // ----------------------------------------------------
    // Exibição dos Resultados (ATUALIZADA)
    // ----------------------------------------------------
    println!("\n--- Resultados ---");
    println!(
        "Idade Cronológica: {} semanas ({} meses)",
        chronological_age.total_weeks, chronological_age.total_months,
    );
    println!(
        "Idade Corrigida: {} semanas ({} meses) e {} dias",
        corrected_age.weeks, corrected_age.total_months, corrected_age.days_in_week
    );
    println!(
        "Idade Corrigida (Anos): {} anos, {} meses e {} dias",
        corrected_age.years, corrected_age.months, corrected_age.days
    );
}

// ----------------------------------------------------
// Funções Auxiliares para Entrada do Usuário (sem alteração)
// ----------------------------------------------------

/// Solicita uma data ao usuário e continua pedindo até que o formato seja válido.
fn get_date_input(prompt: &str) -> NaiveDate {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Falha ao ler a linha");

        match NaiveDate::parse_from_str(input.trim(), "%d-%m-%Y") {
            Ok(date) => return date,
            Err(_) => println!("Erro: Formato de data inválido. Por favor, use DD-MM-AAAA."),
        }
    }
}

/// Solicita um número ao usuário e continua pedindo até que a entrada seja um número válido.
fn get_numeric_input<T: FromStr>(prompt: &str) -> T {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Falha ao ler a linha");

        match input.trim().parse::<T>() {
            Ok(num) => return num,
            Err(_) => println!("Erro: Entrada inválida. Por favor, digite um número válido."),
        }
    }
}

// ----------------------------------------------------
// Lógica de Cálculo (ATUALIZADA)
// ----------------------------------------------------

fn calculate_chronological_age(birthdate: NaiveDate, today: NaiveDate) -> ChronologicalAge {
    // Cálculo de anos, meses e dias
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

    // Cálculo de dias, semanas e meses totais
    let total_days = today.signed_duration_since(birthdate).num_days();
    let total_weeks = total_days / 7;
    let total_months = (total_days as f64 / 30.4375).floor() as i64; // Média de dias no mês

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
        // Se não for prematuro, a idade corrigida é igual à cronológica
        let chronological = calculate_chronological_age(birthdate, today);
        return CorrectedAge {
            years: chronological.years,
            months: chronological.months,
            days: chronological.days,
            weeks: chronological.total_weeks,
            days_in_week: chronological.total_weeks % 7,
            total_months: chronological.total_months,
        };
    }

    let corrected_birthdate = birthdate + chrono::Duration::days(prematurity_days as i64);

    // Calcula a idade no formato "anos, meses, dias" a partir da data corrigida
    let corrected_age_as_chrono = calculate_chronological_age(corrected_birthdate, today);

    // Calcula a idade no formato "semanas e dias" a partir da data corrigida
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

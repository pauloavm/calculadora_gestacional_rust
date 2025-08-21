// NOTA: Para compilar este código, adicione as seguintes dependências ao seu arquivo `Cargo.toml`:
//
// [dependencies]
// chrono = "0.4"
// eframe = "0.27" # A biblioteca para a interface gráfica (GUI)
// arboard = "3.4" # Para funcionalidade de copiar para a área de transferência

// --- Importações de Bibliotecas (Crates) ---
// `chrono` é usado para lidar com datas e durações.
use chrono::{Datelike, NaiveDate, Utc};
// `eframe` e seu módulo `egui` são usados para criar a interface gráfica do usuário (GUI).
use eframe::egui;
// `FromStr` é um trait que permite converter uma string em outro tipo, usado aqui para os números.
use std::str::FromStr;

// ----------------------------------------------------
// Estruturas de Dados para os Resultados
// ----------------------------------------------------
// Armazena os componentes da idade cronológica (a idade real desde o nascimento).
struct ChronologicalAge {
    years: i32,
    months: i32,
    days: i32,
    total_weeks: i64,
    total_months: i64,
}

// Armazena os componentes da idade corrigida (idade ajustada pela prematuridade).
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
// Esta estrutura contém o "estado" do nosso aplicativo, ou seja,
// todos os dados que precisam ser mantidos entre os frames (atualizações da tela).
struct AgeCalculatorApp {
    // Campos de texto para armazenar a entrada do usuário.
    // Usamos String porque os campos de texto da GUI trabalham com texto.
    birth_date_str: String,
    gestational_weeks_str: String,
    gestational_days_str: String,

    // `Option` é usado porque o resultado pode ou não existir (antes do primeiro cálculo).
    // Armazena o texto do resultado já formatado para exibição.
    result_text: Option<String>,

    // Armazena mensagens de erro, que também podem ou não existir.
    error_message: Option<String>,

    // Acesso à área de transferência do sistema para a função de "copiar".
    clipboard: Option<arboard::Clipboard>,
}

// Implementa o valor padrão para a nossa estrutura de aplicativo.
// Isso é chamado quando o aplicativo é iniciado pela primeira vez.
impl Default for AgeCalculatorApp {
    fn default() -> Self {
        Self {
            // Inicializa todos os campos de texto como vazios.
            birth_date_str: String::new(),
            gestational_weeks_str: String::new(),
            gestational_days_str: String::new(),
            // Inicializa os campos opcionais como `None` (sem valor).
            result_text: None,
            error_message: None,
            // Tenta criar uma nova conexão com a área de transferência do sistema.
            // `.ok()` converte o `Result` em um `Option`, pois a operação pode falhar.
            clipboard: arboard::Clipboard::new().ok(),
        }
    }
}

// ----------------------------------------------------
// Função Principal - Ponto de Entrada do Programa
// ----------------------------------------------------
fn main() -> Result<(), eframe::Error> {
    // Configura as opções da janela nativa (tamanho, etc.).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([440.0, 360.0]), // Define o tamanho inicial da janela.
        ..Default::default()
    };

    // Inicia o aplicativo `eframe`.
    // - O primeiro argumento é o título da janela.
    // - O segundo são as opções que acabamos de definir.
    // - O terceiro é uma função que cria e retorna o estado inicial do nosso aplicativo.
    eframe::run_native(
        "Calculadora de Idade Gestacional do Bebê",
        options,
        Box::new(|_cc| Box::<AgeCalculatorApp>::default()),
    )
}

// ----------------------------------------------------
// Implementação da Lógica da Aplicação
// ----------------------------------------------------
impl AgeCalculatorApp {
    /// Esta função contém a lógica de validação e cálculo.
    /// É separada da lógica da GUI para manter o código organizado.
    fn calculate(&mut self) {
        // Limpa os resultados e erros anteriores antes de um novo cálculo.
        self.result_text = None;
        self.error_message = None;

        // Tenta converter o texto da data de nascimento para um objeto `NaiveDate`.
        let birthdate = match NaiveDate::parse_from_str(&self.birth_date_str, "%d/%m/%Y") {
            Ok(date) => date, // Se for bem-sucedido, armazena a data.
            Err(_) => {
                // Se falhar, define uma mensagem de erro e para a função.
                self.error_message = Some("Formato de data inválido. Use DD/MM/AAAA.".to_string());
                return;
            }
        };
        // Faz o mesmo para as semanas gestacionais, convertendo para um número inteiro.
        let gestational_weeks = match i32::from_str(&self.gestational_weeks_str) {
            Ok(val) => val,
            Err(_) => {
                self.error_message = Some("Idade gestacional deve ser um número.".to_string());
                return;
            }
        };
        // E para os dias gestacionais.
        let gestational_days = match i32::from_str(&self.gestational_days_str) {
            Ok(val) => val,
            Err(_) => {
                self.error_message =
                    Some("Dias na semana de nascimento devem ser um número.".to_string());
                return;
            }
        };

        // Se todas as entradas forem válidas, obtém a data atual.
        let today = Utc::now().date_naive();
        // Chama as funções de cálculo.
        let chronological_age = calculate_chronological_age(birthdate, today);
        let corrected_age =
            calculate_corrected_age(birthdate, today, gestational_weeks, gestational_days);

        // Formata os resultados em uma única string para exibição.
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

// ----------------------------------------------------
// Implementação da Lógica da Interface Gráfica
// ----------------------------------------------------
// O trait `eframe::App` requer uma função `update`, que é chamada a cada frame
// para desenhar a interface e lidar com a interação do usuário.
impl eframe::App for AgeCalculatorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // --- Painel do Rodapé ---
        // Cria um painel fixo na parte inferior da janela.
        egui::TopBottomPanel::bottom("footer_panel").show(ctx, |ui| {
            // Centraliza o conteúdo do painel.
            ui.vertical_centered(|ui| {
                ui.add_space(5.0); // Adiciona um pequeno espaço.
                ui.hyperlink_to(
                    "Desenvolvido em Rust por Paulo A V Munhoz",
                    "https://www.linkedin.com/in/paulomunhoz/",
                );
                ui.add_space(5.0);
            });
        });

        // --- Painel Central para o conteúdo principal ---
        // O `CentralPanel` ocupa todo o espaço restante da janela.
        egui::CentralPanel::default().show(ctx, |ui| {
            // --- Título ---
            // `vertical_centered` garante que o conteúdo dentro dele seja centralizado horizontalmente.
            ui.vertical_centered(|ui| {
                ui.add_space(10.0);
                ui.heading("Calculadora de Idade Gestacional do Bebê");
            });
            ui.add_space(15.0);

            // --- Seção de Entradas ---
            // Cria IDs únicos para cada campo de texto. Isso é necessário para controlar o foco (qual campo está selecionado).
            let birth_date_id = ui.id().with("birth_date_input");
            let weeks_id = ui.id().with("weeks_input");
            let days_id = ui.id().with("days_input");
            // Variáveis para armazenar a "resposta" (estado) de cada campo de texto após ser desenhado.
            let mut birth_date_response = None;
            let mut weeks_response = None;
            let mut days_response = None;

            ui.vertical_centered(|ui| {
                // `Grid` é usado para alinhar rótulos e campos de texto em colunas.
                egui::Grid::new("input_grid")
                    .num_columns(2)
                    .spacing([10.0, 12.0])
                    .show(ui, |ui| {
                        ui.label("Data de Nascimento (DD/MM/AAAA):");
                        // Adiciona o campo de texto e armazena sua resposta.
                        birth_date_response = Some(ui.add(
                            egui::TextEdit::singleline(&mut self.birth_date_str).id(birth_date_id),
                        ));
                        ui.end_row(); // Termina a linha da grade.

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

            // Lógica para trocar de campo com a tecla Enter.
            // Verifica se o campo perdeu o foco E se a tecla Enter foi pressionada.
            if birth_date_response.unwrap().lost_focus()
                && ctx.input(|i| i.key_pressed(egui::Key::Enter))
            {
                // Pede para a memória da GUI focar no próximo campo (semanas).
                ctx.memory_mut(|m| m.request_focus(weeks_id));
            }
            if weeks_response.unwrap().lost_focus()
                && ctx.input(|i| i.key_pressed(egui::Key::Enter))
            {
                ctx.memory_mut(|m| m.request_focus(days_id));
            }
            // Se Enter for pressionado no último campo, aciona o cálculo.
            if days_response.unwrap().lost_focus() && ctx.input(|i| i.key_pressed(egui::Key::Enter))
            {
                self.calculate();
            }

            ui.add_space(15.0);

            // --- Botões de Ação (Calcular e Limpar) ---
            ui.vertical_centered(|ui| {
                // `horizontal` organiza os widgets lado a lado.
                ui.horizontal(|ui| {
                    // INÍCIO DA CORREÇÃO: Calcula a largura total dos botões (incluindo espaçamento entre eles) e adiciona espaço à esquerda para centralizar o grupo de botões na janela.
                    let button_width = 100.0;
                    let spacing = ui.spacing().item_spacing.x;
                    let total_width = (button_width * 2.0) + spacing;
                    let left_space = (ui.available_width() - total_width).max(0.0) / 2.0;
                    ui.add_space(left_space);
                    // FIM DA CORREÇÃO

                    // Adiciona o botão "Calcular" com um tamanho fixo.
                    if ui
                        .add_sized([button_width, 30.0], egui::Button::new("Calcular"))
                        .clicked()
                    {
                        self.calculate(); // Chama a função de cálculo se clicado.
                    }
                    // Adiciona o botão "Limpar".
                    if ui
                        .add_sized([button_width, 30.0], egui::Button::new("Limpar"))
                        .clicked()
                    {
                        // Limpa todos os campos de entrada e resultados.
                        self.birth_date_str.clear();
                        self.gestational_weeks_str.clear();
                        self.gestational_days_str.clear();
                        self.result_text = None;
                        self.error_message = None;
                    }
                });
            });

            ui.add_space(15.0);

            // --- Seção de Resultados ---
            ui.vertical_centered(|ui| {
                // Se houver uma mensagem de erro, exibe-a em vermelho.
                if let Some(error) = &self.error_message {
                    ui.colored_label(egui::Color32::RED, error);
                }
                // Se houver um resultado, exibe-o.
                if let Some(result) = &self.result_text {
                    let mut result_clone = result.clone();
                    // `text_edit_multiline` é usado para exibir o texto, mas desabilitado para edição.
                    ui.text_edit_multiline(&mut result_clone).enabled = false;
                    ui.add_space(10.0);
                    // Adiciona o botão "Copiar Resultado".
                    if ui
                        .add_sized([150.0, 30.0], egui::Button::new("Copiar Resultado"))
                        .clicked()
                    {
                        // Verifica se a área de transferência está disponível.
                        if let Some(clipboard) = &mut self.clipboard {
                            // Tenta copiar o texto do resultado.
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

// ----------------------------------------------------
// Lógica de Cálculo (sem alteração da versão anterior)
// ----------------------------------------------------
fn calculate_chronological_age(birthdate: NaiveDate, today: NaiveDate) -> ChronologicalAge {
    // Cálculo simples de anos, meses e dias.
    let mut years = today.year() - birthdate.year();
    let mut months = today.month() as i32 - birthdate.month() as i32;
    let mut days = today.day() as i32 - birthdate.day() as i32;

    // Ajustes para casos em que o dia ou mês de hoje é menor que o de nascimento.
    if days < 0 {
        months -= 1;
        // Pega o último dia do mês anterior para somar aos dias.
        let prev_month = today.with_day(1).unwrap() - chrono::Duration::days(1);
        days += prev_month.day() as i32;
    }
    if months < 0 {
        years -= 1;
        months += 12; // "Empresta" um ano.
    }

    // Calcula os totais usando a duração em dias fornecida por `chrono`.
    let total_days = today.signed_duration_since(birthdate).num_days();
    let total_weeks = total_days / 7;
    let total_months = (total_days as f64 / 30.4375).floor() as i64; // Usa uma média de dias por mês.

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
    // Calcula o total de dias de gestação.
    let total_gestational_days = gestational_weeks * 7 + gestational_days;
    // Define o termo completo como 40 semanas (280 dias).
    let full_term_days = 40 * 7;
    // Calcula quantos dias o bebê nasceu "adiantado".
    let prematurity_days = full_term_days - total_gestational_days;

    // Se não for prematuro (ou nasceu depois do termo), a idade corrigida é igual à cronológica.
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

    // "Empurra" a data de nascimento para a frente pelos dias de prematuridade.
    // Esta é a data de nascimento "corrigida".
    let corrected_birthdate = birthdate + chrono::Duration::days(prematurity_days as i64);
    // Agora, calcula a idade cronológica a partir desta nova data.
    let corrected_age_as_chrono = calculate_chronological_age(corrected_birthdate, today);
    let corrected_total_days = today
        .signed_duration_since(corrected_birthdate)
        .num_days()
        .max(0); // Garante que não seja negativo.
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

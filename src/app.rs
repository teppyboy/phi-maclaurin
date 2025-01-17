use std::hash::{BuildHasher, Hasher, RandomState};

use egui::epaint::text::{FontInsert, InsertFontFamily};

#[derive(PartialEq, Clone, serde::Deserialize, serde::Serialize)]
struct Question {
    question: String,
    choices: Vec<String>,
    answer: usize,
}

#[derive(PartialEq, Clone, serde::Deserialize, serde::Serialize)]
enum QuestionChoice {
    First = 0,
    Second = 1,
    Third = 2,
    Fourth = 3,
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    randomized_question: bool,
    randomized_answers: bool,
    question_amount: i32,
    from_question: i32,
    to_question: i32,
    test_question_choice: QuestionChoice,
    my_question_choices: Vec<QuestionChoice>,
    my_score: i32,
    loaded_questions: Vec<Question>,
    all_questions: Vec<Question>,
    begin_quiz: bool,
    show_answer: bool,
    label: String,

    #[serde(skip)] // This how you opt-out of serialization of a field
    value: f32,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            randomized_question: false,
            randomized_answers: false,
            question_amount: 30,
            from_question: 1,
            to_question: 1100,
            test_question_choice: QuestionChoice::First,
            my_question_choices: vec![],
            my_score: 0,
            loaded_questions: vec![],
            all_questions: serde_json::from_str(include_str!("../parser/questions.json")).unwrap(),
            show_answer: false,
            begin_quiz: false,
            label: "Hello World!".to_owned(),
            value: 2.7,
        }
    }
}

// Demonstrates how to add a font to the existing ones
fn add_font(ctx: &egui::Context) {
    ctx.add_font(FontInsert::new(
        "my_font",
        egui::FontData::from_static(include_bytes!("../assets/font.ttf")),
        vec![
            InsertFontFamily {
                family: egui::FontFamily::Proportional,
                priority: egui::epaint::text::FontPriority::Highest,
            },
            InsertFontFamily {
                family: egui::FontFamily::Monospace,
                priority: egui::epaint::text::FontPriority::Lowest,
            },
        ],
    ));
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        add_font(&cc.egui_ctx);
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_theme_preference_buttons(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("Triết học Mac - Lenin");
            ui.checkbox(&mut self.randomized_question, "Tráo câu hỏi");
            ui.checkbox(&mut self.randomized_answers, "Tráo đáp án");
            ui.horizontal(|ui| {
                ui.label("Số lượng câu hỏi ");
                ui.add(egui::Slider::new(
                    &mut self.question_amount,
                    1..=(self.all_questions.len() as i32),
                ));
            });
            ui.separator();
            ui.label("Cài dặt dưới đây chỉ được áp dụng khi không tráo câu hỏi và tráo đáp án");
            ui.horizontal(|ui| {
                ui.label("Từ câu ");
                ui.add(egui::Slider::new(
                    &mut self.from_question,
                    1..=(self.all_questions.len() as i32),
                ));
                ui.label("đến câu ");
                ui.add(egui::Slider::new(
                    &mut self.to_question,
                    1..=(self.all_questions.len() as i32),
                ));
            });
            if ui
                .button("Bắt đầu")
                .on_hover_text("Bắt đầu bài kiểm tra")
                .clicked()
            {
                self.loaded_questions = vec![];
                self.my_question_choices = vec![];
                self.my_score = 0;
                if self.randomized_question {
                    for _ in 0..self.question_amount {
                        let question_num = RandomState::new().build_hasher().finish()
                            % self.all_questions.len() as u64;
                        let mut question = self.all_questions[question_num as usize].clone();
                        if self.randomized_answers {
                            let mut choices = question.choices.clone();
                            let swap_target: usize = (RandomState::new().build_hasher().finish()
                                % choices.len() as u64)
                                as usize;
                            if swap_target != question.answer {
                                choices.swap(question.answer, swap_target);
                                question.answer = swap_target;
                            }
                            let mut all_choices: Vec<usize> = Vec::new();
                            for i in 0..choices.len() - 1 {
                                all_choices.push(i);
                            }
                            let others = all_choices
                                .iter()
                                .filter(|x| **x != question.answer)
                                .collect::<Vec<&usize>>();
                            for i in others {
                                let swap_target: usize =
                                    (RandomState::new().build_hasher().finish()
                                        % (choices.len() - 1) as u64)
                                        as usize;
                                if swap_target != question.answer {
                                    choices.swap(*i, swap_target);
                                }
                            }
                            question.choices = choices;
                        }
                        self.loaded_questions.push(question);
                        self.my_question_choices.push(QuestionChoice::First);
                    }
                } else {
                    for i in self.from_question..self.to_question {
                        self.loaded_questions
                            .push(self.all_questions[i as usize].clone());
                        self.my_question_choices.push(QuestionChoice::First);
                    }
                }
                self.begin_quiz = true;
                self.show_answer = false;
            };
            if self.begin_quiz {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.separator();
                    for (i, question) in &mut self.loaded_questions.iter_mut().enumerate() {
                        if self.show_answer {
                            let mut additional_text = "";
                            if self.my_question_choices[i].clone() as usize
                                != question.answer as usize
                            {
                                additional_text = "[TRẢ LỜI SAI] ";
                            }
                            ui.label(format!("{}{}", additional_text, &question.question));
                        } else {
                            ui.label(&question.question);
                        }
                        ui.radio_value(
                            &mut self.my_question_choices[i],
                            QuestionChoice::First,
                            &question.choices[0],
                        );
                        ui.radio_value(
                            &mut self.my_question_choices[i],
                            QuestionChoice::Second,
                            &question.choices[1],
                        );
                        if question.choices.len() > 2 {
                            ui.radio_value(
                                &mut self.my_question_choices[i],
                                QuestionChoice::Third,
                                &question.choices[2],
                            );
                        }
                        if question.choices.len() > 3 {
                            ui.radio_value(
                                &mut self.my_question_choices[i],
                                QuestionChoice::Fourth,
                                &question.choices[3],
                            );
                        }
                        if self.show_answer {
                            ui.label(format!("Đáp án: {}", question.choices[question.answer]));
                        }
                        ui.separator();
                    }
                    if self.show_answer {
                        ui.label(format!(
                            "Số điểm của bạn: {}/{}",
                            self.my_score,
                            self.loaded_questions.len()
                        ));
                    }
                    if ui
                        .button("Xem đáp án")
                        .on_hover_text("Xem đáp án")
                        .clicked()
                    {
                        self.my_score = 0;
                        for (i, question) in &mut self.loaded_questions.iter().enumerate() {
                            if self.my_question_choices[i].clone() as usize
                                == question.answer as usize
                            {
                                self.my_score += 1;
                            }
                        }
                        self.show_answer = true;
                        let mut scroll_delta = egui::Vec2::ZERO;
                        scroll_delta.y = 99999.9;
                        ui.scroll_with_delta(scroll_delta);
                    };
                });
            }
            ui.with_layout(egui::Layout::bottom_up(egui::Align::RIGHT), |ui| {
                powered_by_egui_and_eframe(ui);
                egui::warn_if_debug_build(ui);
            });
        });
    }
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.hyperlink_to("GitHub", "https://github.com/teppyboy/phi-maclaurin");
        ui.spacing_mut().item_spacing.x = 0.0;
    });
    ui.horizontal(|ui| {
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(" and ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label("Powered by ");
        ui.spacing_mut().item_spacing.x = 0.0;
    });
}

use egui::{remap, vec2, Color32, RichText, Stroke};
use egui_plot::{Line, LineStyle, Plot, PlotPoints, Points};

use self::calculator::{Calculator, Coordinates};

//use self::calculator::math_eng_init;

mod calculator;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct TemplateApp {
    dot_color: Color32,
    dot_size: f32,
    calc_cycles: f32,
    calc_step: f32,
    user_equation: String,
    #[serde(skip)]
    math_output: String,
    #[serde(skip)]
    equation_vec: Option<Vec<Coordinates>>,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            dot_color: Color32::WHITE,
            dot_size: 1.,
            calc_cycles: 10.,
            calc_step: 1.,
            user_equation: String::new(),
            math_output: String::new(),
            equation_vec: None,
        }
    }
}

impl TemplateApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for TemplateApp {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        //Side main, user input
        egui::SidePanel::left("side_main")
            .show_separator_line(false)
            .resizable(false)
            .exact_width(ctx.available_rect().width() / 5.5)
            .show(ctx, |ui| {
                ui.allocate_space(vec2(ui.available_width(), 20.));

                let temp_string_size = self.user_equation.len();

                let temp_cyc_size = self.calc_cycles;
                let temp_step_size = self.calc_step;

                ui.group(|ui| {
                    ui.label("Equation");
                    ui.text_edit_singleline(&mut self.user_equation);
                    ui.separator();
                    ui.menu_button("Help", |ui|{
                        ui.label("Need help? Click on the template equations, to experiment with them.");
                        let equations = ["1 / x", "sroot x", "croot x", "abs x", "x ^ 2"];
                        for item in equations {
                            if ui.button(item).clicked() {
                                self.user_equation = item.to_string();
                                self.calc();
                            }
                        }
                        ui.label(RichText::from("Always make sure to separate the numbers and expressions with a space!").strong())
                    });
                });

                ui.group(|ui| {
                    ui.label("Limit");
                    ui.add(
                        egui::DragValue::new(&mut self.calc_cycles)
                            .clamp_range(1.0..=f32::MAX)
                            .speed(2),
                    );
                    ui.label("Step");
                    ui.add(
                        egui::DragValue::new(&mut self.calc_step)
                            .clamp_range(0.01..=f32::MAX)
                            .speed(0.01),
                    );
                })
                .response
                .on_hover_text("Settings for the calculation");

                ui.group(|ui|{
                    ui.label("Dot size");
                    ui.add(egui::DragValue::new(&mut self.dot_size).clamp_range(0.1..=f64::MAX));
                    ui.label("Dot color");
                    ui.color_edit_button_srgba(&mut self.dot_color);
                });

                //call calculator on every change
                if temp_string_size != self.user_equation.len()
                    || temp_cyc_size != self.calc_cycles
                    || temp_step_size != self.calc_step
                {
                    self.calc();
                }
            });
        //dbg!(self.equation_vec.clone());
        //Main, display curve
        egui::CentralPanel::default().show(ctx, |ui| {
            Plot::new("main")
                .allow_scroll(true)
                .sharp_grid_lines(false)
                .allow_boxed_zoom(false)
                .show(ui, |plot_ui| {
                    if let Some(vector) = self.equation_vec.clone() {
                        for item in 0..vector.len() {
                            let line_points: PlotPoints =
                                //x                 y
                                [vector[item][0], vector[item][1]].into();
                            plot_ui.line(
                                Line::new(line_points)
                                    .stroke(Stroke::new(self.dot_size, self.dot_color)),
                            );
                        }
                    }
                });
        });
    }
}
impl TemplateApp {
    fn calc(&mut self) {
        self.equation_vec = Calculator::init(&mut Calculator {
            buf: self.user_equation.clone(),
            bounds: self.calc_cycles,
            step: self.calc_step,
            ..Default::default()
        } );
    }
}
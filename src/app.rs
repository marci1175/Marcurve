use egui::{vec2, Color32, RichText};
use egui_plot::{Plot, PlotPoints, Points};

use self::calculator::math_eng_init;

mod calculator;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct TemplateApp {
    calc_cycles: i16,
    calc_step: f32,
    user_equation: String,
    #[serde(skip)]
    math_output: String,
    #[serde(skip)]
    equation_vec: Vec<f64>,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            calc_cycles: 10,
            calc_step: 1.,
            user_equation: String::new(),
            math_output: String::new(),
            equation_vec: Vec::new(),
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

                ui.group(|ui| {
                    ui.label("Equation");
                    ui.text_edit_singleline(&mut self.user_equation);
                    ui.separator();
                    ui.label(RichText::from("Output : ").strong().size(15.));
                    ui.label(
                        RichText::from(&self.math_output)
                            .color(Color32::RED)
                            .size(15.),
                    );
                });

                let temp_cyc_size = self.calc_cycles;
                let temp_step_size = self.calc_step;

                ui.group(|ui| {
                    ui.label("Limit");
                    ui.add(egui::Slider::new(&mut self.calc_cycles, 0..=2000));
                    ui.label("Step");
                    ui.add(egui::Slider::new(&mut self.calc_step, 0.0..=2000.0));
                })
                .response
                .on_hover_text("Settings for the calculation");

                //call calculator on every change
                if temp_string_size != self.user_equation.len() || temp_cyc_size != self.calc_cycles || temp_step_size != self.calc_step  {
                    self.equation_vec = math_eng_init(self.user_equation.clone(), self.calc_step.clone(), self.calc_cycles.clone());
                }
            });
        dbg!(self.equation_vec.clone());
        //Main, display curve
        egui::CentralPanel::default().show(ctx, |ui| {
            Plot::new("main")
                .allow_scroll(true)
                .allow_boxed_zoom(false)
                .show(ui, |plot_ui| {
                    plot_ui.points(Points::new(PlotPoints::from_ys_f64(&[1.2, 2.3])).filled(true));
                });
        });
    }
}

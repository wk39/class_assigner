use egui::Layout;
use egui_extras::{Column, TableBuilder};
use egui_extras::{Size, StripBuilder};

use crate::class_room::{BuilderData, Student, StudentId};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct MainApp {
    // Example stuff:
    label: String,

    #[serde(skip)] // This how you opt-out of serialization of a field
    value: f32,

    app_mode: AppMode,

    #[serde(skip)]
    builder_data: BuilderData,

    reversed: bool,
}

impl Default for MainApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Class Assigner".to_owned(),
            value: 2.7,
            app_mode: AppMode::default(),

            builder_data: BuilderData::new_demo(),

            reversed: false,
        }
    }
}

impl MainApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            Default::default()
        }
    }

    fn ui_data_input(&mut self, ui: &mut egui::Ui) {
        // The central panel the region left after adding TopPanel's and SidePanel's
        ui.heading("Data Input");
        ui.add_space(10.0);

        StripBuilder::new(ui)
            .size(Size::relative(0.5).at_least(100.0))
            .size(Size::exact(1.0)) // [핵심] 구분선용 1px 셀
            .size(Size::relative(0.25).at_least(60.0))
            .size(Size::exact(1.0)) // [핵심] 구분선용 1px 셀
            .size(Size::remainder())
            .horizontal(|mut strip| {
                strip.cell(|ui| {
                    ui.strong("Students");
                    ui.add_space(10.0);
                    ui_student_table(ui, &mut self.builder_data.students);
                });
                // separator
                strip.cell(|ui| {
                    let rect = ui.available_rect_before_wrap();
                    ui.painter().rect_filled(
                        rect,
                        0.0,
                        ui.visuals().widgets.noninteractive.bg_stroke.color,
                    );
                });
                strip.cell(|ui| {
                    ui.strong("Dislike Group");
                    ui.add_space(10.0);
                    ui_dislike_group(ui, &mut self.builder_data.dislike_group);
                });
                // separator
                strip.cell(|ui| {
                    let rect = ui.available_rect_before_wrap();
                    ui.painter().rect_filled(
                        rect,
                        0.0,
                        ui.visuals().widgets.noninteractive.bg_stroke.color,
                    );
                });
                strip.cell(|ui| {
                    ui.strong("Like Group");
                    ui.add_space(10.0);
                    ui_like_group(ui, &mut self.builder_data.like_group);
                });
            });

        // ui.horizontal(|ui| {
        //     ui.label("Write something: ");
        //     ui.text_edit_singleline(&mut self.label);
        // });
        //
        // ui.add(egui::Slider::new(&mut self.value, 0.0..=10.0).text("value"));
        // if ui.button("Increment").clicked() {
        //     self.value += 1.0;
        // }

        // ui.separator();

        // ui.add(egui::github_link_file!(
        //     "https://github.com/emilk/eframe_template/blob/main/",
        //     "Source code."
        // ));
    }

    fn ui_assign(&mut self, ui: &mut egui::Ui) {
        //
        ui.strong("Class Setup:");
        let mut need_init = false;
        ui.horizontal(|ui| {
            need_init |= ui
                .add(
                    egui::Slider::new(&mut self.builder_data.n_class, 2..=30)
                        .text("number of classes"),
                )
                .changed();
            ui.add_space(50.0);
            need_init |= ui
                .add_sized([100.0, 30.0], egui::Button::new("Simple Assign"))
                .clicked();
        });

        if need_init || self.builder_data.assign_result.is_none() {
            self.builder_data.initial_assign();
        }

        ui.separator();

        if let Some(assign) = &self.builder_data.assign_result {
            ui.strong("Statistics:");
            ui.add_space(10.0);

            assign.ui_statistics(ui);

            ui.add_space(10.0);
            ui.separator();

            ui.strong("Class & Students:");
            ui.add_space(10.0);
            egui::ScrollArea::vertical().show(ui, |ui| {
                for class in &assign.rooms {
                    class.ui_layout(ui, &self.builder_data.students);
                }
            });
        }
    }
}

fn ui_like_group(ui: &mut egui::Ui, like_group: &mut [Vec<StudentId>]) {
    for (igroup, group) in like_group.iter_mut().enumerate() {
        ui.label(format!("Like Group {igroup}"));
        ui.group(|ui| {
            for iid in group {
                ui.label(format!("Student {iid}"));
            }
        });
    }
}

fn ui_dislike_group(ui: &mut egui::Ui, dislike_group: &mut [Vec<StudentId>]) {
    for (igroup, group) in dislike_group.iter_mut().enumerate() {
        ui.label(format!("Dislike Group {igroup}"));
        ui.group(|ui| {
            for iid in group {
                ui.label(format!("Student {iid}"));
            }
        });
    }
}

fn ui_student_table(ui: &mut egui::Ui, students: &mut [Student]) {
    ui.with_layout(Layout::right_to_left(egui::Align::Min), |ui| {
        let response = ui.button("import student list (csv)");
        response.on_hover_ui(|ui| {
            ui.label("Not Implented Yet!");
        });
    });

    let available_height = ui.available_height();
    let table = TableBuilder::new(ui)
        .striped(true)
        .resizable(true)
        .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
        .column(Column::auto())
        .column(Column::auto().at_least(100.0))
        .column(
            Column::auto(), // Column::remainder()
                            //     .at_least(40.0)
                            //     .clip(true)
                            //     .resizable(true),
        )
        .column(Column::auto())
        .column(Column::remainder())
        // .column(Column::remainder())
        .min_scrolled_height(0.0)
        .max_scroll_height(available_height);

    table
        .header(20.0, |mut header| {
            header.col(|ui| {
                ui.strong("Id");
            });
            header.col(|ui| {
                ui.strong("Name");
            });
            header.col(|ui| {
                ui.strong("Gender");
            });
            header.col(|ui| {
                ui.strong("Score");
            });
            header.col(|ui| {
                ui.strong("Note");
            });
        })
        .body(|mut body| {
            for student in students.iter_mut() {
                let is_thick = false; //thick_row(row_index);
                let row_height = if is_thick { 30.0 } else { 18.0 };
                body.row(row_height, |mut row| {
                    row.col(|ui| {
                        ui.label(format!("{:?}", student.id));
                    });
                    row.col(|ui| {
                        // ui.label(long_text(row_index));
                        let mut checked = student.name.is_some();
                        if ui.checkbox(&mut checked, "").changed() {
                            if checked {
                                student.name = Some(String::new());
                            } else {
                                student.name = None;
                            }
                        }
                        if let Some(name) = student.name.as_mut() {
                            ui.text_edit_singleline(name);
                        }
                    });
                    row.col(|ui| {
                        // expanding_content(ui);
                        ui.label(student.gender.as_str());
                    });
                    row.col(|ui| {
                        // ui.checkbox(&mut self.checked, "Click me");
                        ui.label(format!("{:.1}", student.score));
                    });
                    row.col(|ui| {
                        // ui.label(long_text(row_index));
                        let mut checked = student.note.is_some();
                        if ui.checkbox(&mut checked, "").changed() {
                            if checked {
                                student.note = Some(String::new());
                            } else {
                                student.note = None;
                            }
                        }
                        if let Some(note) = student.note.as_mut() {
                            ui.text_edit_singleline(note);
                        }
                    });
                });
            }
        });
}

impl eframe::App for MainApp {
    /// Called by the framework to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        // log::info!("update()");

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.app_mode, AppMode::DataInput, "Step1. Data Input");
                ui.selectable_value(
                    &mut self.app_mode,
                    AppMode::Assign,
                    "Step2. Assign Class Room",
                );
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    egui::widgets::global_theme_preference_buttons(ui);
                });
            });
        });

        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                powered_by_egui_and_eframe(ui);
                egui::warn_if_debug_build(ui);
            });
            //
        });

        egui::CentralPanel::default().show(ctx, |ui| match self.app_mode {
            AppMode::DataInput => self.ui_data_input(ui),
            AppMode::Assign => self.ui_assign(ui),
        });
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
enum AppMode {
    #[default]
    DataInput,
    Assign,
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}

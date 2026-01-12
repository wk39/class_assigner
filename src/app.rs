use crate::class_room::BuilderData;

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
            label: "Class Builder".to_owned(),
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
        use egui_extras::{Column, TableBuilder};

        // The central panel the region left after adding TopPanel's and SidePanel's
        ui.heading("Data Input");

        ui.strong("Class Info:");
        ui.add(
            egui::Slider::new(&mut self.builder_data.n_class, 2..=100).text("number of classes"),
        );

        ui.separator();

        let response = ui.button("import student list (csv)");
        response.on_hover_ui(|ui| {
            ui.label("Not Implented Yet!");
        });

        // let text_height = egui::TextStyle::Body
        //     .resolve(ui.style())
        //     .size
        //     .max(ui.spacing().interact_size.y);

        let available_height = ui.available_height();
        let mut table = TableBuilder::new(ui)
            .striped(true)
            .resizable(true)
            .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
            .column(Column::auto())
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
                    egui::Sides::new().show(
                        ui,
                        |ui| {
                            ui.strong("Id");
                        },
                        |ui| {
                            self.reversed ^=
                                ui.button(if self.reversed { "⬆" } else { "⬇" }).clicked();
                        },
                    );
                });
                header.col(|ui| {
                    ui.strong("Alias");
                });
                header.col(|ui| {
                    ui.strong("Gender");
                });
                header.col(|ui| {
                    ui.strong("Score");
                });
                // header.col(|ui| {
                //     ui.strong("Content");
                // });
            })
            .body(|mut body| {
                self.builder_data.students.iter_mut().for_each(|st| {
                    // ui.horizontal(|ui| {
                    //     ui.label(format!("{:?}", st));
                    // });
                    // let row_index = if self.reversed {
                    //     NUM_MANUAL_ROWS - 1 - row_index
                    // } else {
                    //     row_index
                    // };

                    let is_thick = false; //thick_row(row_index);
                    let row_height = if is_thick { 30.0 } else { 18.0 };
                    body.row(row_height, |mut row| {
                        // row.set_selected(self.selection.contains(&row_index));
                        // row.set_overline(self.overline && row_index % 7 == 3);

                        row.col(|ui| {
                            ui.label(format!("{:?}", st.id));
                        });
                        row.col(|ui| {
                            // ui.label(long_text(row_index));
                            let mut checked = st.alias.is_some();
                            if ui.checkbox(&mut checked, "").changed() {
                                if checked {
                                    st.alias = Some(String::new());
                                } else {
                                    st.alias = None;
                                }
                            }
                            if let Some(mut alias) = st.alias.as_mut() {
                                ui.text_edit_singleline(alias);
                            }
                        });
                        row.col(|ui| {
                            // expanding_content(ui);
                            ui.label(format!("{}", st.gender.as_str()));
                        });
                        row.col(|ui| {
                            // ui.checkbox(&mut self.checked, "Click me");
                            ui.label(format!("{:.1}", st.score));
                        });
                        // row.col(|ui| {
                        //     ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Extend);
                        //     if is_thick {
                        //         ui.heading("Extra thick row");
                        //     } else {
                        //         ui.label("Normal row");
                        //     }
                        // });

                        // self.toggle_row_selection(row_index, &row.response());
                    });
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
    }
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

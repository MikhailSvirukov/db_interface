use egui::TextEdit;
use egui_modal::Modal;

use crate::ui_utils::{
    add_acc_level_drop, render_accessories_header, render_section_header, render_user_header,
};
use crate::ui_utils::{add_is_material_drop, add_pipeline_type_select, render_chain_header};
use crate::{AccessoriesUpdater, UserUpdater};
use crate::{ChainUpdater, SectionUpdater};

pub fn render_user_change_modal(
    change_modal: &Modal,
    user_updater: &mut UserUpdater,
    user_change_flag: &mut bool,
    updater: &mut bool,
) {
    change_modal.show(|ui| {
        ui.add_space(10.0);
        ui.heading("Цепь");

        egui::Grid::new("user_change_grid")
            .striped(true)
            .min_col_width(100.0)
            .show(ui, |ui| {
                render_user_header(ui);
                ui.end_row();

                ui.add(TextEdit::singleline(&mut user_updater.name));
                ui.add(TextEdit::singleline(&mut user_updater.hash));
                ui.add(TextEdit::singleline(&mut user_updater.email));
                ui.add(TextEdit::singleline(&mut user_updater.phone));
                add_acc_level_drop(ui, &mut user_updater.level);

                ui.end_row();
            });

        ui.add_space(10.0);

        ui.horizontal(|ui| {
            if ui.button("Закрыть").clicked() {
                change_modal.close();
            }

            ui.add_space(10.0);

            if ui.button("Отправить").clicked() {
                *user_change_flag = true;
                change_modal.close();
            }
        });
    });
    *updater = true;
}

pub fn render_accessory_change_modal(
    change_modal: &Modal,
    accessories_updater: &mut AccessoriesUpdater,
    accessory_change_flag: &mut bool,
    updater: &mut bool,
) {
    change_modal.show(|ui| {
        ui.add_space(10.0);
        ui.heading("Аксессуары");

        egui::Grid::new("acc_change_grid")
            .striped(true)
            .min_col_width(100.0)
            .show(ui, |ui| {
                render_accessories_header(ui);
                ui.end_row();

                ui.add(TextEdit::singleline(&mut accessories_updater.name));
                ui.add(TextEdit::singleline(&mut accessories_updater.price));
                ui.add(TextEdit::singleline(&mut accessories_updater.tags));
                ui.add(TextEdit::singleline(&mut accessories_updater.opaque));

                ui.end_row();
            });

        ui.add_space(10.0);

        ui.horizontal(|ui| {
            if ui.button("Закрыть").clicked() {
                change_modal.close();
            }

            ui.add_space(10.0);

            if ui.button("Отправить").clicked() {
                *accessory_change_flag = true;
                change_modal.close();
            }
        });
    });
    *updater = true;
}

pub fn render_chain_change_modal(
    change_modal: &Modal,
    chain_updater: &mut ChainUpdater,
    chain_change_flag: &mut bool,
    updater: &mut bool,
) {
    change_modal.show(|ui| {
        ui.add_space(10.0);
        ui.heading("Цепь");

        egui::Grid::new("chains_change_grid")
            .striped(true)
            .min_col_width(100.0)
            .show(ui, |ui| {
                render_chain_header(ui);
                ui.end_row();

                add_pipeline_type_select(ui, &mut chain_updater.pipeline_type);
                ui.add(TextEdit::singleline(&mut chain_updater.price));
                ui.add(TextEdit::singleline(&mut chain_updater.name));
                add_is_material_drop(ui, &mut chain_updater.material);
                ui.add(TextEdit::singleline(&mut chain_updater.tags));
                ui.add(TextEdit::singleline(&mut chain_updater.opaque));

                ui.end_row();
            });

        ui.add_space(10.0);

        ui.horizontal(|ui| {
            if ui.button("Закрыть").clicked() {
                change_modal.close();
            }

            ui.add_space(10.0);

            if ui.button("Отправить").clicked() {
                *chain_change_flag = true;
                change_modal.close();
            }
        });
    });
    *updater = true;
}

pub fn render_section_change_modal(
    change_modal: &Modal,
    section_updater: &mut SectionUpdater,
    section_change_flag: &mut bool,
    updater: &mut bool,
) {
    change_modal.show(|ui| {
        ui.add_space(10.0);
        ui.heading("Секция");

        egui::Grid::new("sections_change_grid")
            .striped(true)
            .min_col_width(100.0)
            .show(ui, |ui| {
                render_section_header(ui);
                ui.end_row();

                add_pipeline_type_select(ui, &mut section_updater.pipeline_type);

                ui.add(TextEdit::singleline(&mut section_updater.section_price));
                ui.add(TextEdit::singleline(&mut section_updater.section_lenght));
                ui.add(TextEdit::singleline(&mut section_updater.coefficient));
                ui.add(TextEdit::singleline(&mut section_updater.tags));
                ui.add(TextEdit::singleline(&mut section_updater.opaque));

                ui.end_row();
            });

        ui.add_space(10.0);

        ui.horizontal(|ui| {
            if ui.button("Закрыть").clicked() {
                change_modal.close();
            }

            ui.add_space(10.0);

            if ui.button("Отправить").clicked() {
                *section_change_flag = true;
                change_modal.close();
            }
        });
    });
    *updater = true;
}

use egui_modal::Modal;

use crate::ui_utils::{render_accessories, render_accessories_header};
use crate::SelectBlockHolder;
use core_app::types::Accessories;

pub fn render_add_accessory_modal(
    modal: &Modal,
    accessories_addition_target: &mut Option<usize>,
    accessories: &Vec<Accessories>,
    selected_block: &mut Vec<SelectBlockHolder>,
) {
    modal.show(|ui| {
        if let Some(i) = *accessories_addition_target {
            egui::Grid::new("accessory_addition_grid")
                .striped(true)
                .min_col_width(100.0)
                .show(ui, |ui| {
                    render_accessories_header(ui);
                    ui.end_row();

                    for accessorie in accessories {
                        render_accessories(accessorie, ui);

                        if ui.button("+").clicked() {
                            selected_block[i]
                                .selected_block
                                .accessories
                                .push(accessorie.id);
                            *accessories_addition_target = None;
                            modal.close();
                        }
                        ui.end_row();
                    }
                });

            if ui.button("Закрыть").clicked() {
                *accessories_addition_target = None;
                modal.close();
            }
        }
    });
}

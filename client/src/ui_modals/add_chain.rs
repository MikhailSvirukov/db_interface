use egui_modal::Modal;

use crate::ui_utils::{render_chain, render_chain_header};
use crate::SelectBlockHolder;
use core_app::types::{Chain, PipelineType};

pub fn render_add_chain_modal(
    modal: &Modal,
    chains: &Vec<Chain>,
    chain_addition_target: &mut Option<usize>,
    current_block_pipeline_type: &mut PipelineType,
    selected_block: &mut Vec<SelectBlockHolder>,
) {
    modal.show(|ui| {
        if let Some(i) = *chain_addition_target {
            egui::Grid::new("chain_addition_grid")
                .striped(true)
                .min_col_width(100.0)
                .show(ui, |ui| {
                    render_chain_header(ui);
                    ui.end_row();
                    for chain in chains {
                        if chain.pipeline_type == *current_block_pipeline_type {
                            render_chain(chain, ui);
                            if ui.button("+").clicked() {
                                selected_block[i].selected_block.chains = chain.id;
                                *chain_addition_target = None;
                                modal.close();
                            }
                            ui.end_row();
                        }
                    }
                });
            if ui.button("Закрыть").clicked() {
                *chain_addition_target = None;
                modal.close();
            }
        }
    });
}

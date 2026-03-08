use egui_modal::Modal;

use crate::ui_utils::{render_field_isize_input, render_section, render_section_header};
use crate::utils::get_section_by_id;
use crate::PipelineTypeHolder;
use core_app::requests::{Id, Lenght, SelectedBlock, Wheel};
use core_app::types::{PipelineType, Section};

pub struct BlockModalState {
    pub selected_section: Option<Id>,
    pub width: String,
    pub distance: String,
    pub error: Option<String>,
}

pub fn render_add_block_modal(
    block_addition: &Modal,
    block_selection_lenght_flag: &mut Option<Id>,
    sections: &mut Vec<Section>,
    error_message: &mut Option<String>,
    pipeline_type_holder: &mut PipelineTypeHolder,
    selected_block: &mut Vec<SelectedBlock>,
    current_block_pipeline_type: &mut PipelineType,
) {
    block_addition.show(|ui| {
        egui::Grid::new("block_addition_grid")
            .striped(true)
            .min_col_width(100.0)
            .show(ui, |ui| {
                ui.heading("Тип конвейера:");
                if *block_selection_lenght_flag == None {
                    render_section_header(ui);
                    ui.end_row();
                    ui.end_row();

                    for section in sections.as_slice() {
                        render_section(section, ui);
                        if ui.button("+").clicked() {
                            *block_selection_lenght_flag = Some(section.id);
                        }
                        ui.end_row()
                    }
                }
            });

        if ui.button("Закрыть").clicked() {
            block_addition.close();
        }
        if let Some(id) = &block_selection_lenght_flag {
            let section = match get_section_by_id(*id, sections) {
                Some(section) => section,
                None => {
                    *error_message = Some("No such section".to_string());
                    return;
                }
            };
            match section.pipeline_type {
                PipelineType::Lamellar | PipelineType::Madal => ui.vertical(|ui| {
                    render_field_isize_input(ui, "Ширина", &mut pipeline_type_holder.length);
                }),
                PipelineType::Rolgang => ui.vertical(|ui| {
                    render_field_isize_input(ui, "Ширина", &mut pipeline_type_holder.length);

                    render_field_isize_input(
                        ui,
                        "Расстояние между роликами",
                        &mut pipeline_type_holder.distance,
                    );
                }),
                PipelineType::None => {
                    *error_message = Some("No type".to_string());
                    block_addition.close();
                    return;
                }
            };
            if ui.button("Подтвердить").clicked() {
                let length = match section.pipeline_type {
                    PipelineType::Lamellar | PipelineType::Madal => {
                        Lenght::Line(if pipeline_type_holder.length.is_empty() {
                            *error_message = Some("Поле ширины не может быть пустым".to_string());
                            return;
                        } else {
                            match pipeline_type_holder.length.parse() {
                                Ok(length) => length,
                                Err(_) => {
                                    *error_message =
                                        Some("Поле ширины некорректно заполнено".to_string());
                                    return;
                                }
                            }
                        })
                    }
                    PipelineType::Rolgang => Lenght::Wheels(Wheel {
                        length: if pipeline_type_holder.length.is_empty() {
                            *error_message = Some("Поле ширины не может быть пустым".to_string());
                            return;
                        } else {
                            match pipeline_type_holder.length.parse() {
                                Ok(length) => length,
                                Err(_) => {
                                    *error_message =
                                        Some("Поле ширины некорректно заполнено".to_string());
                                    return;
                                }
                            }
                        },
                        distance: if pipeline_type_holder.distance.is_empty() {
                            *error_message =
                                Some("Поле расстояния не может быть пустым".to_string());
                            return;
                        } else {
                            match pipeline_type_holder.distance.parse() {
                                Ok(length) => length,
                                Err(_) => {
                                    *error_message =
                                        Some("Поле расстояния некорректно заполнено".to_string());
                                    return;
                                }
                            }
                        },
                    }),
                    _ => unreachable!(),
                };
                selected_block.push(SelectedBlock {
                    section: *id,
                    pipeline_type: section.pipeline_type.clone(),
                    length,
                    chains: vec![],
                    accessories: vec![],
                });

                *current_block_pipeline_type = section.pipeline_type.clone();
                *block_selection_lenght_flag = None;
                block_addition.close();
            }

            if ui.button("Назад").clicked() {
                *block_selection_lenght_flag = None;
                return;
            }
        };
    });
}

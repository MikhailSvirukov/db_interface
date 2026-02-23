use core_app::types::{Chain, Section};

pub fn render_section(section: &Section, ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.label(section.section_type.to_string());
        ui.label(section.width.to_string());
        ui.label(section.price.to_string());
        ui.label(section.length.to_string());
        ui.label(section.is_magnet.to_string());
        ui.label(section.material_sides.to_string());
        ui.label(section.angle.to_string());
        ui.label(section.radius.to_string());
        ui.end_row()
    });
}

pub fn render_chain(chain: &Chain, ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.label(chain.chain_type.to_string());
        ui.label(chain.price.to_string());
        ui.label(chain.is_magnet.to_string());
        ui.label(chain.width.to_string());
        ui.label(chain.name.to_string());
        ui.label(chain.material.to_string());
        ui.end_row()
    });
}

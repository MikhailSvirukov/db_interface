use core_app::types::{Accessories, Chain, Section, User};

pub fn render_section(section: &Section, ui: &mut egui::Ui) {

        ui.label(section.section_type.to_string());
        ui.label(section.width.to_string());
        ui.label(section.price.to_string());
        ui.label(section.length.to_string());
        ui.label(section.is_magnet.to_string());
        ui.label(section.material_sides.to_string());
        ui.label(section.angle.to_string());
        ui.label(section.radius.to_string());

}

pub fn render_section_header(ui: &mut egui::Ui) {
        ui.strong("Тип");
        ui.strong("Ширина");
        ui.strong("Цена");
        ui.strong("Длина");
        ui.strong("Магнитность");
        ui.strong("Материал боков");
        ui.strong("Угол");
        ui.strong("Радиус");
}

pub fn render_chain(chain: &Chain, ui: &mut egui::Ui) {
    
        ui.label(chain.chain_type.to_string());
        ui.label(chain.price.to_string());
        ui.label(chain.is_magnet.to_string());
        ui.label(chain.width.to_string());
        ui.label(chain.name.to_string());
        ui.label(chain.material.to_string());
}

pub fn render_chain_header(ui: &mut egui::Ui) {
        ui.strong("Тип");
        ui.strong("Цена");
        ui.strong("Магнитность");
        ui.strong("Ширина");
        ui.strong("Имя");
        ui.strong("Материал");
}

pub fn render_accessories(accessories: &Accessories, ui: &mut egui::Ui) {
    
        ui.label(accessories.name.to_string());
    
    
}

pub fn render_accessories_header(ui: &mut egui::Ui) {
        ui.strong("Имя");
}

pub fn render_user(user: &User, ui: &mut egui::Ui) {
    
        ui.label(user.name.to_string());
        ui.label(user.hash.to_string());
        ui.label(user.email.to_string());
        ui.label(user.phone.to_string());
        ui.label(user.level.to_string());
    
}

pub fn render_user_header(ui: &mut egui::Ui) {
        ui.strong("Имя");
        ui.strong("Пароль");
        ui.strong("Почта");
        ui.strong("Телефон");
        ui.strong("Уровень");
}

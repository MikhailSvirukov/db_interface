use core_app::requests::Lenght;
use core_app::types::{Accessories, Chain, Section, User};
use egui::TextEdit;

pub fn render_section(section: &Section, ui: &mut egui::Ui) {
    ui.label(section.pipeline_type.to_string());
    ui.label(section.section_type.to_string());
    ui.label(section.price.to_string());
    ui.label(section.length.to_string());
    ui.label(section.is_magnet.to_string());
    ui.label(section.material_sides.to_string());
    ui.label(section.angle.to_string());
    ui.label(section.radius.to_string());
    ui.label(section.tags.join(","));
}

pub fn render_section_header(ui: &mut egui::Ui) {
    ui.strong("Конвейер");
    ui.strong("Тип");
    ui.strong("Цена");
    ui.strong("Длина");
    ui.strong("Магнитность");
    ui.strong("Материал боков");
    ui.strong("Угол");
    ui.strong("Радиус");
    ui.strong("Теги");
}

pub fn render_chain(chain: &Chain, ui: &mut egui::Ui) {
    ui.label(chain.pipeline_type.to_string());
    ui.label(chain.chain_type.to_string());
    ui.label(chain.price.to_string());
    ui.label(chain.is_magnet.to_string());
    ui.label(chain.name.to_string());
    ui.label(chain.material.to_string());
    ui.label(chain.tags.join(","));
}

pub fn render_chain_header(ui: &mut egui::Ui) {
    ui.strong("Конвейер");
    ui.strong("Тип");
    ui.strong("Цена");
    ui.strong("Магнитность");
    ui.strong("Имя");
    ui.strong("Материал");
    ui.strong("Теги");
}

pub fn render_accessories(accessories: &Accessories, ui: &mut egui::Ui) {
    ui.label(&accessories.name);
    ui.label(accessories.price.to_string());
    ui.label(accessories.tags.join(","));
}

pub fn render_accessories_header(ui: &mut egui::Ui) {
    ui.strong("Имя");
    ui.strong("Цена");
    ui.strong("Теги");
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
pub fn add_selected_for_type(ui: &mut egui::Ui, typ: &mut String) {
    ui.horizontal(|ui| {
        egui::ComboBox::new("section_type", "")
            .selected_text(if typ == "Приводящая" {
                "Приводящая"
            } else if typ == "Конечная" {
                "Конечная"
            } else if typ == "Промежуточная" {
                "Промежуточная"
            } else if typ == "Тройная 1к2" {
                "Тройная 1к2"
            } else if typ == "Тройная 2к1" {
                "Тройная 2к1"
            } else if typ == "Поворотная" {
                "Поворотная"
            } else if typ == "Двойная" {
                "Двойная"
            } else {
                ""
            })
            .show_ui(ui, |ui| {
                ui.selectable_value(typ, "Приводящая".to_string(), "Приводящая");
                ui.selectable_value(typ, "Конечная".to_string(), "Конечная");
                ui.selectable_value(typ, "Промежуточная".to_string(), "Промежуточная");
                ui.selectable_value(typ, "Тройная 1к2".to_string(), "Тройная 1к2");
                ui.selectable_value(typ, "Тройная 2к1".to_string(), "Тройная 2к1");
                ui.selectable_value(typ, "Поворотная".to_string(), "Поворотная");
                ui.selectable_value(typ, "Двойная".to_string(), "Двойная");
            });
    });
}

pub fn add_is_magnet_drop(ui: &mut egui::Ui, is_magnet: &mut String) {
    ui.horizontal(|ui| {
        egui::ComboBox::new("magnet_dropdown", "")
            .selected_text(if is_magnet == "true" {
                "да"
            } else if is_magnet == "false" {
                "нет"
            } else {
                ""
            })
            .show_ui(ui, |ui| {
                ui.selectable_value(is_magnet, "true".to_string(), "да");
                ui.selectable_value(is_magnet, "false".to_string(), "нет");
            });
    });
}

pub fn add_is_material_drop(ui: &mut egui::Ui, material: &mut String) {
    ui.horizontal(|ui| {
        egui::ComboBox::new("material_dropdown", "")
            .selected_text(if material == "Сталь" {
                "Сталь"
            } else if material == "Пластик" {
                "Пластик"
            } else {
                ""
            })
            .show_ui(ui, |ui| {
                ui.selectable_value(material, "Сталь".to_string(), "Сталь");
                ui.selectable_value(material, "Пластик".to_string(), "Пластик");
            });
    });
}

pub fn add_sides_material_drop(ui: &mut egui::Ui, material: &mut String) {
    ui.horizontal(|ui| {
        egui::ComboBox::new("section_side_material_sides", "")
            .selected_text(if material == "Сталь" {
                "Сталь"
            } else {
                ""
            })
            .show_ui(ui, |ui| {
                ui.selectable_value(material, "Сталь".to_string(), "Сталь");
            });
    });
}

pub fn add_acc_level_drop(ui: &mut egui::Ui, level: &mut String) {
    ui.horizontal(|ui| {
        egui::ComboBox::new("access", "")
            .selected_text(if level == "Пользователь" {
                "Пользователь"
            } else if level == "Экономист" {
                "Экономист"
            } else if level == "Менеджер" {
                "Менеджер"
            } else if level == "Администратор" {
                "Администратор"
            } else {
                "Программист"
            })
            .show_ui(ui, |ui| {
                ui.selectable_value(level, "Пользователь".to_string(), "Пользователь");
                ui.selectable_value(level, "Экономист".to_string(), "Экономист");
                ui.selectable_value(level, "Менеджер".to_string(), "Менеджер");
                ui.selectable_value(level, "Администратор".to_string(), "Администратор");
                ui.selectable_value(level, "Программист".to_string(), "Программист");
            });
    });
}

pub fn add_pipeline_type_select(ui: &mut egui::Ui, typ: &mut String) {
    ui.horizontal(|ui| {
        egui::ComboBox::new(format!("pipeline_type_{typ}"), "")
            .selected_text(if typ == "Пластинчатый" {
                "Пластинчатый"
            } else if typ == "Модальный" {
                "Модальный"
            } else if typ == "Рольганг" {
                "Рольганг"
            } else {
                ""
            })
            .show_ui(ui, |ui| {
                ui.selectable_value(typ, "Пластинчатый".to_string(), "Пластинчатый");
                ui.selectable_value(typ, "Модальный".to_string(), "Модальный");
                ui.selectable_value(typ, "Рольганг".to_string(), "Рольганг");
            });
    });
}

pub fn render_field_isize_input(ui: &mut egui::Ui, name: &str, modify: &mut String) {
    ui.add_space(10.0);
    ui.horizontal(|ui| {
        ui.label(name);
        ui.add_space(10.0);
        ui.add(TextEdit::singleline(modify));
    });
}

pub fn render_length_type(ui: &mut egui::Ui, length: &Lenght) {
    match length {
        Lenght::None => unreachable!(),
        Lenght::Line(n) => {
            ui.vertical(|ui| {
                ui.add_space(10.0);
                ui.horizontal(|ui| {
                    ui.strong("Ширина");
                    ui.add_space(10.0);
                    ui.label(n.to_string());
                });
            });
        }
        Lenght::Wheels(wh) => {
            ui.vertical(|ui| {
                ui.add_space(10.0);
                ui.horizontal(|ui| {
                    ui.strong("Ширина:");
                    ui.add_space(10.0);
                    ui.label(wh.length.to_string());
                });
                ui.add_space(10.0);
                ui.horizontal(|ui| {
                    ui.strong("Расстояние между роликами:");
                    ui.add_space(10.0);
                    ui.label(wh.length.to_string());
                });
            });
        }
    };
}

use core_app::requests::Id;
use egui_modal::Modal;

pub fn render_delete_modal(delete_modal: &Modal, delete_flag: &mut (bool, Option<Id>)) {
    delete_modal.show(|ui| {
        ui.strong("Удалить?");
        ui.add_space(10.0);

        ui.horizontal(|ui| {
            if ui.button("Удалить").clicked() {
                let (_, id) = *delete_flag;
                *delete_flag = (true, id);
                delete_modal.close();
            }

            if ui.button("Отмена").clicked() {
                *delete_flag = (false, None);
                delete_modal.close();
            }
        });
    });
}

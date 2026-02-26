pub mod ui_utils;
pub mod utils;

use crate::ui_utils::{
    render_accessories, render_accessories_header, render_chain, render_chain_header,
    render_section, render_section_header, render_user, render_user_header,
};
use crate::utils::{
    fill_chain_updater, fill_section_updater, fill_user_updater, get_accessories_by_id,
    get_chain_by_id, get_section_by_id, parse_input_chain, parse_input_section, parse_input_user,
    remove_selected_block, remove_selected_by_id,
};
use core_app::credentials::{AccessLevel, Credentials};
use core_app::requests::{Id, SelectedBlock};
use core_app::types::{Accessories, AuthReply, AuthRequest, Chain, Section, User};
use eframe::{run_native, App, CreationContext, NativeOptions};
use egui::{CentralPanel, Color32, RichText, TextEdit};
use egui_modal::Modal;
use reqwest::blocking::Client;

const ADDRESS: &str = "127.0.0.1:3000";

enum AppState {
    Login,
    // several buttons to modes
    Dashboard,
    // basic mode to create some set of items
    Calculations,
    // change/add sections
    Sections,
    // change/add chains
    Chains,
    // change/add users
    Users,
    // change/add accessories
    Accessories,
}

#[derive(Clone)]
enum UpdateStatus {
    None,
    Update,
    Add,
}

#[derive(Clone)]
pub struct SectionUpdater {
    // Add/Update form inputs
    section_mode: UpdateStatus,
    section_id: String,
    section_type: String,
    section_width: String,
    section_price: String,
    section_is_magnet: String,
    section_material_sides: String,
    section_radius: String,
    section_angle: String,
    section_chains: String,
    section_lenght: String,
}

#[derive(Clone)]
pub struct ChainUpdater {
    section_mode: UpdateStatus,
    id: String,
    r#type: String,
    material: String,
    width: String,
    price: String,
    is_magnet: String,
    name: String,
}

#[derive(Clone)]
pub struct UserUpdater {
    section_mode: UpdateStatus,
    id: String,
    hash: String,
    name: String,
    email: String,
    phone: String,
    level: String,
}

#[derive(Clone)]
struct AccessoriesUpdater {
    section_mode: UpdateStatus,
    id: String,
    name: String,
}

pub struct TemplateApp {
    app_state: AppState,
    login_input: String,
    password_input: String,

    client: Client,

    error_message: Option<String>,
    calculation_sum: Option<String>,

    credentials: Credentials,

    sections: Vec<Section>,
    chains: Vec<Chain>,
    users: Vec<User>,
    accessories: Vec<Accessories>,

    section_updater: SectionUpdater,
    chain_updater: ChainUpdater,
    user_updater: UserUpdater,
    accessories_updater: AccessoriesUpdater,

    selected_block: Vec<SelectedBlock>,

    block_to_remove: Option<usize>,
    chain_to_remove: Option<(usize, Id)>,
    accessories_to_remove: Option<(usize, Id)>,

    chain_addition_target: Option<usize>,
    accessories_addition_target: Option<usize>,

    section_delete: (bool, Option<Id>),
    chain_delete: (bool, Option<Id>),
    accessory_delete: (bool, Option<Id>),
    user_delete: (bool, Option<Id>),

    section_change: bool,
    chain_change: bool,
    user_change: bool,
    accessory_change: bool,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            app_state: AppState::Login,
            login_input: "".to_owned(),
            password_input: "".to_owned(),
            client: Client::new(),
            error_message: None,
            calculation_sum: None,
            credentials: Credentials::default(),
            sections: Vec::new(),
            chains: Vec::new(),
            users: Vec::new(),
            accessories: Vec::new(),
            section_updater: SectionUpdater {
                section_mode: UpdateStatus::None,
                section_id: "".to_string(),
                section_type: "".to_string(),
                section_width: "".to_string(),
                section_price: "".to_string(),
                section_is_magnet: "".to_string(),
                section_material_sides: "".to_string(),
                section_radius: "".to_string(),
                section_angle: "".to_string(),
                section_chains: "".to_string(),
                section_lenght: "".to_string(),
            },
            chain_updater: ChainUpdater {
                section_mode: UpdateStatus::None,
                id: "".to_string(),
                r#type: "".to_string(),
                material: "".to_string(),
                width: "".to_string(),
                price: "".to_string(),
                is_magnet: "".to_string(),
                name: "".to_string(),
            },
            user_updater: UserUpdater {
                section_mode: UpdateStatus::None,
                id: "".to_string(),
                hash: "".to_string(),
                name: "".to_string(),
                email: "".to_string(),
                phone: "".to_string(),
                level: "".to_string(),
            },
            accessories_updater: AccessoriesUpdater {
                section_mode: UpdateStatus::None,
                id: "".to_string(),
                name: "".to_string(),
            },
            selected_block: Vec::new(),
            block_to_remove: None,
            chain_to_remove: None,
            accessories_to_remove: None,
            chain_addition_target: None,
            accessories_addition_target: None,
            section_delete: (false, None),
            chain_delete: (false, None),
            accessory_delete: (false, None),
            user_delete: (false, None),
            section_change: false,
            chain_change: false,
            user_change: false,
            accessory_change: false,
        }
    }
}

impl TemplateApp {
    pub fn new(_cc: &CreationContext<'_>) -> Self {
        Default::default()
    }

    fn render_login_ui(&mut self, ui: &mut egui::Ui) {
        ui.heading("Авторизация");

        ui.add(TextEdit::singleline(&mut self.login_input).hint_text("Логин"));
        ui.add(
            TextEdit::singleline(&mut self.password_input)
                .hint_text("Пароль")
                .password(true),
        );

        if ui.button("Войти").clicked() {
            let login = self.login_input.clone();
            let password = self.password_input.clone();
            let auth_request = AuthRequest {
                credentials: Credentials {
                    login: login.clone(),
                    password: password.clone(),
                    access_level: AccessLevel::User, // Default for login attempt
                },
                payload: (),
            };

            match self
                .client
                .post("http://127.0.0.1:3000/login")
                .json(&auth_request)
                .send()
            {
                Ok(response) => {
                    if response.status().is_success() {
                        match response.json::<AuthReply<()>>() {
                            Ok(auth_reply) => {
                                self.credentials = auth_reply.credentials;
                                self.app_state = AppState::Dashboard;
                            }
                            Err(e) => {
                                self.error_message =
                                    Some(format!("Failed to parse response: {}", e))
                            }
                        }
                    } else {
                        self.error_message =
                            Some("Login failed. Please check your credentials.".to_owned());
                    }
                }
                Err(e) => self.error_message = Some(format!("Error during login: {}", e)),
            }
        }
    }

    fn send_auth_request<T: serde::Serialize + Send + Sync + 'static>(
        &mut self,
        endpoint: &str,
        payload: T,
    ) -> Result<(), String> {
        let auth_request = AuthRequest {
            credentials: self.credentials.clone(),
            payload,
        };

        match self
            .client
            .post(format!("http://{ADDRESS}{endpoint}"))
            .json(&auth_request)
            .send()
        {
            Ok(response) => {
                if response.status().is_success() {
                    Ok(())
                } else {
                    Err(format!(
                        "Server responded with an error: {:?}",
                        response.status()
                    ))
                }
            }
            Err(e) => Err(format!("Failed to send request: {}", e)),
        }
    }

    fn render_dashboard_ui(&mut self, ui: &mut egui::Ui) {
        ui.heading("Dashboard");
        ui.add_space(10.0);
        if ui.button("Авторизация").clicked() {
            self.credentials = Credentials::default();
            self.app_state = AppState::Login;
        }
        ui.add_space(10.0);
        if ui.button("Создать запрос").clicked() {
            self.app_state = AppState::Calculations;
        }
        ui.add_space(10.0);
        if self.credentials.access_level != AccessLevel::User
            && ui.button("Редактировать секции").clicked()
        {
            self.app_state = AppState::Sections;
        }
        ui.add_space(10.0);
        if self.credentials.access_level != AccessLevel::User
            && ui.button("Редактировать цепи").clicked()
        {
            self.app_state = AppState::Chains;
        }
        ui.add_space(10.0);
        if self.credentials.access_level != AccessLevel::User
            && ui.button("Редактировать аксессуары").clicked()
        {
            self.app_state = AppState::Accessories;
        }
        ui.add_space(10.0);
        if self.credentials.access_level == AccessLevel::Programmer
            && ui.button("Редактировать пользователей").clicked()
        {
            self.app_state = AppState::Users;
        }
    }

    fn get_sections(&mut self) {
        match self
            .client
            .get(format!("http://{ADDRESS}/section/get"))
            .json(&AuthRequest {
                credentials: self.credentials.clone(),
                payload: (),
            })
            .send()
        {
            Ok(response) => {
                if response.status().is_success() {
                    match response.json::<Vec<Section>>() {
                        Ok(sections) => {
                            self.sections = sections;
                        }
                        Err(e) => {
                            self.error_message = Some(format!("Failed to parse response: {}", e))
                        }
                    }
                }
            }
            Err(e) => self.error_message = Some(format!("Error during get get: {}", e)),
        }
    }

    fn get_chains(&mut self) {
        match self
            .client
            .get(format!("http://{ADDRESS}/chain/get"))
            .json(&AuthRequest {
                credentials: self.credentials.clone(),
                payload: (),
            })
            .send()
        {
            Ok(response) => {
                if response.status().is_success() {
                    match response.json::<Vec<Chain>>() {
                        Ok(chains) => {
                            self.chains = chains;
                        }
                        Err(e) => {
                            self.error_message = Some(format!("Failed to parse response: {}", e))
                        }
                    }
                }
            }
            Err(e) => self.error_message = Some(format!("Error during get get: {}", e)),
        }
    }

    fn get_users(&mut self) {
        match self
            .client
            .get(format!("http://{ADDRESS}/user/get"))
            .json(&AuthRequest {
                credentials: self.credentials.clone(),
                payload: (),
            })
            .send()
        {
            Ok(response) => {
                if response.status().is_success() {
                    match response.json::<Vec<User>>() {
                        Ok(users) => {
                            self.users = users;
                        }
                        Err(e) => {
                            self.error_message = Some(format!("Failed to parse response: {}", e))
                        }
                    }
                }
            }
            Err(e) => self.error_message = Some(format!("Error during get get: {}", e)),
        }
    }

    fn get_accessories(&mut self) {
        match self
            .client
            .get(format!("http://{ADDRESS}/accessories/get"))
            .json(&AuthRequest {
                credentials: self.credentials.clone(),
                payload: (),
            })
            .send()
        {
            Ok(response) => {
                if response.status().is_success() {
                    match response.json::<Vec<Accessories>>() {
                        Ok(accessories) => {
                            self.accessories = accessories;
                        }
                        Err(e) => {
                            self.error_message = Some(format!("Failed to parse response: {}", e))
                        }
                    }
                }
            }
            Err(e) => self.error_message = Some(format!("Error during get get: {}", e)),
        }
    }

    fn render_calculations_ui(&mut self, ui: &mut egui::Ui) {
        // actually get all associated data
        {
            self.get_sections();
            self.get_chains();
            self.get_accessories();
        }

        ui.heading("Формирование запроса");
        ui.add_space(10.0);
        if ui.button("Назад").clicked() {
            self.sections.clear();
            self.chains.clear();
            self.users.clear();
            self.selected_block.clear();
            self.app_state = AppState::Dashboard;
        }

        let block_addition = egui_modal::Modal::new(ui.ctx(), "Добавить блок");

        block_addition.show(|ui| {
            egui::Grid::new("block_addition_grid")
                .striped(true)
                .min_col_width(100.0)
                .show(ui, |ui| {
                    render_section_header(ui);
                    ui.end_row();
                    for section in &self.sections {
                        render_section(section, ui);
                        if ui.button("+").clicked() {
                            self.selected_block.push(SelectedBlock {
                                section: section.id,
                                chains: Vec::new(),
                                accessories: Vec::new(),
                            });
                            block_addition.close();
                        }
                        ui.end_row()
                    }
                });
            if ui.button("Закрыть").clicked() {
                block_addition.close();
            }
        });

        if ui.button("Добавить блок").clicked() {
            block_addition.open();
        }

        ui.add_space(10.0);

        let chain_addition = egui_modal::Modal::new(ui.ctx(), "Добавить цепь");
        chain_addition.show(|ui| {
            if let Some(i) = self.chain_addition_target {
                egui::Grid::new("chain_addition_grid")
                    .striped(true)
                    .min_col_width(100.0)
                    .show(ui, |ui| {
                        render_chain_header(ui);
                        ui.end_row();

                        for chain in &self.chains {
                            render_chain(chain, ui);

                            if ui.button("+").clicked() {
                                self.selected_block[i].chains.push(chain.id);
                                self.chain_addition_target = None;
                                chain_addition.close();
                            }
                            ui.end_row();
                        }
                    });
                if ui.button("Закрыть").clicked() {
                    self.chain_addition_target = None;
                    chain_addition.close();
                }
            }
        });

        let accessories_addition = egui_modal::Modal::new(ui.ctx(), "Добавить аксессуар");
        accessories_addition.show(|ui| {
            if let Some(i) = self.accessories_addition_target {
                egui::Grid::new("accessory_addition_grid")
                    .striped(true)
                    .min_col_width(100.0)
                    .show(ui, |ui| {
                        render_accessories_header(ui);
                        ui.end_row();

                        for accessories in &self.accessories {
                            render_accessories(accessories, ui);

                            if ui.button("+").clicked() {
                                self.selected_block[i].accessories.push(accessories.id);
                                self.accessories_addition_target = None;
                                accessories_addition.close();
                            }
                            ui.end_row();
                        }
                    });

                if ui.button("Закрыть").clicked() {
                    self.accessories_addition_target = None;
                    accessories_addition.close();
                }
            }
        });

        ui.vertical(|ui| {
            for (block_index, block) in self.selected_block.iter_mut().enumerate() {
                ui.heading(format!("Блок {}", block_index));

                // section
                ui.strong("Секция:");
                let section = match get_section_by_id(block.section, &self.sections) {
                    Some(section) => section,
                    None => {
                        self.error_message = Some("No such section".to_string());
                        return;
                    }
                };
                egui::Grid::new(format!("section_in_block_grid_{block_index}"))
                    .striped(true)
                    .min_col_width(100.0)
                    .show(ui, |ui| {
                        render_section_header(ui);
                        ui.end_row();
                        render_section(section, ui);
                        if ui.button("Убрать").clicked() {
                            self.block_to_remove = Some(block_index);
                        }
                        ui.end_row();
                    });

                // chains
                ui.strong("Цепи:");
                egui::Grid::new(format!("chains_grid_{block_index}"))
                    .striped(true)
                    .min_col_width(100.0)
                    .show(ui, |ui| {
                        render_chain_header(ui);
                        ui.end_row();
                        for (_, chain) in block.chains.iter().enumerate() {
                            let chain = match get_chain_by_id(*chain, &self.chains) {
                                Some(chain) => chain,
                                None => {
                                    self.error_message = Some("No such section".to_string());
                                    return;
                                }
                            };

                            render_chain(chain, ui);
                            if ui.button("Убрать").clicked() {
                                self.chain_to_remove = Some((block_index, chain.id));
                            }
                            ui.end_row();
                        }
                    });
                if ui.button("Добавить цепь").clicked() {
                    self.chain_addition_target = Some(block_index);
                    chain_addition.open();
                }

                // accessories
                ui.strong("Аксессуары:");
                egui::Grid::new(format!("accessories_grid_{block_index}"))
                    .striped(true)
                    .min_col_width(100.0)
                    .show(ui, |ui| {
                        render_accessories_header(ui);
                        ui.end_row();
                        for (_, accessories) in block.accessories.iter().enumerate() {
                            let accessories =
                                match get_accessories_by_id(*accessories, &self.accessories) {
                                    Some(acc) => acc,
                                    None => {
                                        self.error_message = Some("No such section".to_string());
                                        return;
                                    }
                                };

                            render_accessories(accessories, ui);
                            if ui.button("Убрать").clicked() {
                                self.accessories_to_remove = Some((block_index, accessories.id));
                            }
                            ui.end_row();
                        }
                    });

                if ui.button("Добавить аксессуар").clicked() {
                    self.accessories_addition_target = Some(block_index);
                    accessories_addition.open();
                }
            }
        });

        if let Some(index) = self.block_to_remove.take() {
            remove_selected_block(index, &mut self.selected_block)
        }

        if let Some((block, id)) = self.chain_to_remove.take() {
            remove_selected_by_id(id, &mut self.selected_block[block].chains)
        }

        if let Some((block, id)) = self.accessories_to_remove.take() {
            remove_selected_by_id(id, &mut self.selected_block[block].accessories)
        }
    }

    fn render_sections_ui(&mut self, ui: &mut egui::Ui) {
        self.get_sections();

        let delete_modal = Modal::new(ui.ctx(), "Подтверждение");
        delete_modal.show(|ui| {
            ui.strong("Удалить?");
            ui.add_space(10.0);
            ui.horizontal(|ui| {
                if ui.button("Удалить").clicked() {
                    let (_, id) = self.section_delete;
                    self.section_delete = (true, id);
                    delete_modal.close();
                }
                if ui.button("Отмена").clicked() {
                    self.section_delete = (false, None);
                    delete_modal.close();
                }
            })
        });

        let change = Modal::new(ui.ctx(), "Изменить");
        change.show(|ui| {
            ui.add_space(10.0);
            ui.heading("Секция");
            egui::Grid::new("sections_change_grid")
                .striped(true)
                .min_col_width(100.0)
                .show(ui, |ui| {
                    render_section_header(ui);
                    ui.end_row();
                    ui.add(TextEdit::singleline(&mut self.section_updater.section_type));
                    ui.add(TextEdit::singleline(
                        &mut self.section_updater.section_width,
                    ));
                    ui.add(TextEdit::singleline(
                        &mut self.section_updater.section_price,
                    ));
                    ui.add(TextEdit::singleline(
                        &mut self.section_updater.section_lenght,
                    ));
                    ui.add(TextEdit::singleline(
                        &mut self.section_updater.section_is_magnet,
                    ));
                    ui.add(TextEdit::singleline(
                        &mut self.section_updater.section_material_sides,
                    ));
                    ui.add(TextEdit::singleline(
                        &mut self.section_updater.section_angle,
                    ));
                    ui.add(TextEdit::singleline(
                        &mut self.section_updater.section_radius,
                    ));
                    ui.add(TextEdit::singleline(
                        &mut self.section_updater.section_chains,
                    ));
                    ui.end_row();
                });
            ui.add_space(10.0);
            ui.horizontal(|ui| {
                if ui.button("Закрыть").clicked() {
                    change.close();
                }
                ui.add_space(10.0);
                if ui.button("Отправить").clicked() {
                    self.section_change = true;
                    change.close();
                }
            })
        });

        ui.add_space(10.0);
        ui.heading("Секции");
        ui.add_space(10.0);
        if ui.button("Назад").clicked() {
            self.sections.clear();
            self.app_state = AppState::Dashboard;
        }
        ui.add_space(10.0);

        egui::Grid::new("sections_ui_grid")
            .striped(true)
            .min_col_width(100.0)
            .show(ui, |ui| {
                render_section_header(ui);
                ui.end_row();
                for section in &self.sections.clone() {
                    render_section(section, ui);
                    if ui.button("Изменить").clicked() {
                        fill_section_updater(&mut self.section_updater, section);
                        self.section_updater.section_mode = UpdateStatus::Update;
                        change.open();
                    }

                    if ui.button("Удалить").clicked() {
                        self.section_delete = (false, Some(section.id));
                        delete_modal.open();
                    }
                    ui.end_row()
                }
            });
        ui.add_space(10.0);
        if ui.button("Добавить").clicked() {
            self.section_updater.section_mode = UpdateStatus::Add;
            change.open();
        }

        if let (flag, Some(id)) = self.section_delete {
            if flag {
                match self.send_auth_request("/section/delete", vec![id]) {
                    Ok(_) => {}
                    Err(err) => {
                        self.error_message = Some(format!("Error sending delete message: {}", err));
                    }
                }
            }
        }

        if self.section_change {
            match parse_input_section(&mut self.section_updater) {
                Ok(section) => {
                    match self.section_updater.section_mode {
                        UpdateStatus::None => {}
                        UpdateStatus::Update => {
                            match self.send_auth_request("/section/update", section) {
                                Ok(_) => {}
                                Err(err) => {
                                    self.error_message =
                                        Some(format!("Error sending delete message: {}", err));
                                }
                            }
                        }
                        UpdateStatus::Add => {
                            match self.send_auth_request("/section/add", section) {
                                Ok(_) => {}
                                Err(err) => {
                                    self.error_message =
                                        Some(format!("Error sending delete message: {}", err));
                                }
                            }
                        }
                    };
                }
                Err(err) => {
                    self.error_message = Some(format!("Error sending delete message: {}", err));
                }
            };
            self.section_updater.section_mode = UpdateStatus::None;
            self.section_change = false;
        }
    }

    fn render_chains_ui(&mut self, ui: &mut egui::Ui) {
        self.get_chains();

        let delete_modal = Modal::new(ui.ctx(), "Подтверждение");
        delete_modal.show(|ui| {
            ui.strong("Удалить?");
            ui.add_space(10.0);
            ui.horizontal(|ui| {
                if ui.button("Удалить").clicked() {
                    let (_, id) = self.chain_delete;
                    self.chain_delete = (true, id);
                    delete_modal.close();
                }
                if ui.button("Отмена").clicked() {
                    self.chain_delete = (false, None);
                    delete_modal.close();
                }
            })
        });

        let change = Modal::new(ui.ctx(), "Изменить");
        change.show(|ui| {
            ui.add_space(10.0);
            ui.heading("Цепь");
            egui::Grid::new("chains_change_grid")
                .striped(true)
                .min_col_width(100.0)
                .show(ui, |ui| {
                    render_chain_header(ui);
                    ui.end_row();
                    ui.add(TextEdit::singleline(&mut self.chain_updater.r#type));
                    ui.add(TextEdit::singleline(&mut self.chain_updater.price));
                    ui.add(TextEdit::singleline(&mut self.chain_updater.is_magnet));
                    ui.add(TextEdit::singleline(&mut self.chain_updater.width));
                    ui.add(TextEdit::singleline(&mut self.chain_updater.name));
                    ui.add(TextEdit::singleline(&mut self.chain_updater.material));
                    ui.end_row();
                });
            ui.add_space(10.0);
            ui.horizontal(|ui| {
                if ui.button("Закрыть").clicked() {
                    change.close();
                }
                ui.add_space(10.0);
                if ui.button("Отправить").clicked() {
                    self.chain_change = true;
                    change.close();
                }
            })
        });

        ui.add_space(10.0);

        ui.heading("Цепи");
        ui.add_space(10.0);
        if ui.button("Назад").clicked() {
            self.chains.clear();
            self.app_state = AppState::Dashboard;
        }
        ui.add_space(10.0);
        egui::Grid::new("chains_ui_grid")
            .striped(true)
            .min_col_width(100.0)
            .show(ui, |ui| {
                render_chain_header(ui);
                ui.end_row();

                for chain in &self.chains.clone() {
                    render_chain(chain, ui);
                    if ui.button("Изменить").clicked() {
                        fill_chain_updater(&mut self.chain_updater, chain);
                        self.chain_updater.section_mode = UpdateStatus::Update;
                        change.open();
                    }

                    if ui.button("Удалить").clicked() {
                        self.chain_delete = (false, Some(chain.id));
                        delete_modal.open();
                    }
                    ui.end_row()
                }
            });
        ui.add_space(10.0);
        if ui.button("Добавить").clicked() {
            self.chain_updater.section_mode = UpdateStatus::Add;
            change.open();
        }

        if let (flag, Some(id)) = self.chain_delete {
            if flag {
                match self.send_auth_request("/chain/delete", vec![id]) {
                    Ok(_) => {}
                    Err(err) => {
                        self.error_message = Some(format!("Error sending delete message: {}", err));
                    }
                }
            }
        }

        if self.chain_change {
            match parse_input_chain(&mut self.chain_updater) {
                Ok(chain) => {
                    match self.chain_updater.section_mode {
                        UpdateStatus::None => {}
                        UpdateStatus::Update => {
                            match self.send_auth_request("/chain/update", chain) {
                                Ok(_) => {}
                                Err(err) => {
                                    self.error_message =
                                        Some(format!("Error sending delete message: {}", err));
                                }
                            }
                        }
                        UpdateStatus::Add => match self.send_auth_request("/chain/add", chain) {
                            Ok(_) => {}
                            Err(err) => {
                                self.error_message =
                                    Some(format!("Error sending delete message: {}", err));
                            }
                        },
                    };
                }

                Err(err) => {
                    self.error_message = Some(format!("Error sending delete message: {}", err));
                }
            }
            self.chain_updater.section_mode = UpdateStatus::None;
            self.chain_change = false;
        }
    }

    fn render_user_ui(&mut self, ui: &mut egui::Ui) {
        self.get_users();

        let delete_modal = Modal::new(ui.ctx(), "Подтверждение");
        delete_modal.show(|ui| {
            ui.strong("Удалить?");
            ui.add_space(10.0);
            ui.horizontal(|ui| {
                if ui.button("Удалить").clicked() {
                    let (_, id) = self.user_delete;
                    self.user_delete = (true, id);
                    delete_modal.close();
                }
                if ui.button("Отмена").clicked() {
                    self.user_delete = (false, None);
                    delete_modal.close();
                }
            })
        });

        let change = Modal::new(ui.ctx(), "Изменить");
        change.show(|ui| {
            ui.add_space(10.0);
            ui.heading("Цепь");
            egui::Grid::new("user_change_grid")
                .striped(true)
                .min_col_width(100.0)
                .show(ui, |ui| {
                    render_user_header(ui);
                    ui.end_row();
                    ui.add(TextEdit::singleline(&mut self.user_updater.name));
                    ui.add(TextEdit::singleline(&mut self.user_updater.hash));
                    ui.add(TextEdit::singleline(&mut self.user_updater.email));
                    ui.add(TextEdit::singleline(&mut self.user_updater.phone));
                    ui.add(TextEdit::singleline(&mut self.user_updater.level));
                    ui.end_row();
                });
            ui.add_space(10.0);
            ui.horizontal(|ui| {
                if ui.button("Закрыть").clicked() {
                    change.close();
                }
                ui.add_space(10.0);
                if ui.button("Отправить").clicked() {
                    self.user_change = true;
                    change.close();
                }
            })
        });

        ui.add_space(10.0);

        ui.heading("Пользователи");
        ui.add_space(10.0);
        if ui.button("Назад").clicked() {
            self.sections.clear();
            self.app_state = AppState::Dashboard;
        }
        ui.add_space(10.0);
        egui::Grid::new("user_ui_grid")
            .striped(true)
            .min_col_width(100.0)
            .show(ui, |ui| {
                render_user_header(ui);
                ui.end_row();

                for user in &self.users.clone() {
                    render_user(user, ui);
                    // default user
                    if user.id != 1 {
                        if ui.button("Изменить").clicked() {
                            fill_user_updater(&mut self.user_updater, user);
                            self.user_updater.section_mode = UpdateStatus::Update;
                            change.open();
                        }

                        if ui.button("Удалить").clicked() {
                            self.user_delete = (false, Some(user.id));
                            delete_modal.open();
                        }
                    }
                    ui.end_row()
                }
            });

        ui.add_space(10.0);
        if ui.button("Добавить").clicked() {
            self.user_updater.section_mode = UpdateStatus::Add;
            change.open();
        }

        if let (flag, Some(id)) = self.user_delete {
            if flag {
                match self.send_auth_request("/user/delete", vec![id]) {
                    Ok(_) => {}
                    Err(err) => {
                        self.error_message = Some(format!("Error sending delete message: {}", err));
                    }
                }
            }
        }

        if self.user_change {
            match parse_input_user(&mut self.user_updater) {
                Ok(user) => {
                    match self.user_updater.section_mode {
                        UpdateStatus::None => {}
                        UpdateStatus::Update => {
                            match self.send_auth_request("/user/update", user) {
                                Ok(_) => {}
                                Err(err) => {
                                    self.error_message =
                                        Some(format!("Error sending delete message: {}", err));
                                }
                            }
                        }
                        UpdateStatus::Add => match self.send_auth_request("/user/add", user) {
                            Ok(_) => {}
                            Err(err) => {
                                self.error_message =
                                    Some(format!("Error sending delete message: {}", err));
                            }
                        },
                    };
                }
                Err(err) => {
                    self.error_message = Some(format!("Error sending delete message: {}", err));
                }
            };
            self.user_updater.section_mode = UpdateStatus::None;
            self.user_change = false;
        }
    }

    fn render_accessories_ui(&mut self, ui: &mut egui::Ui) {
        self.get_accessories();

        let delete_modal = Modal::new(ui.ctx(), "Подтверждение");
        delete_modal.show(|ui| {
            ui.strong("Удалить?");
            ui.add_space(10.0);
            ui.horizontal(|ui| {
                if ui.button("Удалить").clicked() {
                    let (_, id) = self.accessory_delete;
                    self.accessory_delete = (true, id);
                    delete_modal.close();
                }
                if ui.button("Отмена").clicked() {
                    self.accessory_delete = (false, None);
                    delete_modal.close();
                }
            })
        });

        let change = Modal::new(ui.ctx(), "Изменить");
        change.show(|ui| {
            ui.add_space(10.0);
            ui.heading("Аксессуары");
            egui::Grid::new("acc_change_grid")
                .striped(true)
                .min_col_width(100.0)
                .show(ui, |ui| {
                    render_accessories_header(ui);
                    ui.end_row();
                    ui.add(TextEdit::singleline(&mut self.accessories_updater.name));
                    ui.end_row();
                });
            ui.add_space(10.0);
            ui.horizontal(|ui| {
                if ui.button("Close").clicked() {
                    change.close();
                }
                ui.add_space(10.0);
                if ui.button("Send").clicked() {
                    self.accessory_change = true;
                    change.close();
                }
            })
        });

        if let Some(msg) = self.error_message.take() {
            ui.label(msg);
        }

        ui.add_space(10.0);

        ui.heading("Аксессуары");
        ui.add_space(10.0);
        if ui.button("Назад").clicked() {
            self.accessories.clear();
            self.app_state = AppState::Dashboard;
        }
        ui.add_space(10.0);
        egui::Grid::new("acc_ui_grid")
            .striped(true)
            .min_col_width(100.0)
            .show(ui, |ui| {
                render_accessories_header(ui);
                ui.end_row();

                for accessories in &self.accessories.clone() {
                    render_accessories(accessories, ui);

                    if ui.button("Изменить").clicked() {
                        change.open();
                    }

                    if ui.button("Удалить").clicked() {
                        self.accessory_delete = (false, Some(accessories.id));
                        delete_modal.open();
                    }
                    ui.end_row()
                }
            });

        if let (flag, Some(id)) = self.accessory_delete {
            if flag {
                match self.send_auth_request("/accessories/delete", vec![id]) {
                    Ok(_) => {}
                    Err(err) => {
                        self.error_message = Some(format!("Error sending delete message: {}", err));
                    }
                }
            }
        }

        if self.accessory_change {
            //TODO
        }
    }
}

impl App for TemplateApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::both()
                .auto_shrink([false; 2])
                .show(ui, |ui| {
                    if let Some(msg) = self.error_message.take() {
                        ui.label(RichText::new(msg).color(Color32::RED));
                    }
                    ui.add_space(15.0);
                    match self.app_state {
                        AppState::Login => self.render_login_ui(ui),
                        AppState::Dashboard => self.render_dashboard_ui(ui),
                        AppState::Calculations => self.render_calculations_ui(ui),
                        AppState::Sections => self.render_sections_ui(ui),
                        AppState::Chains => self.render_chains_ui(ui),
                        AppState::Users => self.render_user_ui(ui),
                        AppState::Accessories => self.render_accessories_ui(ui),
                    }
                })
        });
    }
}

fn main() -> eframe::Result {
    run_native(
        "GUI Client",
        NativeOptions::default(),
        Box::new(|cc| Ok(Box::new(TemplateApp::new(cc)))),
    )
}

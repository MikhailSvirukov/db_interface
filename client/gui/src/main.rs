pub mod ui_utils;
pub mod utils;

use crate::ui_utils::{render_accessories, render_chain, render_section};
use crate::utils::{
    get_accessories_by_id, get_chain_by_id, get_section_by_id, remove_selected_block,
    remove_selected_by_id,
};
use core_app::credentials::{AccessLevel, Credentials};
use core_app::requests::{Id, SelectedBlock};
use core_app::types::{Accessories, AuthReply, AuthRequest, Chain, Section, User};
use eframe::{run_native, App, CreationContext, NativeOptions};
use egui::{CentralPanel, TextEdit};
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
    Remove,
}

#[derive(Clone)]
struct SectionUpdater {
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
struct ChainUpdater {
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
struct UserUpdater {
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

    // fn send_auth_request<T: serde::Serialize + Send + Sync + 'static>(
    //     &mut self,
    //     endpoint: &str,
    //     payload: T,
    // ) -> Result<(), String> {
    //     let client = Client::new();
    //     let auth_request = AuthRequest {
    //         credentials: self.credentials.clone().ok_or("Not logged in")?,
    //         payload,
    //     };
    //
    //     match client.post(endpoint).json(&auth_request).send() {
    //         Ok(response) => {
    //             if response.status().is_success() {
    //                 self.fetch_dashboard_data();
    //                 Ok(())
    //             } else {
    //                 Err(format!(
    //                     "Server responded with an error: {:?}",
    //                     response.status()
    //                 ))
    //             }
    //         }
    //         Err(e) => Err(format!("Failed to send request: {}", e)),
    //     }
    // }

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
            .get(format!("http://{ADDRESS}/users/get"))
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
            ui.horizontal(|ui| {
                ui.label("Тип");
                ui.label("Ширина");
                ui.label("Цена");
                ui.label("Длина");
                ui.label("Магнитность");
                ui.label("Материал боков");
                ui.label("Угол");
                ui.label("Радиус");
                ui.end_row();
            });
            for section in &self.sections {
                ui.horizontal(|ui| {
                    render_section(section, ui);
                    ui.add_space(10.0);
                    if ui.button("+").clicked() {
                        self.selected_block.push(SelectedBlock {
                            section: section.id,
                            chains: Vec::new(),
                            accessories: Vec::new(),
                        });
                        block_addition.close();
                    }
                    ui.end_row()
                });
            }
            ui.add_space(10.0);
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
                ui.horizontal(|ui| {
                    ui.label("Тип");
                    ui.label("Цена");
                    ui.label("Магнитность");
                    ui.label("Ширина");
                    ui.label("Имя");
                    ui.label("Материал");
                    ui.end_row();
                });

                for chain in &self.chains {
                    ui.horizontal(|ui| {
                        render_chain(chain, ui);
                        ui.add_space(10.0);

                        if ui.button("+").clicked() {
                            self.selected_block[i].chains.push(chain.id);
                            self.chain_addition_target = None;
                            chain_addition.close();
                        }
                    });
                }

                ui.add_space(10.0);
                if ui.button("Закрыть").clicked() {
                    self.chain_addition_target = None;
                    chain_addition.close();
                }
            }
        });

        let accessories_addition = egui_modal::Modal::new(ui.ctx(), "Добавить аксессуар");
        accessories_addition.show(|ui| {
            if let Some(i) = self.accessories_addition_target {
                ui.horizontal(|ui| {
                    ui.label("Имя");
                    ui.end_row();
                });

                for accessories in &self.accessories {
                    ui.horizontal(|ui| {
                        render_accessories(accessories, ui);
                        ui.add_space(10.0);

                        if ui.button("+").clicked() {
                            self.selected_block[i].accessories.push(accessories.id);
                            self.accessories_addition_target = None;
                            accessories_addition.close();
                        }
                    });
                }

                ui.add_space(10.0);
                if ui.button("Закрыть").clicked() {
                    self.accessories_addition_target = None;
                    accessories_addition.close();
                }
            }
        });

        for (block_index, block) in self.selected_block.iter_mut().enumerate() {
            ui.heading(format!("Блок {}", block_index));

            // section
            ui.add_space(5.0);
            ui.label("Секция:");
            ui.add_space(5.0);
            let section = match get_section_by_id(block.section, &self.sections) {
                Some(section) => section,
                None => {
                    self.error_message = Some("No such section".to_string());
                    return;
                }
            };
            ui.horizontal(|ui| {
                render_section(section, ui);
                ui.add_space(10.0);
                if ui.button("Убрать").clicked() {
                    self.block_to_remove = Some(block_index);
                }
            });

            // chains
            ui.add_space(5.0);
            ui.label("Цепи:");
            ui.add_space(5.0);
            for (_, chain) in block.chains.iter().enumerate() {
                let chain = match get_chain_by_id(*chain, &self.chains) {
                    Some(chain) => chain,
                    None => {
                        self.error_message = Some("No such section".to_string());
                        return;
                    }
                };
                ui.horizontal(|ui| {
                    render_chain(chain, ui);
                    ui.add_space(10.0);
                    if ui.button("Убрать").clicked() {
                        self.chain_to_remove = Some((block_index, chain.id));
                    }
                });
            }
            ui.add_space(5.0);
            if ui.button("Добавить цепь").clicked() {
                self.chain_addition_target = Some(block_index);
                chain_addition.open();
            }
            ui.add_space(5.0);

            // accessories
            ui.add_space(5.0);
            ui.label("Аксессуары:");
            ui.add_space(5.0);
            for (_, accessories) in block.accessories.iter().enumerate() {
                let accessories = match get_accessories_by_id(*accessories, &self.accessories) {
                    Some(acc) => acc,
                    None => {
                        self.error_message = Some("No such section".to_string());
                        return;
                    }
                };
                ui.horizontal(|ui| {
                    render_accessories(accessories, ui);
                    ui.add_space(10.0);
                    if ui.button("Убрать").clicked() {
                        self.accessories_to_remove = Some((block_index, accessories.id));
                    }
                });
            }
            ui.add_space(5.0);
            if ui.button("Добавить цепь").clicked() {
                self.accessories_addition_target = Some(block_index);
                chain_addition.open();
            }
            ui.add_space(5.0);
        }

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
        ui.heading("Секции");
        ui.add_space(10.0);
        if ui.button("Назад").clicked() {
            self.sections.clear();
            self.app_state = AppState::Dashboard;
        }
        ui.add_space(10.0);
        ui.label("Not yet implemented");
    }

    fn render_chains_ui(&mut self, ui: &mut egui::Ui) {
        ui.heading("Цепи");
        ui.add_space(10.0);
        if ui.button("Назад").clicked() {
            self.sections.clear();
            self.app_state = AppState::Dashboard;
        }
        ui.add_space(10.0);
        ui.label("Not yet implemented");
    }

    fn render_user_ui(&mut self, ui: &mut egui::Ui) {
        ui.heading("Пользователи");
        ui.add_space(10.0);
        if ui.button("Назад").clicked() {
            self.sections.clear();
            self.app_state = AppState::Dashboard;
        }
        ui.add_space(10.0);
        ui.label("Not yet implemented");
    }

    fn render_accessories_ui(&mut self, ui: &mut egui::Ui) {
        ui.heading("Аксессуары");
        ui.add_space(10.0);
        if ui.button("Назад").clicked() {
            self.sections.clear();
            self.app_state = AppState::Dashboard;
        }
        ui.add_space(10.0);
        ui.label("Not yet implemented");
    }
}

impl App for TemplateApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| match self.app_state {
            AppState::Login => self.render_login_ui(ui),
            AppState::Dashboard => self.render_dashboard_ui(ui),
            AppState::Calculations => self.render_calculations_ui(ui),
            AppState::Sections => self.render_sections_ui(ui),
            AppState::Chains => self.render_chains_ui(ui),
            AppState::Users => self.render_user_ui(ui),
            AppState::Accessories => self.render_accessories_ui(ui),
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

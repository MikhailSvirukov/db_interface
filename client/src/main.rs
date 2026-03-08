pub mod action_utils;
pub mod request;
pub mod ui_modals;
pub mod ui_utils;
pub mod utils;

use crate::ui_utils::{
    render_accessories, render_accessories_header, render_chain, render_chain_header,
    render_length_type, render_section, render_section_header, render_user, render_user_header,
};
use crate::utils::{
    fill_accessories_updater, fill_chain_updater, fill_section_updater, fill_user_updater,
    get_accessories_by_id, get_chain_by_id, get_section_by_id, parse_input_accessories,
    parse_input_chain, parse_input_section, parse_input_user, remove_selected_block,
    remove_selected_by_id,
};
use core_app::credentials::{AccessLevel, Credentials};
use core_app::requests::{Id, SelectedBlock};
use core_app::types::{Accessories, AuthReply, AuthRequest, Chain, PipelineType, Section, User};
use eframe::{run_native, App, CreationContext, NativeOptions};
use egui::{CentralPanel, Color32, FontId, RichText, TextEdit};
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

#[derive(Clone, Copy)]
pub enum UpdateStatus {
    None,
    Update,
    Add,
}

#[derive(Clone)]
pub struct SectionUpdater {
    // Add/Update form inputs
    section_mode: UpdateStatus,
    section_id: String,
    pipeline_type: String,
    section_price: String,
    tags: String,
    section_lenght: String,
    coefficient: String,
    opaque: String,
    name: String,
}

#[derive(Clone)]
pub struct ChainUpdater {
    section_mode: UpdateStatus,
    id: String,
    pipeline_type: String,
    material: String,
    price: String,
    name: String,
    tags: String,
    opaque: String,
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
pub struct AccessoriesUpdater {
    section_mode: UpdateStatus,
    id: String,
    name: String,
    price: String,
    tags: String,
    opaque: String,
}

pub struct SelectBlockHolder {
    selected_block: SelectedBlock,
    fields: LengthFields,
}

#[derive(Default, Clone)]
pub struct LengthFields {
    length: String,
    distance: String,
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

    selected_block: Vec<SelectBlockHolder>,

    block_to_remove: Option<usize>,
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

    block_selection_lenght_flag: Option<Id>,
    new_type_holder: LengthFields,
    current_block_pipeline_type: PipelineType,

    make_request: bool,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            app_state: AppState::Login,
            login_input: "".to_owned(),
            password_input: "".to_owned(),
            client: Client::new(),
            error_message: None,
            //calculation_sum: None,
            calculation_sum: None,
            credentials: Credentials::default(),
            sections: Vec::new(),
            chains: Vec::new(),
            users: Vec::new(),
            accessories: Vec::new(),
            section_updater: SectionUpdater {
                section_mode: UpdateStatus::None,
                section_id: "".to_string(),
                pipeline_type: "".to_string(),
                section_price: "".to_string(),
                section_lenght: "".to_string(),
                tags: "".to_string(),
                coefficient: "".to_string(),
                opaque: "".to_string(),
                name: "".to_string(),
            },
            chain_updater: ChainUpdater {
                section_mode: UpdateStatus::None,
                id: "".to_string(),
                pipeline_type: "".to_string(),
                material: "".to_string(),
                price: "".to_string(),
                name: "".to_string(),
                tags: "".to_string(),
                opaque: "".to_string(),
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
                price: "".to_string(),
                tags: "".to_string(),
                opaque: "".to_string(),
            },
            selected_block: Vec::new(),
            block_to_remove: None,
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
            block_selection_lenght_flag: None,
            new_type_holder: LengthFields::default(),
            current_block_pipeline_type: PipelineType::None,
            make_request: true,
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
                                self.error_message.take();
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
        self.error_message.take();
    }

    fn render_calculations_ui(&mut self, ui: &mut egui::Ui) {
        // actually get all associated data

        if ui.button("Обновить").clicked() || self.make_request {
            action_utils::get::get_section(
                &mut self.sections,
                &mut self.error_message,
                self.credentials.clone(),
                &mut self.client,
            );
            action_utils::get::get_chains(
                &mut self.chains,
                &mut self.error_message,
                self.credentials.clone(),
                &mut self.client,
            );
            action_utils::get::get_accessories(
                &mut self.accessories,
                &mut self.error_message,
                self.credentials.clone(),
                &mut self.client,
            );
            self.make_request = false;
        }
        ui.add_space(20.0);

        ui.heading("Формирование запроса");
        ui.add_space(10.0);

        let block_addition = egui_modal::Modal::new(ui.ctx(), "Добавить блок");

        ui_modals::add_block::render_add_block_modal(
            &block_addition,
            &mut self.block_selection_lenght_flag,
            &mut self.sections,
            &mut self.error_message,
            &mut self.new_type_holder,
            &mut self.selected_block,
            &mut self.current_block_pipeline_type,
        );

        if ui.button("Добавить блок").clicked() {
            block_addition.open();
        }

        ui.add_space(10.0);

        let chain_addition = egui_modal::Modal::new(ui.ctx(), "Добавить цепь");
        ui_modals::add_chain::render_add_chain_modal(
            &chain_addition,
            &self.chains,
            &mut self.chain_addition_target,
            &mut self.current_block_pipeline_type,
            &mut self.selected_block,
        );

        let accessories_addition = egui_modal::Modal::new(ui.ctx(), "Добавить аксессуар");
        ui_modals::add_accessory::render_add_accessory_modal(
            &accessories_addition,
            &mut self.accessories_addition_target,
            &self.accessories,
            &mut self.selected_block,
        );

        ui.vertical(|ui| {
            for (block_index, block) in self.selected_block.iter_mut().enumerate() {
                let section = match get_section_by_id(block.selected_block.section, &self.sections)
                {
                    Some(section) => section,
                    None => {
                        self.error_message = Some("No such section".to_string());
                        return;
                    }
                };
                ui.heading(format!("Секция {}", section.name));
                match block.selected_block.pipeline_type {
                    PipelineType::Madal | PipelineType::Rolgang => {
                        render_length_type(ui, block);
                    }
                    _ => {}
                }

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
                ui.strong("Цепь:");
                egui::Grid::new(format!("chains_grid_{block_index}"))
                    .striped(true)
                    .min_col_width(100.0)
                    .show(ui, |ui| {
                        render_chain_header(ui);
                        ui.end_row();

                        let chain = match get_chain_by_id(block.selected_block.chains, &self.chains)
                        {
                            Some(chain) => chain,
                            None => {
                                // TODO: fix in future
                                if block.selected_block.chains > 0 {
                                    self.error_message = Some("No such chain".to_string());
                                }
                                return;
                            }
                        };

                        render_chain(chain, ui);
                        if ui.button("Заненить").clicked() {
                            self.chain_addition_target = Some(block_index);
                            chain_addition.open();
                        }
                        ui.end_row();
                    });

                if block.selected_block.chains < 0 && ui.button("Добавить цепь").clicked()
                {
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
                        for accessories in block.selected_block.accessories.iter() {
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
                ui.add_space(20.0);
            }
        });

        if let Some(index) = self.block_to_remove.take() {
            remove_selected_block(index, &mut self.selected_block)
        }

        if let Some((block, id)) = self.accessories_to_remove.take() {
            remove_selected_by_id(
                id,
                &mut self.selected_block[block].selected_block.accessories,
            )
        }

        let calculation_modal = Modal::new(ui.ctx(), "get_calculation_modal");
        calculation_modal.show(|ui| {
            if let Some(n) = &self.calculation_sum {
                ui.heading("Итоговая сумма:");
                ui.add_space(10.0);
                ui.label(RichText::new(n).font(FontId::proportional(15.0)));
            }
            if ui.button("Закрыть").clicked() {
                calculation_modal.close();
            }
        });

        if ui
            .button(RichText::new("Расчитать сумму").font(FontId::proportional(15.0)))
            .clicked()
        {
            if self
                .selected_block
                .iter()
                .filter(|block| block.selected_block.chains < 0)
                .count()
                > 0
            {
                self.error_message = Some("Некоторые поля Цепи не заполнены".to_string());
                return;
            }
            action_utils::calculations::get_calculations(
                &mut self.calculation_sum,
                &self.selected_block,
                &mut self.error_message,
                self.credentials.clone(),
                &mut self.client,
            );

            if self.calculation_sum.is_some() {
                self.error_message.take();
                calculation_modal.open();
            }
        }
    }

    fn render_sections_ui(&mut self, ui: &mut egui::Ui) {
        if ui.button("Обновить").clicked() || self.make_request {
            action_utils::get::get_section(
                &mut self.sections,
                &mut self.error_message,
                self.credentials.clone(),
                &mut self.client,
            );
            self.make_request = false;
        }
        ui.add_space(20.0);

        let delete_modal = Modal::new(ui.ctx(), "Подтверждение");
        ui_modals::delete::render_delete_modal(&delete_modal, &mut self.section_delete);

        let change = Modal::new(ui.ctx(), "Изменить");
        ui_modals::change::render_section_change_modal(
            &change,
            &mut self.section_updater,
            &mut self.section_change,
            &mut self.make_request,
        );

        ui.add_space(10.0);
        ui.heading("Секции");
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

        action_utils::delete::process_delete(
            &mut self.section_delete,
            &mut self.error_message,
            self.credentials.clone(),
            &mut self.client,
            ADDRESS,
            "section",
        );

        action_utils::update::process_update(
            self.section_updater.clone(),
            &mut self.section_change,
            &mut self.section_updater.section_mode,
            &mut self.error_message,
            parse_input_section,
            ADDRESS,
            "section",
            self.credentials.clone(),
            &mut self.client,
        );
    }

    fn render_chains_ui(&mut self, ui: &mut egui::Ui) {
        if ui.button("Обновить").clicked() || self.make_request {
            action_utils::get::get_chains(
                &mut self.chains,
                &mut self.error_message,
                self.credentials.clone(),
                &mut self.client,
            );
            self.make_request = false;
        }
        ui.add_space(20.0);

        let delete_modal = Modal::new(ui.ctx(), "Подтверждение");
        ui_modals::delete::render_delete_modal(&delete_modal, &mut self.chain_delete);

        let change = Modal::new(ui.ctx(), "Изменить");
        ui_modals::change::render_chain_change_modal(
            &change,
            &mut self.chain_updater,
            &mut self.chain_change,
            &mut self.make_request,
        );

        ui.add_space(10.0);

        ui.heading("Цепи");
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

        action_utils::delete::process_delete(
            &mut self.chain_delete,
            &mut self.error_message,
            self.credentials.clone(),
            &mut self.client,
            ADDRESS,
            "chain",
        );

        action_utils::update::process_update(
            self.chain_updater.clone(),
            &mut self.chain_change,
            &mut self.chain_updater.section_mode,
            &mut self.error_message,
            parse_input_chain,
            ADDRESS,
            "chain",
            self.credentials.clone(),
            &mut self.client,
        );
    }

    fn render_user_ui(&mut self, ui: &mut egui::Ui) {
        if ui.button("Обновить").clicked() || self.make_request {
            action_utils::get::get_users(
                &mut self.users,
                &mut self.error_message,
                self.credentials.clone(),
                &mut self.client,
            );
            self.make_request = false;
        }
        ui.add_space(20.0);

        let delete_modal = Modal::new(ui.ctx(), "Подтверждение");
        ui_modals::delete::render_delete_modal(&delete_modal, &mut self.chain_delete);

        let change = Modal::new(ui.ctx(), "Изменить");
        ui_modals::change::render_user_change_modal(
            &change,
            &mut self.user_updater,
            &mut self.user_change,
            &mut self.make_request,
        );

        ui.add_space(10.0);

        ui.heading("Пользователи");
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

        action_utils::delete::process_delete(
            &mut self.user_delete,
            &mut self.error_message,
            self.credentials.clone(),
            &mut self.client,
            ADDRESS,
            "user",
        );

        action_utils::update::process_update(
            self.user_updater.clone(),
            &mut self.user_change,
            &mut self.user_updater.section_mode,
            &mut self.error_message,
            parse_input_user,
            ADDRESS,
            "user",
            self.credentials.clone(),
            &mut self.client,
        );
    }

    fn render_accessories_ui(&mut self, ui: &mut egui::Ui) {
        if ui.button("Обновить").clicked() || self.make_request {
            action_utils::get::get_accessories(
                &mut self.accessories,
                &mut self.error_message,
                self.credentials.clone(),
                &mut self.client,
            );
            self.make_request = false;
        }
        ui.add_space(20.0);

        let delete_modal = Modal::new(ui.ctx(), "Подтверждение");
        ui_modals::delete::render_delete_modal(&delete_modal, &mut self.user_delete);

        let change = Modal::new(ui.ctx(), "Изменить");
        ui_modals::change::render_accessory_change_modal(
            &change,
            &mut self.accessories_updater,
            &mut self.accessory_change,
            &mut self.make_request,
        );

        ui.add_space(10.0);

        ui.heading("Аксессуары");
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
                        fill_accessories_updater(&mut self.accessories_updater, accessories);
                        self.accessories_updater.section_mode = UpdateStatus::Update;
                        change.open();
                    }

                    if ui.button("Удалить").clicked() {
                        self.accessory_delete = (false, Some(accessories.id));
                        delete_modal.open();
                    }
                    ui.end_row()
                }
            });

        ui.add_space(10.0);
        if ui.button("Добавить").clicked() {
            self.accessories_updater.section_mode = UpdateStatus::Add;
            change.open();
        }

        action_utils::delete::process_delete(
            &mut self.accessory_delete,
            &mut self.error_message,
            self.credentials.clone(),
            &mut self.client,
            ADDRESS,
            "accessories",
        );

        action_utils::update::process_update(
            self.accessories_updater.clone(),
            &mut self.accessory_change,
            &mut self.accessories_updater.section_mode,
            &mut self.error_message,
            parse_input_accessories,
            ADDRESS,
            "accessories",
            self.credentials.clone(),
            &mut self.client,
        );
    }
}

impl App for TemplateApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::both()
                .auto_shrink([false; 2])
                .show(ui, |ui| {
                    if self.credentials.access_level != AccessLevel::None {
                        if ui.button("Назад").clicked() {
                            self.sections.clear();
                            self.chains.clear();
                            self.users.clear();
                            self.selected_block.clear();
                            self.error_message.take();
                            self.make_request = true;
                            self.app_state = AppState::Dashboard;
                        }
                    }
                    ui.add_space(10.0);
                    if let Some(msg) = self.error_message.as_ref() {
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
        "Pipeline client",
        NativeOptions::default(),
        Box::new(|cc| Ok(Box::new(TemplateApp::new(cc)))),
    )
}

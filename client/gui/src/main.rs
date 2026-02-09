use core_app::credentials::{AccessLevel, Credentials};
use core_app::types::{AuthRequest, AuthReply, Chain, Section, User};
use egui::{CentralPanel, Context, TextEdit, Ui};
use eframe::{run_native, App, CreationContext, NativeOptions};
use reqwest::blocking::Client;
use std::collections::HashMap;
use egui_extras::{Column, TableBuilder};
use core_app::types;

enum AppState {
    Login,
    Dashboard,
}

pub struct TemplateApp {
    app_state: AppState,
    login_input: String,
    password_input: String,
    error_message: Option<String>,
    credentials: Option<Credentials>,
    access_level: Option<AccessLevel>,
    sections: Vec<Section>,
    chains: Vec<Chain>,
    users: Vec<User>,

    // Add/Update form inputs
    add_section_input_name: String,
    add_section_input_description: String,
    update_section_input_id: String,
    update_section_input_name: String,
    update_section_input_description: String,

    add_chain_input_name: String,
    update_chain_input_id: String,
    update_chain_input_name: String,

    add_user_input_login: String,
    add_user_input_password: String,
    add_user_input_access_level: String,
    update_user_input_id: String,
    update_user_input_login: String,
    update_user_input_password: String,
    update_user_input_access_level: String,

}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            app_state: AppState::Login,
            login_input: "".to_owned(),
            password_input: "".to_owned(),
            error_message: None,
            credentials: None,
            access_level: None,
            sections: Vec::new(),
            chains: Vec::new(),
            users: Vec::new(),

            add_section_input_name: "".to_owned(),
            add_section_input_description: "".to_owned(),
            update_section_input_id: "".to_owned(),
            update_section_input_name: "".to_owned(),
            update_section_input_description: "".to_owned(),

            add_chain_input_name: "".to_owned(),
            update_chain_input_id: "".to_owned(),
            update_chain_input_name: "".to_owned(),

            add_user_input_login: "".to_owned(),
            add_user_input_password: "".to_owned(),
            add_user_input_access_level: "User".to_owned(),
            update_user_input_id: "".to_owned(),
            update_user_input_login: "".to_owned(),
            update_user_input_password: "".to_owned(),
            update_user_input_access_level: "User".to_owned(),
        }
    }
}

impl TemplateApp {
    pub fn new(_cc: &CreationContext<'_>) -> Self {
        Default::default()
    }

    fn render_login_ui(&mut self, ui: &mut egui::Ui) {
        ui.heading("Login");

        ui.add(TextEdit::singleline(&mut self.login_input).hint_text("Login"));
        ui.add(TextEdit::singleline(&mut self.password_input).hint_text("Password").password(true));

        if ui.button("Login").clicked() {
            let login = self.login_input.clone();
            let password = self.password_input.clone();
            let client = Client::new();
            let auth_request = AuthRequest {
                credentials: Credentials {
                    login: login.clone(),
                    password: password.clone(),
                    access_level: AccessLevel::User, // Default for login attempt
                },
                payload: (), 
            };

            match client
                .post("http://127.0.0.1:3000/login")
                .json(&auth_request)
                .send()
            {
                Ok(response) => {
                    if response.status().is_success() {
                        match response.json::<AuthReply<HashMap<String, serde_json::Value>>>() {
                            Ok(auth_reply) => {
                                self.credentials = Some(auth_reply.credentials.clone());
                                self.access_level = Some(auth_reply.credentials.access_level);
                                
                                if let Some(sections_value) = auth_reply.payload.get("sections") {
                                    self.sections = serde_json::from_value(sections_value.clone()).unwrap_or_default();
                                }
                                if let Some(chains_value) = auth_reply.payload.get("chains") {
                                    self.chains = serde_json::from_value(chains_value.clone()).unwrap_or_default();
                                }
                                if let Some(users_value) = auth_reply.payload.get("users") {
                                    self.users = serde_json::from_value(users_value.clone()).unwrap_or_default();
                                }
                                self.app_state = AppState::Dashboard;
                                self.error_message = None;
                            }
                            Err(e) => self.error_message = Some(format!("Failed to parse response: {}", e)),
                        }
                    } else {
                        self.error_message = Some("Login failed. Please check your credentials.".to_owned());
                    }
                }
                Err(e) => self.error_message = Some(format!("Error during login: {}", e)),
            }
        }

        if let Some(msg) = &self.error_message {
            ui.label(egui::RichText::new(msg).color(egui::Color32::RED));
        }
    }

    fn send_auth_request<T: serde::Serialize + Send + Sync + 'static>(
        &mut self,
        endpoint: &str,
        payload: T,
    ) -> Result<(), String> {
        let client = Client::new();
        let auth_request = AuthRequest {
            credentials: self.credentials.clone().ok_or("Not logged in")?,
            payload,
        };

        match client.post(endpoint).json(&auth_request).send() {
            Ok(response) => {
                if response.status().is_success() {
                    self.fetch_dashboard_data();
                    Ok(())
                } else {
                    Err(format!("Server responded with an error: {:?}", response.status()))
                }
            }
            Err(e) => Err(format!("Failed to send request: {}", e)),
        }
    }

    fn fetch_dashboard_data(&mut self) {
        let client = Client::new();
        if let Some(credentials) = &self.credentials {
            let auth_request = AuthRequest {
                credentials: credentials.clone(),
                payload: (),
            };
            match client
                .post("http://127.0.0.1:3000/login")
                .json(&auth_request)
                .send()
            {
                Ok(response) => {
                    if response.status().is_success() {
                        match response.json::<AuthReply<HashMap<String, serde_json::Value>>>() {
                            Ok(auth_reply) => {
                                if let Some(sections_value) = auth_reply.payload.get("sections") {
                                    self.sections = serde_json::from_value(sections_value.clone()).unwrap_or_default();
                                }
                                if let Some(chains_value) = auth_reply.payload.get("chains") {
                                    self.chains = serde_json::from_value(chains_value.clone()).unwrap_or_default();
                                }
                                if let Some(users_value) = auth_reply.payload.get("users") {
                                    self.users = serde_json::from_value(users_value.clone()).unwrap_or_default();
                                }
                                self.error_message = None;
                            }
                            Err(e) => self.error_message = Some(format!("Failed to parse response: {}", e)),
                        }
                    } else {
                        self.error_message = Some("Failed to fetch dashboard data.".to_owned());
                    }
                }
                Err(e) => self.error_message = Some(format!("Error fetching dashboard data: {}", e)),
            }
        }
    }


    fn render_dashboard_ui(&mut self, ui: &mut Ui) {
        ui.heading("Dashboard");

        if let Some(msg) = &self.error_message {
            ui.label(egui::RichText::new(msg).color(egui::Color32::RED));
        }

        // Sections Table
        ui.add_space(10.0);
        ui.heading("Sections");
        TableBuilder::new(ui)
            .striped(true)
            .columns(Column::auto(), 3)
            .header(20.0, |mut header| {
                header.col(|ui| { ui.strong("Id"); });
                header.col(|ui| { ui.strong("Name"); });
                header.col(|ui| { ui.strong("Description"); });
            })
            .body(|mut body| {
                for section in &self.sections {
                    body.row(20.0, |mut row| {
                        row.col(|ui| { ui.label(section.id.to_string()); });
                        row.col(|ui| { ui.label(&section.length.to_string()); });
                    });
                }
            });

        // Section Add/Update Forms (for Administrator/Programmer)
        if let Some(AccessLevel::Administrator) | Some(AccessLevel::Programmer) = self.access_level {
            ui.add_space(15.0);
            ui.group(|mut ui: egui::Ui| {
                ui.set_width(300.0);
                ui.heading("Add Section");
                ui.add(TextEdit::singleline(&mut self.add_section_input_name).hint_text("Name"));
                ui.add(TextEdit::singleline(&mut self.add_section_input_description).hint_text("price"));
                if ui.button("Add Section").clicked() {
                    let new_section = Section {
                        id: 0, // ID is auto-generated on server
                        section_type: types::Type::Driving,
                        width: 0,
                        length: 0,
                        price: 0,
                        is_magnet: false,
                        material_sides: types::SideMaterial::Steel,
                        radius: 0,
                        angle: 0,
                        chains: vec![],
                    };
                    match self.send_auth_request("http://127.0.0.1:3000/section/add", new_section) {
                        Ok(_) => {
                            self.add_section_input_name.clear();
                            self.add_section_input_description.clear();
                            self.error_message = None;
                        },
                        Err(e) => self.error_message = Some(e),
                    }
                }
            });

            ui.add_space(10.0);
            ui.group(|ui: &mut egui::Ui| {
                ui.set_width(300.0);
                ui.heading("Update Section");
                ui.add(TextEdit::singleline(&mut self.update_section_input_id).hint_text("Section ID"));
                ui.add(TextEdit::singleline(&mut self.update_section_input_name).hint_text("New Name"));
                ui.add(TextEdit::singleline(&mut self.update_section_input_description).hint_text("New Description"));
                if ui.button("Update Section").clicked() {
                    if let Ok(id) = self.update_section_input_id.parse::<i32>() {
                        let updated_section = Section {
                            id: id as isize,
                            section_type: types::Type::Driving,
                            width: 0,
                            length: 0,
                            price: 0,
                            is_magnet: false,
                            material_sides: types::SideMaterial::Steel,
                            radius: 0,
                            angle: 0,
                            chains: vec![],
                        };
                        match self.send_auth_request("http://127.0.0.1:3000/section/update", updated_section) {
                            Ok(_) => {
                                self.update_section_input_id.clear();
                                self.update_section_input_name.clear();
                                self.update_section_input_description.clear();
                                self.error_message = None;
                            },
                            Err(e) => self.error_message = Some(e),
                        }
                    } else {
                        self.error_message = Some("Invalid Section ID".to_owned());
                    }
                }
            });
        }

        // Chains Table
        ui.add_space(20.0);
        ui.heading("Chains");
        TableBuilder::new(ui)
            .striped(true)
            .columns(Column::auto(), 2)
            .header(20.0, |mut header| {
                header.col(|ui| { ui.strong("Id"); });
                header.col(|ui| { ui.strong("Name"); });
            })
            .body(|mut body| {
                for chain in &self.chains {
                    body.row(20.0, |mut row| {
                        row.col(|ui| { ui.label(chain.id.to_string()); });
                        row.col(|ui| { ui.label(&chain.name); });
                    });
                }
            });

        // Chain Add/Update Forms (for Administrator/Programmer)
        if let Some(AccessLevel::Administrator) | Some(AccessLevel::Programmer) = self.access_level {
            ui.add_space(15.0);
            ui.group(|ui| {
                ui.set_width(300.0);
                ui.heading("Add Chain");
                ui.add(TextEdit::singleline(&mut self.add_chain_input_name).hint_text("Name"));
                if ui.button("Add Chain").clicked() {
                    let new_chain = Chain {
                        id: 0, // ID is auto-generated on server
                        chain_type: types::Type::Driving,
                        material: types::ChainMaterial::Steel,
                        width: 0,
                        price: 0,
                        is_magnet: false,
                        name: self.add_chain_input_name.clone(),
                    };
                    match self.send_auth_request("http://127.0.0.1:3000/chain/add", new_chain) {
                        Ok(_) => {
                            self.add_chain_input_name.clear();
                            self.error_message = None;
                        },
                        Err(e) => self.error_message = Some(e),
                    }
                }
            });

            ui.add_space(10.0);
            ui.group(|ui| {
                ui.set_width(300.0);
                ui.heading("Update Chain");
                ui.add(TextEdit::singleline(&mut self.update_chain_input_id).hint_text("Chain ID"));
                ui.add(TextEdit::singleline(&mut self.update_chain_input_name).hint_text("New Name"));
                if ui.button("Update Chain").clicked() {
                    if let Ok(id) = self.update_chain_input_id.parse::<i32>() {
                        let updated_chain = Chain {
                            id: id as isize,
                            chain_type: types::Type::Driving,
                            material: types::ChainMaterial::Steel,
                            width: 0,
                            price: 0,
                            is_magnet: false,
                            name: self.update_chain_input_name.clone(),
                        };
                        match self.send_auth_request("http://127.0.0.1:3000/chain/update", updated_chain) {
                            Ok(_) => {
                                self.update_chain_input_id.clear();
                                self.update_chain_input_name.clear();
                                self.error_message = None;
                            },
                            Err(e) => self.error_message = Some(e),
                        }
                    } else {
                        self.error_message = Some("Invalid Chain ID".to_owned());
                    }
                }
            });
        }

        // Users Table (only for Admin/Programmer)
        if let Some(AccessLevel::Administrator) | Some(AccessLevel::Programmer) = self.access_level {
            ui.add_space(20.0);
            ui.heading("Users");
             TableBuilder::new(ui)
                .striped(true)
                .columns(Column::auto(), 4)
                .header(20.0, |mut header| {
                    header.col(|ui| { ui.strong("Id"); });
                    header.col(|ui| { ui.strong("Login"); });
                    header.col(|ui| { ui.strong("Password"); });
                    header.col(|ui| { ui.strong("Access Level"); });
                })
                .body(|mut body| {
                    for user in &self.users {
                        body.row(20.0, |mut row| {
                            row.col(|ui| { ui.label(user.id.to_string()); });
                            row.col(|ui| { ui.label(&user.name); });
                            row.col(|ui| { ui.label(&user.hash); });
                            row.col(|ui| { ui.label(&format!("{:?}", user.level)); });
                        });
                    }
                });

            ui.add_space(15.0);
            ui.group(|ui| {
                ui.set_width(300.0);
                ui.heading("Add User");
                ui.add(TextEdit::singleline(&mut self.add_user_input_login).hint_text("Login"));
                ui.add(TextEdit::singleline(&mut self.add_user_input_password).hint_text("Password"));
                ui.add(TextEdit::singleline(&mut self.add_user_input_access_level).hint_text("Access Level (e.g., User, Administrator)"));
                if ui.button("Add User").clicked() {
                    if let Ok(level) = self.add_user_input_access_level.parse::<AccessLevel>() {
                        let new_user = User {
                            id: 0, // ID is auto-generated on server
                            hash: self.add_user_input_password,
                            name: self.add_user_input_login,
                            email: "".to_string(),
                            phone: "".to_string(),
                            level,
                        };
                        match self.send_auth_request("http://127.0.0.1:3000/user/add", new_user) {
                            Ok(_) => {
                                self.add_user_input_login.clear();
                                self.add_user_input_password.clear();
                                self.add_user_input_access_level = "User".to_owned();
                                self.error_message = None;
                            },
                            Err(e) => self.error_message = Some(e),
                        }
                    } else {
                        self.error_message = Some("Invalid Access Level".to_owned());
                    }
                }
            });

            ui.add_space(10.0);
            ui.group(|ui| {
                ui.set_width(300.0);
                ui.heading("Update User");
                ui.add(TextEdit::singleline(&mut self.update_user_input_id).hint_text("User ID"));
                ui.add(TextEdit::singleline(&mut self.update_user_input_login).hint_text("New Login"));
                ui.add(TextEdit::singleline(&mut self.update_user_input_password).hint_text("New Password"));
                ui.add(TextEdit::singleline(&mut self.update_user_input_access_level).hint_text("New Access Level"));
                if ui.button("Update User").clicked() {
                    if let Ok(id) = self.update_user_input_id.parse::<i32>() {
                        if let Ok(level) = self.update_user_input_access_level.parse::<AccessLevel>() {
                            let updated_user = User {
                                id: id as isize,
                                hash: self.update_user_input_password,
                                name: self.update_user_input_login,
                                email: "".to_string(),
                                phone: "".to_string(),
                                level,
                            };
                            match self.send_auth_request("http://127.0.0.1:3000/user/update", updated_user) {
                                Ok(_) => {
                                    self.update_user_input_id.clear();
                                    self.update_user_input_login.clear();
                                    self.update_user_input_password.clear();
                                    self.update_user_input_access_level = "User".to_owned();
                                    self.error_message = None;
                                },
                                Err(e) => self.error_message = Some(e),
                            }
                        } else {
                            self.error_message = Some("Invalid Access Level".to_owned());
                        }
                    } else {
                        self.error_message = Some("Invalid User ID".to_owned());
                    }
                }
            });
        }
    }
}

impl App for TemplateApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui|
{
            match self.app_state {
                AppState::Login => self.render_login_ui(ui),
                AppState::Dashboard => self.render_dashboard_ui(ui),
            }
        });
    }
}

fn main() {
    run_native(
        "GUI Client",
        NativeOptions::default(),
        Box::new(|cc| Box::new(TemplateApp::new(cc))),
    ).unwrap();
}

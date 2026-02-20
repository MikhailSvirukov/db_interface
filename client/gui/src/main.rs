use core_app::credentials::{AccessLevel, Credentials};
use core_app::types::{AuthReply, AuthRequest, Chain, Section, User};
use eframe::{run_native, App, CreationContext, NativeOptions};
use egui::{CentralPanel, TextEdit, Ui};
use egui_modal::Modal;
use reqwest::blocking::Client;
use std::collections::HashMap;

enum AppState {
    Login,
    Dashboard,
}

#[derive(Clone, PartialEq)]
enum UpdateStatus {
    None,
    Add,
    Change,
    //Remove?
}

#[derive(Clone)]
struct SectionUpdater {
    // Add/Update form inputs
    section_mode: UpdateStatus,
    win_open: bool,
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
    // Add/Update form inputs
    section_mode: UpdateStatus,
    win_open: bool,
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
    // Add/Update form inputs
    section_mode: UpdateStatus,
    win_open: bool,
    id: String,
    hash: String,
    name: String,
    email: String,
    phone: String,
    level: String,
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

    section_updater: SectionUpdater,
    chain_updater: ChainUpdater,
    user_updater: UserUpdater, // selected
                               //selected_sections: Vec<isize>,
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
            section_updater: SectionUpdater {
                section_mode: UpdateStatus::None,
                win_open: false,
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
            //selected_sections: vec![],
            chain_updater: ChainUpdater {
                section_mode: UpdateStatus::None,
                win_open: false,
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
                win_open: false,
                id: "".to_string(),
                hash: "".to_string(),
                name: "".to_string(),
                email: "".to_string(),
                phone: "".to_string(),
                level: "".to_string(),
            },
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
        ui.add(
            TextEdit::singleline(&mut self.password_input)
                .hint_text("Password")
                .password(true),
        );

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
                                    self.sections =
                                        serde_json::from_value(sections_value.clone()).unwrap();
                                }
                                if let Some(chains_value) = auth_reply.payload.get("chains") {
                                    self.chains =
                                        serde_json::from_value(chains_value.clone()).unwrap();
                                }
                                if let Some(users_value) = auth_reply.payload.get("users") {
                                    self.users =
                                        serde_json::from_value(users_value.clone()).unwrap();
                                }
                                self.app_state = AppState::Dashboard;
                                self.error_message = None;
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
                    Err(format!(
                        "Server responded with an error: {:?}",
                        response.status()
                    ))
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
                                    self.sections =
                                        serde_json::from_value(sections_value.clone()).unwrap();
                                }
                                if let Some(chains_value) = auth_reply.payload.get("chains") {
                                    self.chains =
                                        serde_json::from_value(chains_value.clone()).unwrap();
                                }
                                if let Some(users_value) = auth_reply.payload.get("users") {
                                    self.users =
                                        serde_json::from_value(users_value.clone()).unwrap();
                                }
                                self.error_message = None;
                            }
                            Err(e) => {
                                self.error_message =
                                    Some(format!("Failed to parse response: {}", e))
                            }
                        }
                    } else {
                        self.error_message = Some("Failed to fetch dashboard data.".to_owned());
                    }
                }
                Err(e) => {
                    self.error_message = Some(format!("Error fetching dashboard data: {}", e))
                }
            }
        }
    }
    fn send_change_section(&mut self) {
        let section = match self.parse_input_section(UpdateStatus::Change) {
            None => {
                return;
            }
            Some(s) => s,
        };
        match self.send_auth_request("http://127.0.0.1:3000/section/update", section) {
            Ok(_) => self.fetch_dashboard_data(),
            Err(err) => self.error_message = Some(format!("Error during update: {}", err)),
        }
    }

    fn send_change_chain(&mut self) {
        let chain = match self.parse_input_chain(UpdateStatus::Change) {
            None => {
                return;
            }
            Some(s) => s,
        };
        match self.send_auth_request("http://127.0.0.1:3000/chain/update", chain) {
            Ok(_) => self.fetch_dashboard_data(),
            Err(err) => self.error_message = Some(format!("Error during update: {}", err)),
        }
    }

    fn send_change_user(&mut self) {
        let user = match self.parse_input_user(UpdateStatus::Change) {
            None => {
                return;
            }
            Some(s) => s,
        };
        match self.send_auth_request("http://127.0.0.1:3000/user/update", user) {
            Ok(_) => self.fetch_dashboard_data(),
            Err(err) => self.error_message = Some(format!("Error during update: {}", err)),
        }
    }

    fn send_add_section(&mut self) {
        let section = match self.parse_input_section(UpdateStatus::Add) {
            None => {
                return;
            }
            Some(s) => s,
        };
        match self.send_auth_request("http://127.0.0.1:3000/section/add", section) {
            Ok(_) => self.fetch_dashboard_data(),
            Err(err) => self.error_message = Some(format!("Error during update: {}", err)),
        }
    }

    fn send_add_chain(&mut self) {
        let chain = match self.parse_input_chain(UpdateStatus::Add) {
            None => {
                return;
            }
            Some(s) => s,
        };
        match self.send_auth_request("http://127.0.0.1:3000/chain/add", chain) {
            Ok(_) => self.fetch_dashboard_data(),
            Err(err) => self.error_message = Some(format!("Error during update: {}", err)),
        }
    }

    fn send_add_user(&mut self) {
        let user = match self.parse_input_user(UpdateStatus::Add) {
            None => {
                return;
            }
            Some(s) => s,
        };
        match self.send_auth_request("http://127.0.0.1:3000/user/add", user) {
            Ok(_) => self.fetch_dashboard_data(),
            Err(err) => self.error_message = Some(format!("Error during update: {}", err)),
        }
    }

    fn parse_input_section(&mut self, update_status: UpdateStatus) -> Option<Section> {
        match update_status {
            UpdateStatus::Add => {
                Some(Section {
                    //because default
                    id: -1,
                    section_type: {
                        if self.section_updater.section_type.is_empty() {
                            self.error_message = Some("Field can't be empty".to_string());
                            return None;
                        }
                        if let Ok(val) = self.section_updater.section_type.parse() {
                            val
                        } else {
                            self.error_message = Some("Error fetching dashboard data".to_string());
                            return None;
                        }
                    },
                    width: {
                        if self.section_updater.section_width.is_empty() {
                            self.error_message = Some("Field can't be empty".to_string());
                            return None;
                        }
                        if let Ok(val) = self.section_updater.section_width.parse() {
                            val
                        } else {
                            self.error_message = Some("Error fetching dashboard data".to_string());
                            return None;
                        }
                    },
                    length: {
                        if self.section_updater.section_lenght.is_empty() {
                            self.error_message = Some("Field can't be empty".to_string());
                            return None;
                        }
                        if let Ok(val) = self.section_updater.section_lenght.parse() {
                            val
                        } else {
                            self.error_message = Some("Error fetching dashboard data".to_string());
                            return None;
                        }
                    },
                    price: {
                        if self.section_updater.section_price.is_empty() {
                            self.error_message = Some("Field can't be empty".to_string());
                            return None;
                        }
                        if let Ok(val) = self.section_updater.section_price.parse() {
                            val
                        } else {
                            self.error_message = Some("Error fetching dashboard data".to_string());
                            return None;
                        }
                    },
                    is_magnet: {
                        if self.section_updater.section_is_magnet.is_empty() {
                            self.error_message = Some("Field can't be empty".to_string());
                            return None;
                        }
                        if let Ok(val) = self.section_updater.section_is_magnet.parse() {
                            val
                        } else {
                            self.error_message = Some("Error fetching dashboard data".to_string());
                            return None;
                        }
                    },
                    material_sides: {
                        if self.section_updater.section_material_sides.is_empty() {
                            self.error_message = Some("Field can't be empty".to_string());
                            return None;
                        }
                        if let Ok(val) = self.section_updater.section_material_sides.parse() {
                            val
                        } else {
                            self.error_message = Some("Error fetching dashboard data".to_string());
                            return None;
                        }
                    },
                    radius: {
                        if self.section_updater.section_radius.is_empty() {
                            self.error_message = Some("Field can't be empty".to_string());
                            return None;
                        }
                        if let Ok(val) = self.section_updater.section_radius.parse() {
                            val
                        } else {
                            self.error_message = Some("Error fetching dashboard data".to_string());
                            return None;
                        }
                    },
                    angle: {
                        if self.section_updater.section_angle.is_empty() {
                            self.error_message = Some("Field can't be empty".to_string());
                            return None;
                        }
                        if let Ok(val) = self.section_updater.section_angle.parse() {
                            val
                        } else {
                            self.error_message = Some("Error fetching dashboard data".to_string());
                            return None;
                        }
                    },
                    chains: {
                        if self.section_updater.section_type.is_empty() {
                            self.error_message = Some("Field can't be empty".to_string());
                            return None;
                        } else {
                            let ids = self.section_updater.section_chains.split(",");
                            let mut vec = Vec::new();
                            for i in ids {
                                if let Ok(i) = i.parse::<isize>() {
                                    vec.push(i);
                                } else {
                                    self.error_message =
                                        Some("Error fetching dashboard data".to_string());
                                    return None;
                                }
                            }
                            self.chains
                                .clone()
                                .into_iter()
                                .filter(|sec| vec.contains(&sec.id))
                                .collect::<Vec<Chain>>()
                        }
                    },
                })
            }
            UpdateStatus::Change => {
                let section = if !self.section_updater.section_id.is_empty() {
                    if let Ok(id) = self.section_updater.section_id.parse::<isize>() {
                        let rs = self
                            .sections
                            .clone()
                            .into_iter()
                            .filter(|sec| sec.id == id)
                            .collect::<Vec<Section>>();
                        if !rs.is_empty() {
                            rs.first().unwrap().clone()
                        } else {
                            self.error_message = Some("incorrect state".to_string());
                            return None;
                        }
                    } else {
                        self.error_message =
                            Some("Error fetching dashboard data - number expected".to_string());
                        return None;
                    }
                } else {
                    self.error_message = Some("incorrect state".to_string());
                    return None;
                };

                Some(Section {
                    id: section.id,
                    section_type: {
                        if self.section_updater.section_type.is_empty() {
                            section.section_type.clone()
                        } else {
                            if let Ok(val) = self.section_updater.section_type.parse() {
                                val
                            } else {
                                self.error_message =
                                    Some("Error fetching dashboard data".to_string());
                                return None;
                            }
                        }
                    },
                    width: {
                        if self.section_updater.section_width.is_empty() {
                            section.width.clone()
                        } else {
                            if let Ok(val) = self.section_updater.section_width.parse() {
                                val
                            } else {
                                self.error_message =
                                    Some("Error fetching dashboard data".to_string());
                                return None;
                            }
                        }
                    },
                    length: {
                        if self.section_updater.section_lenght.is_empty() {
                            section.length.clone()
                        } else {
                            if let Ok(val) = self.section_updater.section_lenght.parse() {
                                val
                            } else {
                                self.error_message =
                                    Some("Error fetching dashboard data".to_string());
                                return None;
                            }
                        }
                    },
                    price: {
                        if self.section_updater.section_price.is_empty() {
                            section.price.clone()
                        } else {
                            if let Ok(val) = self.section_updater.section_price.parse() {
                                val
                            } else {
                                self.error_message =
                                    Some("Error fetching dashboard data".to_string());
                                return None;
                            }
                        }
                    },
                    is_magnet: {
                        if self.section_updater.section_is_magnet.is_empty() {
                            section.is_magnet.clone()
                        } else {
                            if let Ok(val) = self.section_updater.section_is_magnet.parse() {
                                val
                            } else {
                                self.error_message =
                                    Some("Error fetching dashboard data".to_string());
                                return None;
                            }
                        }
                    },
                    material_sides: {
                        if self.section_updater.section_material_sides.is_empty() {
                            section.material_sides.clone()
                        } else {
                            if let Ok(val) = self.section_updater.section_material_sides.parse() {
                                val
                            } else {
                                self.error_message =
                                    Some("Error fetching dashboard data".to_string());
                                return None;
                            }
                        }
                    },
                    radius: {
                        if self.section_updater.section_radius.is_empty() {
                            section.radius.clone()
                        } else {
                            if let Ok(val) = self.section_updater.section_radius.parse() {
                                val
                            } else {
                                self.error_message =
                                    Some("Error fetching dashboard data".to_string());
                                return None;
                            }
                        }
                    },
                    angle: {
                        if self.section_updater.section_angle.is_empty() {
                            section.angle.clone()
                        } else {
                            if let Ok(val) = self.section_updater.section_angle.parse() {
                                val
                            } else {
                                self.error_message =
                                    Some("Error fetching dashboard data".to_string());
                                return None;
                            }
                        }
                    },
                    chains: if self.section_updater.section_type.is_empty() {
                        section.chains.clone()
                    } else {
                        let ids = self.section_updater.section_chains.split(",");
                        let mut vec = Vec::new();
                        for i in ids {
                            if let Ok(i) = i.parse::<isize>() {
                                vec.push(i);
                            } else {
                                self.error_message =
                                    Some("Error fetching dashboard data".to_string());
                                return None;
                            }
                        }
                        self.chains
                            .clone()
                            .into_iter()
                            .filter(|sec| vec.contains(&sec.id))
                            .collect::<Vec<Chain>>()
                    },
                })
            }
            _ => {
                self.error_message = Some("incorrect state".to_string());
                None
            }
        }
    }

    fn parse_input_chain(&mut self, update_status: UpdateStatus) -> Option<Chain> {
        match update_status {
            UpdateStatus::Add => {
                Some(Chain {
                    //because default
                    id: -1,
                    chain_type: {
                        if self.chain_updater.r#type.is_empty() {
                            self.error_message = Some("Field can't be empty".to_string());
                            return None;
                        }
                        if let Ok(val) = self.chain_updater.r#type.parse() {
                            val
                        } else {
                            self.error_message = Some("Error fetching dashboard data".to_string());
                            return None;
                        }
                    },
                    material: {
                        if self.chain_updater.material.is_empty() {
                            self.error_message = Some("Field can't be empty".to_string());
                            return None;
                        }
                        if let Ok(val) = self.chain_updater.material.parse() {
                            val
                        } else {
                            self.error_message = Some("Error fetching dashboard data".to_string());
                            return None;
                        }
                    },
                    width: {
                        if self.chain_updater.width.is_empty() {
                            self.error_message = Some("Field can't be empty".to_string());
                            return None;
                        }
                        if let Ok(val) = self.chain_updater.width.parse() {
                            val
                        } else {
                            self.error_message = Some("Error fetching dashboard data".to_string());
                            return None;
                        }
                    },
                    price: {
                        if self.chain_updater.price.is_empty() {
                            self.error_message = Some("Field can't be empty".to_string());
                            return None;
                        }
                        if let Ok(val) = self.chain_updater.price.parse() {
                            val
                        } else {
                            self.error_message = Some("Error fetching dashboard data".to_string());
                            return None;
                        }
                    },
                    is_magnet: {
                        if self.chain_updater.is_magnet.is_empty() {
                            self.error_message = Some("Field can't be empty".to_string());
                            return None;
                        }
                        if let Ok(val) = self.chain_updater.is_magnet.parse() {
                            val
                        } else {
                            self.error_message = Some("Error fetching dashboard data".to_string());
                            return None;
                        }
                    },
                    name: self.chain_updater.name.clone(),
                })
            }
            UpdateStatus::Change => {
                let chain = if !self.chain_updater.id.is_empty() {
                    if let Ok(id) = self.chain_updater.id.parse::<isize>() {
                        let rs = self
                            .chains
                            .clone()
                            .into_iter()
                            .filter(|sec| sec.id == id)
                            .collect::<Vec<Chain>>();
                        if !rs.is_empty() {
                            rs.first().unwrap().clone()
                        } else {
                            self.error_message = Some("incorrect state".to_string());
                            return None;
                        }
                    } else {
                        self.error_message =
                            Some("Error fetching dashboard data - number expected".to_string());
                        return None;
                    }
                } else {
                    self.error_message = Some("incorrect state".to_string());
                    return None;
                };

                Some(Chain {
                    id: chain.id,
                    chain_type: {
                        if self.chain_updater.r#type.is_empty() {
                            chain.chain_type.clone()
                        } else {
                            if let Ok(val) = self.chain_updater.r#type.parse() {
                                val
                            } else {
                                self.error_message =
                                    Some("Error fetching dashboard data".to_string());
                                return None;
                            }
                        }
                    },
                    width: {
                        if self.chain_updater.width.is_empty() {
                            chain.width.clone()
                        } else {
                            if let Ok(val) = self.chain_updater.width.parse() {
                                val
                            } else {
                                self.error_message =
                                    Some("Error fetching dashboard data".to_string());
                                return None;
                            }
                        }
                    },
                    material: {
                        if self.chain_updater.material.is_empty() {
                            chain.material.clone()
                        } else {
                            if let Ok(val) = self.chain_updater.material.parse() {
                                val
                            } else {
                                self.error_message =
                                    Some("Error fetching dashboard data".to_string());
                                return None;
                            }
                        }
                    },
                    price: {
                        if self.chain_updater.price.is_empty() {
                            chain.price.clone()
                        } else {
                            if let Ok(val) = self.chain_updater.price.parse() {
                                val
                            } else {
                                self.error_message =
                                    Some("Error fetching dashboard data".to_string());
                                return None;
                            }
                        }
                    },
                    is_magnet: {
                        if self.chain_updater.is_magnet.is_empty() {
                            chain.is_magnet.clone()
                        } else {
                            if let Ok(val) = self.chain_updater.is_magnet.parse() {
                                val
                            } else {
                                self.error_message =
                                    Some("Error fetching dashboard data".to_string());
                                return None;
                            }
                        }
                    },

                    name: {
                        if self.chain_updater.name.is_empty() {
                            chain.name.clone()
                        } else {
                            self.chain_updater.name.clone()
                        }
                    },
                })
            }
            _ => {
                self.error_message = Some("incorrect state".to_string());
                None
            }
        }
    }

    fn parse_input_user(&mut self, update_status: UpdateStatus) -> Option<User> {
        match update_status {
            UpdateStatus::Add => {
                Some(User {
                    //because default
                    id: -1,
                    hash: {
                        if self.user_updater.hash.is_empty() {
                            self.error_message = Some("Field can't be empty".to_string());
                            return None;
                        }
                        self.user_updater.hash.clone()
                    },
                    name: {
                        if self.user_updater.name.is_empty() {
                            self.error_message = Some("Field can't be empty".to_string());
                            return None;
                        }
                        self.user_updater.name.clone()
                    },
                    email: {
                        if self.user_updater.email.is_empty() {
                            self.error_message = Some("Field can't be empty".to_string());
                            return None;
                        }
                        self.user_updater.email.clone()
                    },
                    phone: {
                        if self.user_updater.phone.is_empty() {
                            self.error_message = Some("Field can't be empty".to_string());
                            return None;
                        }
                        self.user_updater.phone.clone()
                    },
                    level: {
                        if self.user_updater.hash.is_empty() {
                            self.error_message = Some("Field can't be empty".to_string());
                            return None;
                        }
                        if let Ok(value) = self.user_updater.level.parse() {
                            value
                        } else {
                            self.error_message = Some("Error fetching dashboard data".to_string());
                            return None;
                        }
                    },
                })
            }
            UpdateStatus::Change => {
                let user = if !self.user_updater.id.is_empty() {
                    if let Ok(id) = self.user_updater.id.parse::<isize>() {
                        let rs = self
                            .users
                            .clone()
                            .into_iter()
                            .filter(|sec| sec.id == id)
                            .collect::<Vec<User>>();
                        if !rs.is_empty() {
                            rs.first().unwrap().clone()
                        } else {
                            self.error_message = Some("incorrect state".to_string());
                            return None;
                        }
                    } else {
                        self.error_message =
                            Some("Error fetching dashboard data - number expected".to_string());
                        return None;
                    }
                } else {
                    self.error_message = Some("incorrect state".to_string());
                    return None;
                };

                Some(User {
                    id: user.id,
                    hash: {
                        if self.user_updater.phone.is_empty() {
                            user.hash.clone()
                        } else {
                            self.user_updater.hash.clone()
                        }
                    },
                    name: {
                        if self.user_updater.name.is_empty() {
                            user.name.clone()
                        } else {
                            self.user_updater.name.clone()
                        }
                    },
                    email: {
                        if self.user_updater.email.is_empty() {
                            user.email.clone()
                        } else {
                            self.user_updater.email.clone()
                        }
                    },
                    phone: {
                        if self.user_updater.phone.is_empty() {
                            user.phone.clone()
                        } else {
                            self.user_updater.phone.clone()
                        }
                    },
                    level: {
                        if self.user_updater.level.is_empty() {
                            user.level.clone()
                        } else {
                            if let Ok(value) = self.user_updater.level.parse() {
                                value
                            } else {
                                self.error_message =
                                    Some("Error fetching dashboard data".to_string());
                                return None;
                            }
                        }
                    },
                })
            }
            _ => {
                self.error_message = Some("incorrect state".to_string());
                None
            }
        }
    }

    fn render_dashboard_ui(&mut self, ui: &mut Ui) {
        ui.heading("Dashboard");

        if let Some(msg) = &self.error_message {
            ui.label(egui::RichText::new(msg).color(egui::Color32::RED));
        }
        // Section Table
        {
            // Sections Table()
            ui.add_space(10.0);
            ui.heading("Sections");
            egui::Grid::new("sections_grid")
                .striped(true)
                .min_col_width(100.0)
                .show(ui, |ui| {
                    ui.strong("Id");
                    ui.strong("Type");
                    ui.strong("Width");
                    ui.strong("Length");
                    ui.strong("Price");
                    ui.strong("Is Magnet");
                    ui.strong("Material Sides");
                    ui.strong("Radius");
                    ui.strong("Angle");
                    ui.strong("Chains");
                    ui.end_row();

                    let chains = {
                        let mut vec = Vec::new();
                        for i in self.chains.clone() {
                            vec.push(i.id);
                        }
                        vec
                    };
                    for section in &self.sections {
                        ui.label(section.id.to_string());
                        ui.label(format!("{:?}", section.section_type));
                        ui.label(section.width.to_string());
                        ui.label(section.length.to_string());
                        ui.label(section.price.to_string());
                        ui.label(section.is_magnet.to_string());
                        ui.label(format!("{:?}", section.material_sides));
                        ui.label(section.radius.to_string());
                        ui.label(section.angle.to_string());
                        ui.label(format!("{:?}", chains));
                        ui.end_row();
                    }
                });

            // Create modal (must be before buttons to allow calling open() from click handler)
            let modal = Modal::new(ui.ctx(), "Sections");

            if ui.button("Update").clicked() {
                self.section_updater.section_mode = UpdateStatus::Change;
                self.section_updater.win_open = true;
                modal.open();
            }
            if ui.button("Add").clicked() {
                self.section_updater.section_mode = UpdateStatus::Add;
                self.section_updater.win_open = true;
                modal.open();
            }

            // Show modal
            modal.show(|ui| {
                ui.add_space(10.0);
                ui.heading("Sections");
                egui::Grid::new("sections_grid")
                    .striped(true)
                    .min_col_width(100.0)
                    .show(ui, |ui| {
                        if self.section_updater.section_mode == UpdateStatus::Change {
                            ui.strong("Id");
                        }
                        ui.strong("Type");
                        ui.strong("Width");
                        ui.strong("Length");
                        ui.strong("Price");
                        ui.strong("Is Magnet");
                        ui.strong("Material Sides");
                        ui.strong("Radius");
                        ui.strong("Angle");
                        ui.strong("Chains");
                        ui.end_row();
                        if self.section_updater.section_mode == UpdateStatus::Change {
                            ui.text_edit_singleline(&mut self.section_updater.section_id);
                        }
                        ui.add(TextEdit::singleline(&mut self.section_updater.section_type));
                        ui.add(TextEdit::singleline(
                            &mut self.section_updater.section_width,
                        ));
                        ui.add(TextEdit::singleline(
                            &mut self.section_updater.section_lenght,
                        ));
                        ui.add(TextEdit::singleline(
                            &mut self.section_updater.section_price,
                        ));
                        ui.add(TextEdit::singleline(
                            &mut self.section_updater.section_is_magnet,
                        ));
                        ui.add(TextEdit::singleline(
                            &mut self.section_updater.section_material_sides,
                        ));
                        ui.add(TextEdit::singleline(
                            &mut self.section_updater.section_radius,
                        ));
                        ui.add(TextEdit::singleline(
                            &mut self.section_updater.section_angle,
                        ));
                        ui.add(TextEdit::singleline(
                            &mut self.section_updater.section_chains,
                        ));
                        ui.end_row();
                    });
                ui.add_space(10.0);
                if ui.button("Close").clicked() {
                    self.section_updater.win_open = false;
                    self.section_updater.section_mode = UpdateStatus::None;
                    modal.close();
                }
                ui.add_space(10.0);
                if ui.button("Send").clicked() {
                    match self.section_updater.section_mode {
                        UpdateStatus::None => {}
                        UpdateStatus::Add => self.send_add_section(),
                        UpdateStatus::Change => self.send_change_section(),
                    };
                    self.section_updater.win_open = false;
                    self.section_updater.section_mode = UpdateStatus::None;
                    modal.close();
                }
            });
        }
        {
            // Chains Table
            ui.add_space(20.0);
            ui.heading("Chains");
            egui::Grid::new("chains_grid")
                .striped(true)
                .min_col_width(100.0)
                .show(ui, |ui| {
                    ui.strong("Id");
                    ui.strong("Type");
                    ui.strong("Material");
                    ui.strong("Width");
                    ui.strong("Price");
                    ui.strong("Is Magnet");
                    ui.strong("Name");
                    ui.end_row();

                    for chain in &self.chains {
                        ui.label(chain.id.to_string());
                        ui.label(format!("{:?}", chain.chain_type));
                        ui.label(format!("{:?}", chain.material));
                        ui.label(chain.width.to_string());
                        ui.label(chain.price.to_string());
                        ui.label(chain.is_magnet.to_string());
                        ui.label(&chain.name);
                        ui.end_row();
                    }
                });

            // Create modal (must be before buttons to allow calling open() from click handler)
            let modal = Modal::new(ui.ctx(), "Chains");

            if ui.button("Update").clicked() {
                self.chain_updater.section_mode = UpdateStatus::Change;
                self.chain_updater.win_open = true;
                modal.open();
            }
            if ui.button("Add").clicked() {
                self.chain_updater.section_mode = UpdateStatus::Add;
                self.chain_updater.win_open = true;
                modal.open();
            }

            // Show modal
            modal.show(|ui| {
                ui.add_space(10.0);
                ui.heading("Chains");
                egui::Grid::new("chains_grid")
                    .striped(true)
                    .min_col_width(100.0)
                    .show(ui, |ui| {
                        if self.chain_updater.section_mode == UpdateStatus::Change {
                            ui.strong("Id");
                        }
                        ui.strong("Type");
                        ui.strong("Width");
                        ui.strong("Material");
                        ui.strong("Price");
                        ui.strong("Is Magnet");
                        ui.strong("Name");
                        ui.end_row();
                        if self.chain_updater.section_mode == UpdateStatus::Change {
                            ui.text_edit_singleline(&mut self.chain_updater.id);
                        }
                        ui.add(TextEdit::singleline(&mut self.chain_updater.r#type));
                        ui.add(TextEdit::singleline(&mut self.chain_updater.width));
                        ui.add(TextEdit::singleline(&mut self.chain_updater.material));
                        ui.add(TextEdit::singleline(&mut self.chain_updater.price));
                        ui.add(TextEdit::singleline(&mut self.chain_updater.is_magnet));
                        ui.add(TextEdit::singleline(&mut self.chain_updater.name));
                        ui.end_row();
                    });
                ui.add_space(10.0);
                if ui.button("Close").clicked() {
                    self.chain_updater.win_open = false;
                    self.chain_updater.section_mode = UpdateStatus::None;
                    modal.close();
                }
                ui.add_space(10.0);
                if ui.button("Send").clicked() {
                    match self.chain_updater.section_mode {
                        UpdateStatus::None => {}
                        UpdateStatus::Add => self.send_add_chain(),
                        UpdateStatus::Change => self.send_change_chain(),
                    };
                    self.chain_updater.win_open = false;
                    self.chain_updater.section_mode = UpdateStatus::None;
                    modal.close();
                }
            });
        }

        {
            // Users Table
            ui.add_space(20.0);
            ui.heading("Users");
            egui::Grid::new("users_grid")
                .striped(true)
                .min_col_width(100.0)
                .show(ui, |ui| {
                    ui.strong("Id");
                    ui.strong("Password");
                    ui.strong("Name");
                    ui.strong("Email");
                    ui.strong("Phone");
                    ui.strong("Level");
                    ui.end_row();

                    for user in &self.users {
                        ui.label(user.id.to_string());
                        ui.label(user.hash.clone());
                        ui.label(user.name.clone());
                        ui.label(user.email.clone());
                        ui.label(user.phone.clone());
                        ui.label(format!("{:?}", user.level));
                        ui.end_row();
                    }
                });

            // Create modal (must be before buttons to allow calling open() from click handler)
            let modal = Modal::new(ui.ctx(), "Users");

            if ui.button("Update").clicked() {
                self.user_updater.section_mode = UpdateStatus::Change;
                self.user_updater.win_open = true;
                modal.open();
            }
            if ui.button("Add").clicked() {
                self.user_updater.section_mode = UpdateStatus::Add;
                self.user_updater.win_open = true;
                modal.open();
            }

            // Show modal
            modal.show(|ui| {
                ui.add_space(10.0);
                ui.heading("Users");
                egui::Grid::new("users_grid")
                    .striped(true)
                    .min_col_width(100.0)
                    .show(ui, |ui| {
                        if self.user_updater.section_mode == UpdateStatus::Change {
                            ui.strong("Id");
                        }
                        ui.strong("Password");
                        ui.strong("Name");
                        ui.strong("Email");
                        ui.strong("Phone");
                        ui.strong("Level");
                        ui.end_row();
                        if self.user_updater.section_mode == UpdateStatus::Change {
                            ui.text_edit_singleline(&mut self.user_updater.id);
                        }
                        ui.add(TextEdit::singleline(&mut self.user_updater.hash));
                        ui.add(TextEdit::singleline(&mut self.user_updater.name));
                        ui.add(TextEdit::singleline(&mut self.user_updater.email));
                        ui.add(TextEdit::singleline(&mut self.user_updater.phone));
                        ui.add(TextEdit::singleline(&mut self.user_updater.level));
                        ui.end_row();
                    });
                ui.add_space(10.0);
                if ui.button("Close").clicked() {
                    self.user_updater.win_open = false;
                    self.user_updater.section_mode = UpdateStatus::None;
                    modal.close();
                }
                ui.add_space(10.0);
                if ui.button("Send").clicked() {
                    match self.user_updater.section_mode {
                        UpdateStatus::None => {}
                        UpdateStatus::Add => self.send_add_user(),
                        UpdateStatus::Change => self.send_change_user(),
                    };
                    self.user_updater.win_open = false;
                    self.user_updater.section_mode = UpdateStatus::None;
                    modal.close();
                }
            });
        }
    }
}

impl App for TemplateApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| match self.app_state {
            AppState::Login => self.render_login_ui(ui),
            AppState::Dashboard => self.render_dashboard_ui(ui),
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

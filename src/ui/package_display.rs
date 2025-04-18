//TODO: IMPLEMENT WIDGET WHERE USER CAN SEE AND MANIPULATE A SELECTED PACKAGE
use iced::{
    Task,
    widget::{button, column, row, text},
};
use std::sync::{Arc, Mutex};

use crate::{
    AppMessage,
    logic::{package::Package, server::Server},
};

use super::package_button::PackageCardMessage;

#[derive(Debug, Clone)]
pub enum PackageViewMessage {
    Install(Arc<Mutex<Package>>),
    Uninstall(Arc<Mutex<Package>>),
    Update(Arc<Mutex<Package>>),
    SystemUpdate,
    Finished(String, Arc<Mutex<Package>>),
    FinishedSystemUpdate(String),
}

#[derive(Debug, Clone)]
pub struct PackageDisplay {
    pub server: Arc<Mutex<Server>>,
    pub package: Option<Arc<Mutex<Package>>>,
    pub loading: bool,
}

impl PackageDisplay {
    fn handle_operation(&self, operation: PackageViewMessage) -> String {
        let package_name = self
            .clone()
            .package
            .unwrap()
            .lock()
            .unwrap()
            .get_property("Name".to_string())
            .unwrap_or_default();
        if package_name.len() == 0 {
            return "".to_string();
        }
        return match operation {
            PackageViewMessage::Install(p) | PackageViewMessage::Update(p) => {
                Package::install_or_update(package_name)
            }
            PackageViewMessage::Uninstall(p) => Package::uninstall(package_name),
            _ => {
                unreachable!()
            }
        };
    }

    pub fn update(&mut self, message: AppMessage) -> Task<AppMessage> {
        match message {
            AppMessage::PackageCardMessage(PackageCardMessage::Selected(p)) => {
                self.package = Some(p);
                self.package.as_mut().unwrap().lock().unwrap().sync_all();
                iced::Task::none()
            }
            AppMessage::PackageViewMessage(m) => match m {
                PackageViewMessage::Update(_)
                | PackageViewMessage::Install(_)
                | PackageViewMessage::Uninstall(_) => {
                    self.loading = true;
                    let this = self.clone();
                    let package = this.package.clone().unwrap();
                    iced::Task::perform(
                        async move { this.clone().handle_operation(m.clone()) },
                        move |installed| {
                            AppMessage::PackageViewMessage(PackageViewMessage::Finished(
                                installed,
                                package.clone(),
                            ))
                        },
                    )
                }
                PackageViewMessage::Finished(stderror, package) => {
                	if stderror.len() != 0 {
                 		let _ = native_dialog::MessageDialog::new().set_text(&stderror).set_title("An error has ocurred :(").show_alert();
                 	} else {
                  		let _ = native_dialog::MessageDialog::new().set_title("Operation finished succesfully").set_text("No errors were reported").show_alert();
                  	}
                    package.lock().unwrap().sync_installed();
                    package.lock().unwrap().sync_all();
                    self.loading = false;
                    Task::none()
                }
                PackageViewMessage::SystemUpdate => {
                    self.loading = true;
                    let this = self.clone();

                    return iced::Task::perform(
                        async move {
                            this.server.lock().unwrap().clone().system_update()
                        },
                        |stderror| {
                            AppMessage::PackageViewMessage(PackageViewMessage::FinishedSystemUpdate(stderror))
                        },
                    );
                }
                PackageViewMessage::FinishedSystemUpdate(stderror) => {
               		if stderror.len() != 0 {
                		let _ = native_dialog::MessageDialog::new().set_text(&stderror).set_title("An error has ocurred :(").show_alert();
                	} else {
                 		let _ = native_dialog::MessageDialog::new().set_title("Update finished succesfully").set_text("No errors were reported").show_alert();
                 	}
                    self.loading = false;
                    Task::none()
                }
            },
            _ => iced::Task::none(),
        }
    }

    pub fn view(&self) -> iced::widget::Column<AppMessage> {
        // if self.package.is_none() {
        //     return column![text("No package selected")];
        // }

        let def_package = &Arc::new(Mutex::new(Package::default()));
        let package_lock = self.package.as_ref().unwrap_or(def_package).lock().unwrap();

        let installed = package_lock
            .get_property("Installed".to_string())
            .unwrap_or_default()
            == "True".to_string();

        let install_button = button(if installed { "Uninstall" } else { "Install" })
            .on_press_maybe(if self.loading == false && self.package.is_some() {
                if installed {
                    Some(AppMessage::PackageViewMessage(
                        PackageViewMessage::Uninstall(self.package.clone().unwrap_or_default()),
                    ))
                } else {
                    Some(AppMessage::PackageViewMessage(PackageViewMessage::Install(
                        self.package.clone().unwrap_or_default(),
                    )))
                }
            } else {
                None
            });

        let update_button =
            button("Update").on_press_maybe(if self.loading == false && self.package.is_some() {
                Some(AppMessage::PackageViewMessage(PackageViewMessage::Update(
                    self.package.clone().unwrap_or_default(),
                )))
            } else {
                None
            });

        let system_update = button("Full Update").on_press(AppMessage::PackageViewMessage(PackageViewMessage::SystemUpdate));

        let spinner = if self.loading {
            iced::Element::from(iced_aw::Spinner::new())
        } else {
            iced::Element::new(iced::widget::horizontal_space())
        };

        return column![
            row![
                text("Name: "),
                text(
                    package_lock
                        .get_property("Name".to_string())
                        .unwrap_or_default(),
                ),
            ],
            row![
                text("Description: "),
                text(
                    package_lock
                        .get_property("Description".to_string())
                        .unwrap_or_default(),
                ),
            ],
            row![
                text("Installed: "),
                text(
                    package_lock
                        .get_property("Installed".to_string())
                        .unwrap_or_default(),
                ),
            ],
            row![
                text("Installed Date: "),
                text(
                    package_lock
                        .get_property("Install Date".to_string())
                        .unwrap_or("Not installed".to_string()),
                ),
            ],
            row![
                text("Installed Size: "),
                text(
                    package_lock
                        .get_property("Installed Size".to_string())
                        .unwrap_or("Not installed".to_string()),
                ),
            ],
            row![
                text("Version: "),
                text(
                    package_lock
                        .get_property("Version".to_string())
                        .unwrap_or_default()
                )
            ],
            row![install_button, update_button, system_update, spinner].spacing(10),
        ]
        .spacing(20)
        .width(iced::Length::Fill);
    }
}

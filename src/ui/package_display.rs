//TODO: IMPLEMENT WIDGET WHERE USER CAN SEE AND MANIPULATE A SELECTED PACKAGE
use iced::widget::{button, column, row, text};
use std::sync::{Arc, Mutex};

use crate::{
    AppMessage,
    logic::{package::Package, server::Server},
};

use super::package_button::PackageCardMessage;

#[derive(Debug, Clone)]
pub struct PackageDisplay {
    pub server: Arc<Mutex<Server>>,
    pub package: Option<Arc<Mutex<Package>>>,
}

impl PackageDisplay {
    pub fn update(&mut self, message: AppMessage) {
        match message {
            AppMessage::PackageCardMessage(PackageCardMessage::Selected(p)) => {
                self.package = Some(p);
            }
            _ => {}
        }
    }

    pub fn view(&self) -> iced::widget::Column<AppMessage> {
        if self.package.is_none() {
            return column![text("No package selected")];
        }

        let package_lock = self.package.as_ref().unwrap().lock().unwrap();

        let installed = package_lock
            .get_property("Installed".to_string())
            .unwrap_or_default() == "True".to_string();

        let install_button = button(
        	if installed {"Uninstall"} else {"Install"}
        );

        let update_button = button("Update");

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
             	text(package_lock.get_property("Version".to_string()).unwrap_or_default())
            ],
            row![install_button, update_button].spacing(10)
        ]
        .spacing(20)
        .width(iced::Length::Fill);
    }
}

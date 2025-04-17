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
    Finished(bool, Arc<Mutex<Package>>),
}

#[derive(Debug, Clone)]
pub struct PackageDisplay {
    pub server: Arc<Mutex<Server>>,
    pub package: Option<Arc<Mutex<Package>>>,
}

impl PackageDisplay {
    fn handle_operation(&self, operation: PackageViewMessage) -> bool {
        return match operation {
            PackageViewMessage::Install(p) | PackageViewMessage::Update(p) => {
                p.clone().lock().unwrap().install_or_update()
            }
            PackageViewMessage::Uninstall(p) => p.clone().lock().unwrap().uninstall(),
            _ => {
                unreachable!()
            }
        };
    }

    pub fn update(&mut self, message: AppMessage) -> Task<AppMessage> {
        match message {
            AppMessage::PackageCardMessage(PackageCardMessage::Selected(p)) => {
                self.package = Some(p);
                iced::Task::none()
            }
            AppMessage::PackageViewMessage(m) => match m {
                PackageViewMessage::Update(_)
                | PackageViewMessage::Install(_)
                | PackageViewMessage::Uninstall(_) => {
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
                PackageViewMessage::Finished(_, package) => {
                	package.lock().unwrap().sync_installed();
                    Task::none()
                }
            },
            _ => iced::Task::none(),
        }
    }

    pub fn view(&self) -> iced::widget::Column<AppMessage> {
        if self.package.is_none() {
            return column![text("No package selected")];
        }

        let package_lock = self.package.as_ref().unwrap().lock().unwrap();

        let installed = package_lock
            .get_property("Installed".to_string())
            .unwrap_or_default()
            == "True".to_string();

        let install_button =
            button(if installed { "Uninstall" } else { "Install" }).on_press(if installed {
                AppMessage::PackageViewMessage(PackageViewMessage::Uninstall(
                    self.package.clone().unwrap_or_default(),
                ))
            } else {
                AppMessage::PackageViewMessage(PackageViewMessage::Install(
                    self.package.clone().unwrap_or_default(),
                ))
            });

        let update_button = button("Update").on_press(AppMessage::PackageViewMessage(
            PackageViewMessage::Update(self.package.clone().unwrap_or_default()),
        ));

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
            row![install_button, update_button].spacing(10)
        ]
        .spacing(20)
        .width(iced::Length::Fill);
    }
}

use std::sync::{Arc, Mutex};

use crate::{AppMessage, logic::package::Package};
use iced::widget::{button, row};
use iced_aw::{Badge, style};

#[derive(Clone, Debug)]
pub struct PackageButton {
    pub package: Arc<Mutex<Package>>,
}

#[derive(Clone, Debug)]
pub enum PackageCardMessage {
    Selected(Arc<Mutex<Package>>),
}

impl PackageButton {
    fn update(&mut self, message: AppMessage) {}

    pub fn view(&self) -> iced::widget::Button<'static, AppMessage> {
        let name = iced::widget::text(
            self.package
                .lock()
                .unwrap()
                .get_property("Name".to_string())
                .unwrap_or_default()
                .to_string(),
        );

        let installed = self
            .package
            .lock()
            .unwrap()
            .get_property("Installed".to_string())
            .unwrap()
            == "True".to_string();

        let icon: Badge<AppMessage> = iced_aw::badge(if installed {
            "Installed"
        } else {
            "Not installed"
        })
        .align_x(iced::Alignment::End)
        .style(if installed {style::badge::success} else {style::badge::warning});

        return button(row![name, iced::widget::horizontal_space().width(iced::Length::Fill), icon].spacing(10).padding(5))
            .width(iced::Length::Fill)
            .on_press(AppMessage::PackageCardMessage(
                PackageCardMessage::Selected(self.package.clone()),
            ));
    }
}

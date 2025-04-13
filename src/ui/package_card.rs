use std::{cell::RefCell, rc::Rc, sync::Arc};

use iced::widget::{Row, button, row, text};

use crate::{AppMessage, logic::package::Package};

#[derive(Clone, Debug)]
pub struct PackageCard {
    pub package: Arc<RefCell<Package>>,
}

#[derive(Clone, Debug)]
pub enum PackageCardMessage {
    Selected(String),
}

impl PackageCard {
    fn update(&mut self, message: AppMessage) {}

    pub fn view(&self) -> iced::widget::Button<'static, AppMessage> {
        let name = iced::widget::text(
            self.package
                .borrow()
                .get_property("Name".to_string())
                .unwrap_or_default()
                .to_string(),
        );

        return button(row![name].spacing(10).padding(5))
            .width(iced::Length::Fill)
            .on_press(AppMessage::PackageListMessage(
                PackageCardMessage::Selected(self.package.borrow().get_property("Name".to_string()).unwrap_or_default()),
            ));
    }
}

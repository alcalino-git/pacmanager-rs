use std::{cell::RefCell, rc::Rc};

use iced::widget::{Row, row, text};

use crate::{AppMessage, logic::package::Package};

#[derive(Clone, Debug)]
pub struct PackageCard {
    pub package: Rc<RefCell<Package>>,
}

impl PackageCard {
    fn update(&mut self, message: AppMessage) {}

    pub fn view(&self) -> iced::Element<'static, AppMessage> {
        let name = iced::widget::text(
            self.package
                .borrow()
                .get_property("Name".to_string())
                .unwrap_or_default()
                .to_string()
        );

        return row![name].spacing(10).padding(5).into();
    }
}

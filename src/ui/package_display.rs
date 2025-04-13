//TODO: IMPLEMENT WIDGET WHERE USER CAN SEE AND MANIPULATE A SELECTED PACKAGE
use std::sync::{Arc, Mutex};
use iced::widget::{column, text};

use crate::{logic::{package::Package, server::Server}, AppMessage};

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
			},
			_ => {}
		}
	}

	pub fn view(&self) -> iced::widget::Column<AppMessage> {

		if self.package.is_none() {
			return column![
				text("No package selected")
			]
		}

		let package_lock = self.package.as_ref().unwrap().lock().unwrap();

		return column![
			text(package_lock.get_property("Name".to_string()).unwrap()),
			text(package_lock.get_property("Description".to_string()).unwrap()),
			text(package_lock.get_property("Installed".to_string()).unwrap()),
		].spacing(20);
	}
}

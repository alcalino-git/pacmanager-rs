use iced::widget::{Column, column};
use std::cell::RefCell;
use std::rc::Rc;

use crate::AppMessage;
use crate::logic::server::Server;
use crate::ui::package_card::PackageCard;

#[derive(Default, Debug, Clone)]
pub struct SearchWidget {
    pub server: Rc<RefCell<Server>>,
    pub search: String,
    pub packages: Vec<PackageCard>,
}

#[derive(Debug, Clone)]
pub enum SearchMessage {
    SearchChanged(String),
    SearchSubmited
}

impl SearchWidget {
    pub fn update(&mut self, message: AppMessage) {
        match message {
            AppMessage::SearchMessage(m) => match m {
                SearchMessage::SearchChanged(s) => self.search = s,
                SearchMessage::SearchSubmited => {
	                self.packages = self
	                    .server
	                    .borrow()
	                    .search(self.search.clone())
	                    .into_iter()
	                    .map(|x|  PackageCard {package: x.clone()} )
	                    .collect();
                }
            },
        }
    }

    pub fn view(&self) -> Column<AppMessage> {

    	let packages = column(self.packages.clone().into_iter().map(|x| x.view()).collect::<Vec<_>>());

        column![
            iced::widget::text_input("search", &self.search)
                .on_input(|x| AppMessage::SearchMessage(SearchMessage::SearchChanged(x)))
               	.on_submit(AppMessage::SearchMessage(SearchMessage::SearchSubmited)),
            packages
        ]
    }
}

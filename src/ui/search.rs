use iced::widget::{Column, Scrollable, column, scrollable};
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

use crate::AppMessage;
use crate::logic::server::Server;
use crate::ui::package_card::PackageCard;

const PAGE_SIZE: usize = 100;

#[derive(Default, Debug, Clone)]
pub struct SearchWidget {
    pub server: Arc<RefCell<Server>>,
    pub search: String,
    pub packages: Vec<PackageCard>,
}

#[derive(Debug, Clone)]
pub enum SearchMessage {
    SearchChanged(String),
    SearchSubmited,
}

impl SearchWidget {
	pub fn handle_search(&self) -> Vec<PackageCard> {
		let packages: Vec<PackageCard> = self
	                    .server
	                    .borrow()
	                    .search(self.search.clone())
	                    .into_iter()
	                    .map(|x| PackageCard { package: x.clone() })
	                    .collect();
		return packages
	}

    pub fn update(&mut self, message: AppMessage) {
        match message {
            AppMessage::SearchMessage(m) => match m {
                SearchMessage::SearchChanged(s) => self.search = s,
                SearchMessage::SearchSubmited => {
                    self.packages = self.handle_search()
                }
            },
            _ => {}
        }
    }

    pub fn view(&self) -> Column<AppMessage> {
        let packages = scrollable(column(
            self.packages
                .clone()
                .into_iter()
                .map(|x| x.view())
                .map(|x| iced::Element::from(x))
                .collect::<Vec<_>>(),
        ).spacing(10)).width(iced::Length::Fill);
        //TODO: Implement paging so that app doesn't slow down

        column![
            iced::widget::text_input("search", &self.search)
                .on_input(|x| AppMessage::SearchMessage(SearchMessage::SearchChanged(x)))
                .on_submit(AppMessage::SearchMessage(SearchMessage::SearchSubmited)),
            packages
        ]
    }
}

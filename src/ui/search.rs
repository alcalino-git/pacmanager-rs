use iced::Task;
use iced::widget::{Column, Scrollable, column, scrollable};
use iced_aw::{Spinner, spinner};
use std::cell::RefCell;
use std::process::Command;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

use crate::AppMessage;
use crate::logic::server::Server;
use crate::ui::package_card::PackageCard;

const PAGE_SIZE: usize = 100;

#[derive(Default, Debug, Clone)]
pub struct SearchWidget {
    pub server: Arc<Mutex<Server>>,
    pub search: String,
    pub packages: Vec<PackageCard>,
    pub loading: bool,
}

#[derive(Debug, Clone)]
pub enum SearchMessage {
    SearchChanged(String),
    SearchSubmited,
    SearchFinished(Vec<PackageCard>),
}

impl SearchWidget {
    pub fn handle_search(&self) -> Vec<PackageCard> {
        let packages: Vec<PackageCard> = self
            .server
            .lock()
            .unwrap()
            .search(self.search.clone())
            .into_iter()
            .map(|x| PackageCard { package: x.clone() })
            .collect();
        return packages;
    }

    pub fn update(&mut self, message: AppMessage) -> Task<AppMessage> {
        match message {
            AppMessage::SearchMessage(m) => match m {
                SearchMessage::SearchChanged(s) => {
                    self.search = s;
                    iced::Task::none()
                }
                SearchMessage::SearchSubmited => {
                    let this = self.clone();
                    self.loading = true;
                    self.packages.clear();
                    println!("Search started");
                    return iced::Task::perform(async move { this.handle_search() }, |p| {
                        AppMessage::SearchMessage(SearchMessage::SearchFinished(p))
                    });
                }
                SearchMessage::SearchFinished(packages) => {
                    println!("Search finished. Rendering...");
                    self.packages = packages;
                    self.loading = false;
                    iced::Task::none()
                }
            },
            _ => iced::Task::none(),
        }
    }

    pub fn view(&self) -> Column<AppMessage> {
        let packages = scrollable(
            column(
                self.packages[0..std::cmp::min(100, self.packages.len())]
                    .to_vec()
                    .clone()
                    .into_iter()
                    .map(|x| x.view())
                    .map(|x| iced::Element::from(x))
                    .collect::<Vec<_>>(),
            )
            .spacing(10),
        )
        .width(iced::Length::Fill);

        let packages_display: iced::Element<AppMessage> = if !self.loading {
            iced::Element::from(packages)
        } else {
            iced::Element::from(
                Spinner::new()
                    .width(iced::Length::Fill)
                    .height(iced::Length::Fill).circle_radius(20.0),
            )
        };

        //TODO: Implement paging so that app doesn't slow down

        column![
            iced::widget::text_input("search", &self.search)
                .on_input(|x| AppMessage::SearchMessage(SearchMessage::SearchChanged(x)))
                .on_submit(AppMessage::SearchMessage(SearchMessage::SearchSubmited)),
            packages_display
        ]
    }
}

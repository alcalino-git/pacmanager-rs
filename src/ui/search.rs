use iced::Task;
use iced::widget::{Column, Row, Scrollable, button, column, row, scrollable, Text, text};
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
    pub page: i32,
}

#[derive(Debug, Clone)]
pub enum SearchMessage {
    SearchChanged(String),
    SearchSubmited,
    SearchFinished(Vec<PackageCard>),
    PageUp,
    PageDown,
}

impl SearchWidget {
    fn get_total_pages(&self) -> i32 {
        return (self.packages.len() / PAGE_SIZE) as i32;
    }

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
                    self.page = 0;
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
                SearchMessage::PageUp => {
                    self.page += 1;
                    if self.page > self.get_total_pages() {
                        self.page = 0
                    };
                    Task::none()
                }
                SearchMessage::PageDown => {
                    self.page -= 1;
                    if self.page < 0 {
                        self.page = self.get_total_pages()
                    };
                    Task::none()
                }
            },
            _ => iced::Task::none(),
        }
    }

    pub fn view(&self) -> Column<AppMessage> {
        let packages = scrollable(
            column(
                self.packages[std::cmp::min((self.page as usize * PAGE_SIZE), self.packages.len())
                    ..std::cmp::min(((self.page + 1) as usize * PAGE_SIZE), self.packages.len())]
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
                    .height(iced::Length::Fill)
                    .circle_radius(20.0),
            )
        };

        //TODO: Implement paging so that app doesn't slow down

        column![
            row![
                iced::widget::text_input("search", &self.search)
                    .on_input(|x| AppMessage::SearchMessage(SearchMessage::SearchChanged(x)))
                    .on_submit(AppMessage::SearchMessage(SearchMessage::SearchSubmited)),
                button("<").on_press(AppMessage::SearchMessage(SearchMessage::PageDown)),
                text(format!("{}/{}", self.page, self.get_total_pages())),
                button(">").on_press(AppMessage::SearchMessage(SearchMessage::PageUp)),
            ],
            packages_display
        ].spacing(10)
    }
}

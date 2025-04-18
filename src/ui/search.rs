use iced::Task;
use iced::widget::{Column, Row, Scrollable, Text, button, column, row, scrollable, text};
use iced_aw::{SelectionList, Spinner, spinner};
use std::cell::RefCell;
use std::process::Command;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

use crate::logic::package::Package;
use crate::AppMessage;
use crate::logic::server::Server;
use crate::ui::package_button::PackageButton;

const PAGE_SIZE: usize = 100;

#[derive(Default, Debug, Clone)]
pub struct SearchWidget {
    pub server: Arc<Mutex<Server>>,
    pub search: String,
    pub packages: Vec<PackageButton>,
    pub loading: bool,
    pub page: i32,
    pub filter: FilterState,
    pub sorter: SorterState,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Copy)]
pub enum SorterState {
    #[default]
    Default,
    InstallSize,
    InstallDate,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Copy)]
pub enum FilterState {
    #[default]
    All,
    Installed,
    NotInstalled,
}

#[derive(Debug, Clone,)]
pub enum SearchMessage {
    SearchChanged(String),
    SearchSubmited,
    SearchFinished(Vec<PackageButton>),
    FilterChanged(FilterState),
    SorterChanged(SorterState),
    PageUp,
    PageDown,
}

impl SearchWidget {
    fn get_total_pages(&self) -> i32 {
        return (self.packages.len() / PAGE_SIZE) as i32;
    }

    pub fn handle_search(&self) -> Vec<PackageButton> {
        let mut packages: Vec<Arc<Mutex<Package>>> = self
            .server
            .lock()
            .unwrap()
            .search(self.search.clone())
            .into_iter()
            .filter(|x| {match self.filter {
            	FilterState::All => true,
             	FilterState::Installed => x.lock().unwrap().get_property("Installed".to_string()).unwrap() == "True".to_string(),
              	FilterState::NotInstalled => x.lock().unwrap().get_property("Installed".to_string()).unwrap() != "True".to_string()
            }}).collect();

        packages.sort_by(|a,b| {
        	match self.sorter {
         		SorterState::Default => std::cmp::Ordering::Equal,
           		SorterState::InstallSize => b.lock().unwrap().get_install_size().partial_cmp(&a.lock().unwrap().get_install_size()).unwrap_or(std::cmp::Ordering::Equal),
             	SorterState::InstallDate => b.lock().unwrap().get_installed_date().cmp(&a.lock().unwrap().get_installed_date()),
         	}
        });

        let packages_widgets = packages.into_iter().map(|x| PackageButton { package: x.clone() }).collect();

        return packages_widgets
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
                },

                SearchMessage::FilterChanged(s) => {
               		self.filter = s;
                 	self.update(AppMessage::SearchMessage(SearchMessage::SearchSubmited))
                },
                SearchMessage::SorterChanged(s) => {
                	self.sorter = s;
                	self.update(AppMessage::SearchMessage(SearchMessage::SearchSubmited))
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

        let filter_selector = row![
        	iced::widget::radio("All", FilterState::All, Some(self.filter), |state| AppMessage::SearchMessage(SearchMessage::FilterChanged(state))),
         	iced::widget::radio("Installed", FilterState::Installed, Some(self.filter), |state| AppMessage::SearchMessage(SearchMessage::FilterChanged(state))),
          	iced::widget::radio("Not Installed", FilterState::NotInstalled, Some(self.filter), |state| AppMessage::SearchMessage(SearchMessage::FilterChanged(state)))
        ].spacing(5);

        let sorter_selector = row![
        	iced::widget::radio("Default", SorterState::Default, Some(self.sorter), |state| AppMessage::SearchMessage(SearchMessage::SorterChanged(state))),
         	iced::widget::radio("Size", SorterState::InstallSize, Some(self.sorter), |state| AppMessage::SearchMessage(SearchMessage::SorterChanged(state))),
         	iced::widget::radio("Date", SorterState::InstallDate, Some(self.sorter), |state| AppMessage::SearchMessage(SearchMessage::SorterChanged(state))),
        ].spacing(5);

        column![
            row![
                iced::widget::text_input("search", &self.search)
                    .on_input(|x| AppMessage::SearchMessage(SearchMessage::SearchChanged(x)))
                    .on_submit(AppMessage::SearchMessage(SearchMessage::SearchSubmited)),
                button("<").on_press(AppMessage::SearchMessage(SearchMessage::PageDown)),
                text(format!("{}/{}", self.page, self.get_total_pages())),
                button(">").on_press(AppMessage::SearchMessage(SearchMessage::PageUp)),
            ],
            column![row![text("Filter by: "),filter_selector], row![text("Sort by: "), sorter_selector]].spacing(5),
            packages_display
        ]
        .spacing(10)
    }
}

use std::sync::{Arc, Mutex};

mod logic;
mod ui;

use iced::{
    widget::row, Application, Task
};
use logic::server::Server;
use ui::{
    package_button::PackageCardMessage,
    package_display::{PackageDisplay, PackageViewMessage},
    search::{SearchMessage, SearchWidget},
};

#[derive(Debug, Clone)]
enum AppMessage {
    SearchMessage(SearchMessage),
    PackageCardMessage(PackageCardMessage),
    PackageViewMessage(PackageViewMessage),
    ForceUpdate
}

#[derive(Clone, Debug)]
struct MainUI {
    server: Arc<Mutex<Server>>,
    search: SearchWidget,
    view: PackageDisplay,
}

impl Default for MainUI {
    fn default() -> Self {
        let server = Server::intialized().populate().check_installed();
        let val = Self {
            server: Arc::new(Mutex::new(server.clone())),
            search: SearchWidget {
                server: Arc::new(Mutex::new(server.clone())),
                ..Default::default()
            },
            view: PackageDisplay {
                server: Arc::new(Mutex::new(server.clone())),
                package: None,
                loading: false
            },
        };
        return val;
    }
}

impl MainUI {
	fn update(&mut self, message: AppMessage) -> Task<AppMessage> {
		let theme_task = iced::Task::perform(async move {
			theme
		},  |_| AppMessage::ForceUpdate);

        Task::batch(vec![self.view.update(message.clone()), self.search.update(message.clone()), theme_task])

    }

    fn view(&self) -> iced::widget::Row<AppMessage> {
        return row![
            self.search.view().width(iced::Length::Fill),
            self.view.view().width(iced::Length::Fill)
        ]
        .padding(20)
        .spacing(20);
    }
}

fn main() -> iced::Result {
    //TODO: Use mutex instead of RefCell
    let app = iced::application("Pacmanager", MainUI::update, MainUI::view).theme(theme);


    let state = MainUI::default();

    app.run_with( || (state, Task::batch(vec![Task::done(AppMessage::SearchMessage(SearchMessage::SearchSubmited))]) ))
}

fn theme(state: &MainUI) -> iced::Theme {
	match dark_light::detect().unwrap_or(dark_light::Mode::Light) {
		dark_light::Mode::Light => iced::Theme::Light,
		dark_light::Mode::Dark => iced::Theme::Dark,
		dark_light::Mode::Unspecified => iced::Theme::Light
	}
}

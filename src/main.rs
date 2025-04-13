use std::{
    cell::RefCell,
    rc::Rc,
    sync::{Arc, Mutex},
};

mod logic;
mod ui;

use iced::{
    Task,
    widget::{column, row},
};
use logic::{package::Package, server::Server};
use ui::{
    package_button::PackageCardMessage,
    package_display::PackageDisplay,
    search::{SearchMessage, SearchWidget},
};

#[derive(Debug, Clone)]
enum AppMessage {
    SearchMessage(SearchMessage),
    PackageCardMessage(PackageCardMessage),
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
        return Self {
            server: Arc::new(Mutex::new(server.clone())),
            search: SearchWidget {
                server: Arc::new(Mutex::new(server.clone())),
                ..Default::default()
            },
            view: PackageDisplay {
                server: Arc::new(Mutex::new(server.clone())),
                package: None,
            },
        };
    }
}

impl MainUI {
    fn update(&mut self, message: AppMessage) -> Task<AppMessage> {
    	self.view.update(message.clone());
        Task::none()
        	.chain(self.search.update(message.clone()))
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
    iced::application("Pacmanager", MainUI::update, MainUI::view).run()
}

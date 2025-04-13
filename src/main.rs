use std::{cell::RefCell, rc::Rc, sync::{Arc, Mutex}};

mod logic;
mod ui;

use iced::{widget::column, Task};
use logic::server::Server;
use ui::{package_button::PackageCardMessage, search::{SearchMessage, SearchWidget}};

#[derive(Debug, Clone)]
enum AppMessage {
    SearchMessage(SearchMessage),
    PackageListMessage(PackageCardMessage)
}

#[derive(Clone, Debug)]
struct MainUI {
    server: Arc<Mutex<Server>>,
    search: SearchWidget,
}

impl Default for MainUI {
    fn default() -> Self {
        let server = Server::intialized().populate().check_installed();
        return Self {
            server: Arc::new(Mutex::new(server.clone())),
            search: SearchWidget {server: Arc::new(Mutex::new(server.clone())), ..Default::default()}
        };
    }
}

impl MainUI {
    fn update(&mut self, message: AppMessage) -> Task<AppMessage> {
        self.search.update(message)
    }

    fn view(&self) -> iced::widget::Column<AppMessage> {
        return column![self.search.view()].padding(20);
    }
}

fn main() -> iced::Result {
	//TODO: Use mutex instead of RefCell
    iced::application("Pacmanager", MainUI::update, MainUI::view).run()
}

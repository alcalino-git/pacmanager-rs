use std::{cell::RefCell, rc::Rc};

mod logic;
mod ui;

use iced::widget::column;
use logic::server::Server;
use ui::{package_card::PackageCardMessage, search::{SearchMessage, SearchWidget}};

#[derive(Debug, Clone)]
enum AppMessage {
    SearchMessage(SearchMessage),
    PackageListMessage(PackageCardMessage)
}

#[derive(Clone, Debug)]
struct MainUI {
    server: Rc<RefCell<Server>>,
    search: SearchWidget,
}

impl Default for MainUI {
    fn default() -> Self {
        let server = Server::intialized().populate().check_installed();
        return Self {
            server: Rc::new(RefCell::new(server.clone())),
            search: SearchWidget {server: Rc::new(RefCell::new(server.clone())), ..Default::default()}
        };
    }
}

impl MainUI {
    fn update(&mut self, message: AppMessage) {
        self.search.update(message);
    }

    fn view(&self) -> iced::widget::Column<AppMessage> {
        return column![self.search.view()].padding(20);
    }
}

fn main() -> iced::Result {
	//TODO: Use Arc instead of RC
    iced::application("Pacmanager", MainUI::update, MainUI::view).run()
}

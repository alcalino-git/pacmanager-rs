use rust_fuzzy_search::fuzzy_compare;

use crate::logic::package::Package;
use std::{cell::RefCell, collections::HashMap, process::Command, rc::Rc};

#[derive(Debug, Clone, Default)]
pub struct Server {
    packages: HashMap<String, Rc<RefCell<Package>>>,
}

impl Server {
    //Returns a fully initialized version of `Server`
    pub fn intialized() -> Server {
        return Server::default().populate().check_installed();
    }

    pub fn check_installed(&mut self) -> Server {
        let installed =
            String::from_utf8(Command::new("pacman").arg("-Q").output().unwrap().stdout).unwrap();

        for line in installed.split("\n") {
            let name = line.split(" ").collect::<Vec<&str>>()[0].to_string();
            self.get_package(name)
                .unwrap()
                .borrow_mut()
                .set_property("Installed".to_string(), "True".to_string());
        }

        for p in self.packages.keys() {
            let package = self.get_package(p.clone()).unwrap().clone();
            if package
                .borrow()
                .get_property("Installed".to_string())
                .is_none()
            {
                package
                    .borrow_mut()
                    .set_property("Installed".to_string(), "False".to_string());
            }
        }

        return self.clone();
    }

    pub fn populate(&mut self) -> Server {
        let mut pacman =
            String::from_utf8(Command::new("pacman").arg("-Si").output().unwrap().stdout).unwrap();
        pacman += "\n";
        pacman +=
            &String::from_utf8(Command::new("pacman").arg("-Qi").output().unwrap().stdout).unwrap();

        let mut packages_raw: Vec<Vec<String>> = vec![];
        let mut curr_package: Vec<String> = vec![];

        for line in pacman.split("\n") {
            if line.len() == 0 {
                packages_raw.push(curr_package.clone());
                curr_package.clear();
                continue;
            }
            curr_package.push(line.to_string());
        }

        let mut packages = HashMap::new();

        for p in packages_raw {
            let new_package = Package::from_raw(p);
            packages.insert(
                new_package
                    .get_property("Name".to_string())
                    .unwrap_or_default(),
                Rc::new(RefCell::new(new_package)),
            );
        }

        self.packages = packages.clone();

        return self.clone();
    }

    pub fn get_package(&self, name: String) -> Option<Rc<RefCell<Package>>> {
        match self.packages.get(name.clone().trim()) {
            Some(p) => Some(p.clone()),
            None => None,
        }
    }

    pub fn search(&self, query: String) -> Vec<Rc<RefCell<Package>>> {
    	println!("Querying database against: {}\n Server has {} packages", query, self.packages.keys().len());
        let mut result = self
            .packages
            .keys()
            .into_iter()
            .map(|k| self.get_package(k.to_string()).unwrap().clone())
            .filter(|p| fuzzy_compare(&p.borrow().get_property("Name".to_string()).unwrap_or_default(), &query) > 0.10)
            .collect::<Vec<_>>();

        result.sort_by(|a, b| {
        	let a_score = fuzzy_compare(&a.borrow().get_property("Name".to_string()).unwrap_or_default(), &query);
        	let b_score = fuzzy_compare(&b.borrow().get_property("Name".to_string()).unwrap_or_default(), &query);

         	return b_score.total_cmp(&a_score.clone());
        });

        return result;
    }
}

#[test]
fn test_server() {
    let server = Server::intialized();
    //println!("{:?}", server.packages);
    assert!(server.packages.len() > 10);

    //99% chance linux is installed on linux amairight?
    assert!(server.get_package("linux".to_string()).is_some());

    let linux = server.get_package("kseexpr".to_string()).unwrap();
    assert!(
        linux
            .borrow()
            .get_property("Installed".to_string())
            .unwrap()
            == "False".to_string()
    );
}

use rust_fuzzy_search::fuzzy_compare;

use crate::logic::package::Package;
use std::{
    collections::HashMap,
    process::Command,
    sync::{Arc, Mutex},
};

#[derive(Debug, Clone, Default)]
pub struct Server {
    packages: HashMap<String, Arc<Mutex<Package>>>,
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
            if name.len() == 0 {
                continue;
            }
            self.get_package(name)
                .unwrap()
                .lock()
                .unwrap()
                .set_property("Installed".to_string(), "True".to_string());
        }

        for p in self.packages.keys() {
            let package = self.get_package(p.clone()).unwrap().clone();
            if package
                .lock()
                .unwrap()
                .get_property("Installed".to_string())
                .is_none()
            {
                package
                    .lock()
                    .unwrap()
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
                if curr_package.len() > 0 {
                    packages_raw.push(curr_package.clone())
                };
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
                Arc::new(Mutex::new(new_package)),
            );
        }

        self.packages = packages.clone();

        return self.clone();
    }

    pub fn get_package(&self, name: String) -> Option<Arc<Mutex<Package>>> {
        match self.packages.get(name.clone().trim()) {
            Some(p) => Some(p.clone()),
            None => None,
        }
    }

    pub fn search(&self, query: String) -> Vec<Arc<Mutex<Package>>> {
        println!(
            "Querying database against: \"{}\"\n Server has {} packages",
            query,
            self.packages.keys().len()
        );

        let pacman_search = String::from_utf8(
            std::process::Command::new("pacman")
                .arg("-Ss")
                .arg(query)
                .output()
                .unwrap()
                .stdout,
        )
        .unwrap();

        //Its best to move this outside the function to avoid deadlocks
        let result = pacman_search
            .split("\n")
            .clone()
            .into_iter()
            .filter(|x| x.contains("/"))
            .map(|x| x.split("/").collect::<Vec<_>>()[1].split(" ").collect::<Vec<_>>()[0])
            .filter(|x| self.get_package(x.to_string()).is_some())
            .map(|x| self.get_package(x.to_string()).unwrap())
            .collect::<Vec<Arc<Mutex<Package>>>>();

        return result;
    }

    pub fn system_update(&mut self) -> String {
        let command = std::process::Command::new("sh")
            .arg("-c")
            .arg("pkexec pacman -Syu --noconfirm")
            .output();
        return String::from_utf8(command.unwrap().stderr).unwrap()
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
            .lock()
            .unwrap()
            .get_property("Installed".to_string())
            .unwrap()
            == "False".to_string()
    );
}

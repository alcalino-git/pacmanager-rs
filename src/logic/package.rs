use std::{cell::RefCell, collections::HashMap, fmt::format, process::{Command, ExitStatus}, rc::Rc};

use iced::advanced::graphics::text::cosmic_text::rustybuzz::script::COMMON;

#[derive(Debug, Clone, Default)]
pub struct Package {
    properties: HashMap<String, String>,
}

impl Package {
    pub fn get_property(&self, prop: String) -> Option<String> {
        match self.properties.get(&prop) {
            Some(s) => return Some(s.clone()),
            None => return None,
        };
    }

    pub fn set_property(&mut self, prop: String, value: String) {
        self.properties.insert(prop, value);
    }

    pub fn from_raw(data: Vec<String>) -> Package {
        let mut curr_prop = "".to_string();
        let mut curr_val = "".to_string();

        let mut props: HashMap<String, String> = HashMap::new();

        for line in data {
            if line.contains(":") {
                if curr_prop.len() > 0 {
                    props.insert(curr_prop, curr_val);
                }
                let parts = line.trim().split(":").collect::<Vec<_>>();
                curr_prop = parts[0].trim().to_string();
                curr_val = parts[1].trim().to_string();
            } else {
                curr_val += line.trim();
            }
        }

        return Package { properties: props };
    }

    pub fn install_or_update(&self) -> bool {
    	println!("Attempting to update or install {}", self.get_property("Name".to_string()).unwrap_or_default());
     	let installed = self.get_property("Installed".to_string()).unwrap() == "True".to_string();

        if (self.get_property("Name".to_string())).is_none() {
            return installed;
        }
        let payload = format!("pkexec pacman -Syy {} --noconfirm", self.get_property("Name".to_string()).unwrap_or_default());

        let command = std::process::Command::new("sh")
            .arg("-c")
            .arg(payload)
            .output();
        return if command.is_ok() {true} else {installed}
    }

    pub fn uninstall(&self) -> bool {
    	println!("Attempting to uninstall {}", self.get_property("Name".to_string()).unwrap_or_default());
     	let installed = self.get_property("Installed".to_string()).unwrap() == "True".to_string();
        if (self.get_property("Name".to_string())).is_none() {
            return installed;
        }

        let payload = format!("pkexec pacman -R {} --noconfirm", self.get_property("Name".to_string()).unwrap_or_default());

        let command = std::process::Command::new("sh")
            .arg("-c")
            .arg(payload)
            .output();
        return if command.is_ok() {false} else {installed};
    }

    //Makes a call to the OS package manager to sync the in-memory package with the real one
    pub fn sync_installed(&mut self)  {
    	let name = self.get_property("Name".to_string()).unwrap_or_default();
    	println!("Veryfying installation state of {}", name.clone());

    	let output = Command::new("pacman").arg("-Q").arg(name.clone()).output().unwrap().status.success();
     	println!("Recieved state: {}", output);

     	self.set_property("Installed".to_string(), if output {"True".to_string()} else {"False".to_string()});
    }
}

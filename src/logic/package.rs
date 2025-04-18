use std::{collections::HashMap, process::Command};

use chrono::NaiveDateTime;

#[derive(Debug, Clone, Default)]
pub struct Package {
    properties: HashMap<String, String>,
    installed_size: Option<f64>,
    installed_date: Option<NaiveDateTime>
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
                let split = line.find(":").unwrap();
                let parts = vec![&line[0..split], &line[split+1..]];
                curr_prop = parts[0].trim().to_string();
                curr_val = parts[1].trim().to_string();
            } else {
                curr_val += line.trim();
            }
        }

        return Package { properties: props, ..Default::default() };
    }

    pub fn install_or_update(name: String) -> bool {
    	println!("Attempting to update or install {}", name);

        let payload = format!("pkexec pacman -Syy {} --noconfirm", name);

        let command = std::process::Command::new("sh")
            .arg("-c")
            .arg(payload)
            .output();
        return true
    }

    pub fn uninstall(name: String) -> bool {
    	println!("Attempting to uninstall {}", name);

        let payload = format!("pkexec pacman -R {} --noconfirm", name);

        let command = std::process::Command::new("sh")
            .arg("-c")
            .arg(payload)
            .output();
        return true;
    }

    //Makes a call to the OS package manager to sync the in-memory package with the real one
    pub fn sync_installed(&mut self)  {
    	let name = self.get_property("Name".to_string()).unwrap_or_default();
    	println!("Veryfying installation state of {}", name.clone());

    	let output = Command::new("pacman").arg("-Q").arg(name.clone()).output().unwrap().status.success();
     	println!("Recieved state: {}", output);

     	self.set_property("Installed".to_string(), if output {"True".to_string()} else {"False".to_string()});
    }

    pub fn get_install_size(&mut self) -> f64 {
    	//if self.installed_size.is_some() {return self.installed_size.unwrap()}
    	let size_raw = self.get_property("Installed Size".to_string()).unwrap_or_default();
     	if size_raw.trim().is_empty()  {return 0.0;}

      	let size_number =  size_raw.trim().split(" ").collect::<Vec<_>>()[0].trim();
      	let mut size = size_number.parse::<f64>().unwrap_or_default();
       	if size_raw.contains("MiB") {size *= 1024.0};

        self.installed_size = Some(size);
        return size;
    }

    pub fn get_installed_date(&mut self) -> NaiveDateTime  {
    	//if self.installed_date.is_some() {return self.installed_date.unwrap()}
    	let date_raw = self.get_property("Install Date".to_string()).unwrap_or_default();
     	if date_raw.trim().len() == 0 {return NaiveDateTime::default()}

      	let naive_dt = chrono::NaiveDateTime::parse_from_str(&date_raw[..date_raw.len()-4], "%a %d %b %Y %I:%M:%S %p").unwrap_or_default();

       	self.installed_date = Some(naive_dt);
       	return naive_dt
    }
}

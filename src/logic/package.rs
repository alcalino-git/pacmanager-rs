use std::{cell::RefCell, collections::HashMap, process::Command, rc::Rc};

#[derive(Debug, Clone)]
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
}

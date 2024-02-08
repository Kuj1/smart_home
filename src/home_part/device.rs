use std::{collections::HashMap, io};

use itertools::Itertools;

#[derive(Debug)]
#[allow(dead_code)]
pub struct SmartDevice {
    pub(crate) name: String,
    pub(crate) vendor_id: String,
    pub(crate) status_info: HashMap<String, String>,
}

impl SmartDevice {
    pub fn new(name: &str, vendor_id: &str) -> Self {
        Self {
            name: name.to_string(),
            vendor_id: vendor_id.to_string(),
            status_info: HashMap::new(),
        }
    }

    pub fn new_from_cli() -> Self {
        println!("Please enter you device name: ");
        let mut name = String::new();
        io::stdin().read_line(&mut name).expect("Can't read name");

        println!("Please enter you device vendor_id: ");
        let mut vendor_id: String = String::new();
        io::stdin().read_line(&mut vendor_id).expect("Can't read vendor_id");

        Self {
            name: name.replace("\n", ""),
            vendor_id: vendor_id.replace("\n", ""),
            status_info: HashMap::new(),
        }
    }

    pub fn request_for_status_info<'a>() -> Vec<(String, String)> {
        let mut status_info: Vec<(String, String)> = Vec::new();
        loop {
            println!("Please enter status name and this value (separator is 'space')");
            println!("If you want to end entering status info, enter 'exit':");
            let mut stats: String = String::new();
             
            io::stdin().read_line(&mut stats).expect("Can't read status info");
            if stats.replace("\n", "") == "exit".to_string() {
                break;
            }

            let x: (String, String)= stats.split(" ").map(|x| x.replace("\n", "").to_string()).collect_tuple().unwrap();
            status_info.push(x);

        }

        status_info
    }

    pub fn update_status_info(&mut self, status_info: Vec<(&str, &str)>) {
        for x in status_info {
            self.status_info.insert(x.0.to_string(), x.1.to_string());
        }
    }

    pub fn update_status_info_from_cli(&mut self, status_info: Vec<(String, String)>) {
        for x in status_info {
            self.status_info.insert(x.0, x.1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::SmartDevice;

    #[test]
    fn test_create_device() {
        let mut new_device = SmartDevice::new("Smart Socket", "WE23_134");
        let mut new_device_1 = SmartDevice::new("Smart Socket", "WE23_234");
        let stats: Vec<(&str, &str)> = vec![("voltage", "220"), ("Is_on", "true"), ("Power", "2A")];
        let stats_1: Vec<(&str, &str)> = vec![("voltage", "220"), ("Is_on", "false"), ("Power", "1A")];

        new_device.update_status_info(stats);
        new_device_1.update_status_info(stats_1);
    }
}

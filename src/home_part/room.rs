use std::{collections::HashMap, io};

use super::device::SmartDevice;

#[derive(Debug)]
#[allow(dead_code)]
pub struct Room<'a> {
    pub(crate) name: String,
    pub(crate) smart_devices: HashMap<String, &'a SmartDevice>,
}

impl<'a> Room<'a> {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            smart_devices: HashMap::new(),
        }
    }

    pub fn new_from_cli() -> Self {
        println!("Please enter room name: ");
        let mut room_name = String::new();
        io::stdin().read_line(&mut room_name).expect("Can't read room_name");
        Self { 
            name: room_name.trim().replace("\n", ""),
            smart_devices: HashMap::new(),
        }
    }

    pub fn append_room_device(&mut self, device: &'a SmartDevice) {
        self.smart_devices
            .insert(format!("{}/{}", device.name, device.vendor_id), device);
    }

    pub fn remove_device(&mut self, device: &str) {
        self.smart_devices.remove(device);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_create_room() {
        Room::new("Dinner");
    }

    #[test]
    fn test_append_room_device() {
        let mut new_device = SmartDevice::new("Smart Socket", "WE23_134");
        let mut new_device_1 = SmartDevice::new("Smart Socket", "WE23_234");
        let stats: Vec<(&str, &str)> = vec![("voltage", "220"), ("Is_on", "true"), ("Power", "2A")];
        let stats_1: Vec<(&str, &str)> = vec![("voltage", "220"), ("Is_on", "false"), ("Power", "1A")];

        new_device.update_status_info(stats);
        new_device_1.update_status_info(stats_1);

        let mut dinner = Room::new("Dinner");
        dinner.append_room_device(&new_device);
        dinner.append_room_device(&new_device_1);
    }

    #[test]
    fn test_remove_device() {
        let mut new_device = SmartDevice::new("Smart Socket", "WE23_134");
        let mut new_device_1 = SmartDevice::new("Smart Socket", "WE23_234");
        let stats: Vec<(&str, &str)> = vec![("voltage", "220"), ("Is_on", "true"), ("Power", "2A")];
        let stats_1: Vec<(&str, &str)> = vec![("voltage", "220"), ("Is_on", "false"), ("Power", "1A")];

        new_device.update_status_info(stats);
        new_device_1.update_status_info(stats_1);

        let mut dinner = Room::new("Dinner");
        dinner.append_room_device(&new_device);
        dinner.append_room_device(&new_device_1);

        dinner.remove_device("Smart Socket/WE23_134");

    }
}

use serde_json::to_string as hashmap_to_string;

use super::room::Room;

pub trait DeviceInfo {
    fn get_device_info(&self, room_name: &Room, device_name: &str) -> String;
}

impl DeviceInfo for String {
    fn get_device_info(&self, _room_name: &Room, _device_name: &str) -> String {
        self.to_string()
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct SmartHome<'a> {
    pub(crate) title: String,
    pub(crate) rooms: Vec<&'a Room<'a>>,
}

impl<'a> SmartHome<'a> {
    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_string(),
            rooms: Vec::new(),
        }
    }

    pub fn update_rooms(&mut self, room: &'a Room) {
        self.rooms.push(room);
    }

    pub fn get_rooms(&self) -> Vec<String> {
        self.rooms
            .iter()
            .map(|room| room.name.clone())
            .collect::<Vec<String>>()
    }

    pub fn create_report<D: DeviceInfo>(&'a self, device_info: &'a D) -> &D {
        device_info
    }
}

impl<'a> DeviceInfo for SmartHome<'a> {
    fn get_device_info(&self, room: &Room, device_name: &str) -> String {
        // todo!()
        // let devices = &room_name.smart_devices;
        for device in &room.smart_devices {
            if device.1.name == device_name.trim() {
                let dev_name = &device.1.name;
                let room_name = &room.name;
                let id = &device.1.vendor_id;
                let status = &hashmap_to_string(&device.1.status_info).unwrap();

                let device_info = format!(
                    "Title: {}\nRoom: {}\nVendor ID: {}\nStat's: {}",
                    dev_name, room_name, id, status
                );

                return device_info;
            }
        }

        panic!("Attempt to get device info, which dosent exist")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::home_part::device::SmartDevice;

    #[test]
    fn test_create_smarthome() {
        SmartHome::new("Smart Home");
    }

    #[test]
    fn test_update_rooms() {
        let mut smart_home = SmartHome::new("My Home");
        let mut new_device = SmartDevice::new("Device", "WE23_234");
        let mut new_device_1 = SmartDevice::new("Device 1", "WE243_234");

        let stats: Vec<(&str, &str)> = vec![("sss", "dfsd"), ("gsf", "sdf")];
        let stats_1: Vec<(&str, &str)> = vec![("ssdfss", "dfsd"), ("gsfdsf", "sdf")];

        new_device.update_status_info(stats);
        new_device_1.update_status_info(stats_1);

        let mut dinner = Room::new("Dinner");
        dinner.append_room_device(&new_device);
        dinner.append_room_device(&new_device_1);
        smart_home.update_rooms(&dinner);
    }

    #[test]
    fn test_get_rooms() {
        let mut smart_home = SmartHome::new("My Home");
        let mut new_device = SmartDevice::new("Device", "WE23_234");
        let mut new_device_1 = SmartDevice::new("Device 1", "WE243_234");

        let stats: Vec<(&str, &str)> = vec![("sss", "dfsd"), ("gsf", "sdf")];
        let stats_1: Vec<(&str, &str)> = vec![("ssdfss", "dfsd"), ("gsfdsf", "sdf")];

        new_device.update_status_info(stats);
        new_device_1.update_status_info(stats_1);

        let mut dinner = Room::new("Dinner");
        dinner.append_room_device(&new_device);
        dinner.append_room_device(&new_device_1);
        smart_home.update_rooms(&dinner);
        smart_home.get_rooms();
    }

    #[test]
    fn test_get_device_info() {
        let mut smart_home = SmartHome::new("My Home");
        let mut new_device = SmartDevice::new("Device", "WE23_234");
        let mut new_device_1 = SmartDevice::new("Device 1", "WE243_234");

        let stats: Vec<(&str, &str)> = vec![("sss", "dfsd"), ("gsf", "sdf")];
        let stats_1: Vec<(&str, &str)> = vec![("ssdfss", "dfsd"), ("gsfdsf", "sdf")];

        new_device.update_status_info(stats);
        new_device_1.update_status_info(stats_1);

        let mut dinner = Room::new("Dinner");
        dinner.append_room_device(&new_device);
        dinner.append_room_device(&new_device_1);
        smart_home.update_rooms(&dinner);
        smart_home.get_device_info(&dinner, "Device");
    }

    #[test]
    #[should_panic(expected = "Attempt to get device info, which dosent exist")]
    fn test_get_dont_ex_device_info() {
        let mut smart_home = SmartHome::new("My Home");
        let mut new_device = SmartDevice::new("Device", "WE23_234");
        let mut new_device_1 = SmartDevice::new("Device 1", "WE243_234");

        let stats: Vec<(&str, &str)> = vec![("sss", "dfsd"), ("gsf", "sdf")];
        let stats_1: Vec<(&str, &str)> = vec![("ssdfss", "dfsd"), ("gsfdsf", "sdf")];

        new_device.update_status_info(stats);
        new_device_1.update_status_info(stats_1);

        let mut dinner = Room::new("Dinner");
        dinner.append_room_device(&new_device);
        dinner.append_room_device(&new_device_1);
        smart_home.update_rooms(&dinner);
        smart_home.get_device_info(&dinner, "Smart Thermometr");
    }

    #[test]
    fn test_create_report() {
        let mut smart_home = SmartHome::new("My Home");
        let mut new_device = SmartDevice::new("Device", "WE23_234");
        let mut new_device_1 = SmartDevice::new("Device 1", "WE243_234");

        let stats: Vec<(&str, &str)> = vec![("sss", "dfsd"), ("gsf", "sdf")];
        let stats_1: Vec<(&str, &str)> = vec![("ssdfss", "dfsd"), ("gsfdsf", "sdf")];

        new_device.update_status_info(stats);
        new_device_1.update_status_info(stats_1);

        let mut dinner = Room::new("Dinner");
        dinner.append_room_device(&new_device);
        dinner.append_room_device(&new_device_1);
        smart_home.update_rooms(&dinner);
        let device_info = smart_home.get_device_info(&dinner, "Device");
        smart_home.create_report(&device_info);
    }
}

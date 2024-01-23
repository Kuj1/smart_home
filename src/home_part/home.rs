use serde_json::{de, to_string_pretty as hashmap_to_string};

use super::device::{self, SmartDevice};
use super::errors::CommonError;
use super::room::{self, Room};

pub trait DeviceInfo<'a> {
    fn get_device_info(&self, room: &'a Room, device_name: &str) -> Result<String, CommonError>;
    fn get_room_devices_info(&self, room: &'a Room) -> Result<String, CommonError>;
}

impl<'a> DeviceInfo<'a> for String {
    fn get_device_info(&self, room: &'a Room, device_name: &str) -> Result<String, CommonError> {
        let room_devices = &room.smart_devices;
        let device:Result<String, CommonError> = match room_devices.get(device_name) {
            Some(dev) => {
                let title = &dev.name;
                let id = &dev.vendor_id;
                let room_name = &room.name;
                let status = match !dev.status_info.is_empty() {
                    true => hashmap_to_string(&dev.status_info).unwrap(),
                    false => String::from("No status info")
                };

                let device_info = format!("Name: {}\nVendor ID: {}\nRoom: {}\nStat's: {}", title, id, room_name, status);
                Ok(device_info)
            },
            None => return Err(CommonError::DontExistDevice),
        };
        Ok(device.unwrap())
    }

    fn get_room_devices_info(&self, room: &'a Room) -> Result<String, CommonError> {
        todo!()
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

    pub fn get_rooms(&self) -> Result<Vec<String>, CommonError> {
        let rooms = match !self.rooms.is_empty() {
            true => {
                let rooms: Vec<String> = self.rooms.iter().map(|room| room.name.clone()).collect();
                Ok(rooms)
            }
            false => Err(CommonError::DontExistRoom),
        };

        rooms
    }

    pub fn create_report<D: DeviceInfo<'a>>(&self, device_info: &'a D) -> &'a D {
        device_info
    }
}

impl<'a> DeviceInfo<'a> for SmartHome<'a> {
    fn get_device_info(&self, room: &Room, device_name: &str) -> Result<String, CommonError> {
        let room_devices = &room.smart_devices;
        let device:Result<String, CommonError> = match room_devices.get(device_name) {
            Some(dev) => {
                let title = &dev.name;
                let id = &dev.vendor_id;
                let room_name = &room.name;
                let status = match !dev.status_info.is_empty() {
                    true => hashmap_to_string(&dev.status_info).unwrap(),
                    false => String::from("No status info")
                };

                let device_info = format!("\n[REPORT]\nName: {}\nVendor ID: {}\nRoom: {}\nStat's: {}", title, id, room_name, status);
                Ok(device_info)
            },
            None => return Err(CommonError::DontExistDevice),
        };
        Ok(device.unwrap())
    }

    fn get_room_devices_info(&self, room: &'a Room) -> Result<String, CommonError> {
        let room_devices: Result<String, CommonError> = match !room.smart_devices.is_empty() {
            true => {
                let mut devices = String::new();
                let mut count: i32 = 1;
                for device in &room.smart_devices {
                    let title = &device.0;
                    let id = &device.1.vendor_id;
                    let room_name = &room.name;
                    let status = match !device.1.status_info.is_empty() {
                        true => hashmap_to_string(&device.1.status_info).unwrap(),
                        false => String::from("No status info")
                    };

                    let device_info = format!("\n[REPORT#{}]\nName: {}\nVendor ID: {}\nRoom: {}\nStat's: {}\n", count, title, id, room_name, status);
                    devices.push_str(&device_info);
                    count += 1;
                }
                Ok(devices)
            }
            false => Err(CommonError::DontExistDevice)
        };

        Ok(room_devices.unwrap())
        
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
        smart_home.get_rooms().unwrap();
    }

    // #[test]
    // fn test_get_device_info() {
    //     let mut smart_home = SmartHome::new("My Home");
    //     let mut new_device = SmartDevice::new("Device", "WE23_234");
    //     let mut new_device_1 = SmartDevice::new("Device 1", "WE243_234");

    //     let stats: Vec<(&str, &str)> = vec![("sss", "dfsd"), ("gsf", "sdf")];
    //     let stats_1: Vec<(&str, &str)> = vec![("ssdfss", "dfsd"), ("gsfdsf", "sdf")];

    //     new_device.update_status_info(stats);
    //     new_device_1.update_status_info(stats_1);

    //     let mut dinner = Room::new("Dinner");
    //     dinner.append_room_device(&new_device);
    //     dinner.append_room_device(&new_device_1);
    //     smart_home.update_rooms(&dinner);
    //     let _ = smart_home.get_device_info(&dinner, "Device");
    // }

    // #[test]
    // #[should_panic(expected = "called `Result::unwrap()` on an `Err` value: DontExistDevice")]
    // fn test_get_dont_ex_device_info() {
    //     let mut smart_home = SmartHome::new("My Home");
    //     let mut new_device = SmartDevice::new("Device", "WE23_234");
    //     let mut new_device_1 = SmartDevice::new("Device 1", "WE243_234");

    //     let stats: Vec<(&str, &str)> = vec![("sss", "dfsd"), ("gsf", "sdf")];
    //     let stats_1: Vec<(&str, &str)> = vec![("ssdfss", "dfsd"), ("gsfdsf", "sdf")];

    //     new_device.update_status_info(stats);
    //     new_device_1.update_status_info(stats_1);

    //     let mut dinner = Room::new("Dinner");
    //     dinner.append_room_device(&new_device);
    //     dinner.append_room_device(&new_device_1);
    //     smart_home.update_rooms(&dinner);
    //     let _ = smart_home.get_device_info(&dinner, "Smart Thermometr");
    // }

    // #[test]
    // fn test_create_report() {
    //     let mut smart_home = SmartHome::new("My Home");
    //     let mut new_device = SmartDevice::new("Device", "WE23_234");
    //     let mut new_device_1 = SmartDevice::new("Device 1", "WE243_234");

    //     let stats: Vec<(&str, &str)> = vec![("sss", "dfsd"), ("gsf", "sdf")];
    //     let stats_1: Vec<(&str, &str)> = vec![("ssdfss", "dfsd"), ("gsfdsf", "sdf")];

    //     new_device.update_status_info(stats);
    //     new_device_1.update_status_info(stats_1);

    //     let mut dinner = Room::new("Dinner");
    //     dinner.append_room_device(&new_device);
    //     dinner.append_room_device(&new_device_1);
    //     smart_home.update_rooms(&dinner);
    //     let device_info = smart_home.get_device_info(&dinner, "Device").unwrap();
    //     smart_home.create_report(&device_info);
    // }
}

use serde_json::to_string as hashmap_to_string;

use super::device::SmartDevice;
use super::errors::CommonError;
use super::room::Room;

pub trait DeviceInfo<'a> {
    fn get_device_info(&self, room: &'a Room, device_name: &str) -> Result<String, CommonError>;
}

impl<'a> DeviceInfo<'a> for String {
    fn get_device_info(&self, room: &'a Room, device_name: &str) -> Result<String, CommonError> {
        let mut y = room.smart_devices.iter();
        let x = loop {
            let next = y.next().unwrap();

            if next.1.name == device_name.trim() {
                break *next.1;
            }
        };
        match x.name == device_name {
            true => device_name.to_string(),
            false => panic!("{:?}", CommonError::DontExistDevice),
        };
        let dev_name = &x.name;
        let room_name = &room.name;
        let id = &x.vendor_id;
        let status = match !x.status_info.is_empty() {
            true => hashmap_to_string(&x.status_info).unwrap(),
            false => panic!("{:?}", CommonError::DontExistInfo),
        };

        let device_info = format!(
            "Title: {}\nRoom: {}\nVendor ID: {}\nStat's: {}",
            dev_name, room_name, id, status
        );

        Ok(device_info)
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
        let mut y = room.smart_devices.iter();
        let x: Result<&SmartDevice, CommonError> = loop {
            let next = y.next();

            if next.unwrap().1.name == device_name.trim() {
                break Ok(next.unwrap().1);
            } else if next.unwrap().1.name != device_name.trim() {
                break Err(CommonError::DontExistDevice);
            }

            continue;
        };
        let device_x = x.unwrap();
        match device_x.name == device_name {
            true => {
                // device_name.to_string();
                let dev_name = &device_x.name;
                let room_name = &room.name;
                let id = &device_x.vendor_id;
                let status = match !device_x.status_info.is_empty() {
                    true => hashmap_to_string(&device_x.status_info).unwrap(),
                    false => panic!("{:?}", CommonError::DontExistInfo),
                };

                let device_info = format!(
                    "Title: {}\nRoom: {}\nVendor ID: {}\nStat's: {}",
                    dev_name, room_name, id, status
                );

                Ok(device_info)
            }

            false => Err(CommonError::DontExistDevice),
        }
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
        let _ = smart_home.get_device_info(&dinner, "Device");
    }

    #[test]
    #[should_panic(expected = "called `Result::unwrap()` on an `Err` value: DontExistDevice")]
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
        let _ = smart_home.get_device_info(&dinner, "Smart Thermometr");
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
        let device_info = smart_home.get_device_info(&dinner, "Device").unwrap();
        smart_home.create_report(&device_info);
    }
}

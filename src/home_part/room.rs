use std::collections::HashMap;

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

    pub fn append_room_device(&mut self, device: &'a SmartDevice) {
        self.smart_devices.insert(
            format!("{}/{}", device.name, device.vendor_id),
            device
        );
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
        let mut new_device = SmartDevice::new("Device", "WE23_234");
        let mut new_device_1 = SmartDevice::new("Device 1", "WE243_234");

        let stats: Vec<(&str, &str)> = vec![("sss", "dfsd"), ("gsf", "sdf")];
        let stats_1: Vec<(&str, &str)> = vec![("ssdfss", "dfsd"), ("gsfdsf", "sdf")];

        new_device.update_status_info(stats);
        new_device_1.update_status_info(stats_1);

        let mut dinner = Room::new("Dinner");
        dinner.append_room_device(&new_device);
        dinner.append_room_device(&new_device_1);
    }
}

use std::collections::HashMap;

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

    pub fn update_status_info(&mut self, status_info: Vec<(&str, &str)>) {
        for x in status_info {
            self.status_info.insert(x.0.to_string(), x.1.to_string());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::SmartDevice;

    // use super::*;
    #[test]
    fn test_create_device() {
        let mut new_device = SmartDevice::new("Device", "WE23_234");
        let mut new_device_1 = SmartDevice::new("Device 1", "WE243_234");

        let stats: Vec<(&str, &str)> = vec![("sss", "dfsd"), ("gsf", "sdf")];
        let stats_1: Vec<(&str, &str)> = vec![("ssdfss", "dfsd"), ("gsfdsf", "sdf")];

        new_device.update_status_info(stats);
        new_device_1.update_status_info(stats_1);
    }
}

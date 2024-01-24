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

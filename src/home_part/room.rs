use super::device::SmartDevice;

#[derive(Debug)]
#[allow(dead_code)]
pub struct Room {
    pub(crate) name: String,
    pub(crate) smart_devices: Vec<SmartDevice>,
}

impl Room {
    pub fn new(name: String) -> Self {
        Self {
            name,
            smart_devices: Vec::new(),
        }
    }

    pub fn append_room_device(
        &mut self,
        name_device: String,
        vendor_id_device: String,
    ) -> &mut Room {
        let room_name = self.name.to_string();

        let new_device = SmartDevice {
            name: name_device,
            room_name,
            vendor_id: vendor_id_device,
            status: false,
        };

        self.smart_devices.push(new_device);

        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_create_room() {
        Room::new("Dinner".to_string());
    }

    #[test]
    fn test_append_room_device() {
        let mut dinner = Room::new("Dinner".to_string());
        dinner.append_room_device("Smart Socket".to_string(), "DFK#14324".to_string());
    }
}

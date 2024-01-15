use super::room::Room;

pub trait DeviceInfo<'a> {
    fn get_device_info(&self, room_name: &'a Room, device_name: String) -> String;
}

impl<'a> DeviceInfo<'a> for String {
    fn get_device_info(&self, _room_name: &'a Room, device_name: String) -> String {
        device_name
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct SmartHome<'a> {
    pub(crate) title: String,
    pub(crate) rooms: Vec<&'a Room>,
}

impl<'a> SmartHome<'a> {
    pub fn new(title: String) -> Self {
        Self {
            title,
            rooms: Vec::new(),
        }
    }

    pub fn update_rooms(&mut self, room: &'a Room) {
        self.rooms.push(room);
    }

    pub fn get_rooms(&self) -> &Vec<&'a Room> {
        &self.rooms
    }

    pub fn create_report<D: DeviceInfo<'a>>(&self, device_info: &'a D) -> &D {
        device_info
    }
}

impl<'a> DeviceInfo<'a> for SmartHome<'a> {
    fn get_device_info(&self, room_name: &'a Room, device_name: String) -> String {
        let devices = &room_name.smart_devices;
        for device in devices {
            if device_name.trim() == device.name {
                let dev_name = &device.name;
                let room_name = &device.room_name;
                let id = &device.vendor_id;
                let status = match device.status {
                    false => "Inactive".to_string(),
                    true => "Active".to_string(),
                };

                let device_info = format!(
                    "Name: {} \nWich room: {} \nVendor ID: {} \nStatus: {}",
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
    #[test]
    fn test_create_smarthome() {
        SmartHome::new("Smart Home".to_string());
    }

    #[test]
    fn test_update_rooms() {
        let mut smart_home = SmartHome::new("My Home".to_string());
        let mut dinner = Room::new("Dinner".to_string());
        dinner.append_room_device("Smart Socket".to_string(), "DFK#14324".to_string());
        smart_home.update_rooms(&dinner);
    }

    #[test]
    fn test_get_rooms() {
        let mut smart_home = SmartHome::new("My Home".to_string());
        let mut dinner = Room::new("Dinner".to_string());
        dinner.append_room_device("Smart Socket".to_string(), "DFK#14324".to_string());
        smart_home.update_rooms(&dinner);
        smart_home.get_rooms();
    }

    #[test]
    fn test_get_device_info() {
        let mut smart_home = SmartHome::new("My Home".to_string());
        let mut dinner = Room::new("Dinner".to_string());
        dinner.append_room_device("Smart Socket".to_string(), "DFK#14324".to_string());
        smart_home.update_rooms(&dinner);
        smart_home.get_device_info(&dinner, "Smart Socket".to_string());
    }

    #[test]
    #[should_panic(expected = "Attempt to get device info, which dosent exist")]
    fn test_get_dont_ex_device_info() {
        let mut smart_home = SmartHome::new("My Home".to_string());
        let mut dinner = Room::new("Dinner".to_string());
        dinner.append_room_device("Smart Socket".to_string(), "DFK#14324".to_string());
        smart_home.update_rooms(&dinner);
        smart_home.get_device_info(&dinner, "Smart Thermometr".to_string());
    }

    #[test]
    fn test_create_report() {
        let mut smart_home = SmartHome::new("My Home".to_string());
        let mut dinner = Room::new("Dinner".to_string());
        dinner.append_room_device("Smart Socket".to_string(), "DFK#14324".to_string());
        smart_home.update_rooms(&dinner);
        let device_info = smart_home.get_device_info(&dinner, "Smart Socket".to_string());
        smart_home.create_report(&device_info);
    }
}

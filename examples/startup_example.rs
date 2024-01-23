use smart_home::home_part::{device::SmartDevice, home::*, room::Room};

fn main() {
    // Initialize SmartHome
    let mut smart_home = SmartHome::new("My Home");
    println!("{:?}", smart_home);

    // Initializ SmartDevice
    let mut new_device = SmartDevice::new("Smart Socket", "WE23_134");
    let mut new_device_1 = SmartDevice::new("Smart Socket", "WE23_234");
    let stats: Vec<(&str, &str)> = vec![("voltage", "220"), ("Is_on", "true"), ("Power", "2A")];
    let stats_1: Vec<(&str, &str)> = vec![("voltage", "220"), ("Is_on", "false"), ("Power", "1A")];

    new_device.update_status_info(stats);
    new_device_1.update_status_info(stats_1);

    // Initialize rooms
    let mut dinner = Room::new("Dinner");
    let _bedroom = Room::new("Bedroom");

    // Append SmartDevice to room
    dinner.append_room_device(&new_device);
    dinner.append_room_device(&new_device_1);

    // Append rooms to SmartHome
    smart_home.update_rooms(&dinner);
    // smart_home.update_rooms(&bedroom);

    // Get rooms and SmartDevice in it
    let rooms = smart_home.get_rooms();
    println!("{:?}", rooms);
    println!("{:?}", smart_home);

    // Create report
    smart_home.update_rooms(&dinner);
    let device_info = smart_home.get_device_info(&dinner, "Smart Socket/WE23_134").unwrap();
    let report = smart_home.create_report(&device_info);
    println!("{}", report);

    let room_devices_info = smart_home.get_room_devices_info(&dinner).unwrap();
    let reports = smart_home.create_report(&room_devices_info);
    println!("{}", reports)
}

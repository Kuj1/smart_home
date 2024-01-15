use smart_home::home_part::{
    home::{DeviceInfo, SmartHome},
    room::Room,
};

fn main() {
    // Initialize SmartHome
    let mut smart_home = SmartHome::new("My Home".to_string());
    println!("{:?}", smart_home);

    // Initialize rooms
    let bedroom = Room::new("Bedroom".to_string());
    let mut dinner = Room::new("Dinner".to_string());

    // Initializ SmartDevice and append to rooms
    dinner.append_room_device("Smart Socket".to_string(), "DFK#14324".to_string());
    dinner.append_room_device("Smart Thermometr".to_string(), "LK+14324".to_string());

    // Append rooms to SmartHome
    smart_home.update_rooms(&dinner);
    smart_home.update_rooms(&bedroom);

    // Get rooms and SmartDevice in it
    let rooms = smart_home.get_rooms();
    println!("{:?}", rooms);
    println!("{:?}", smart_home);

    // Create report
    let device_info = smart_home.get_device_info(&dinner, "Smart Socket".to_string());
    let report = smart_home.create_report(&device_info);
    println!("Report#1:\n{}", report)
}

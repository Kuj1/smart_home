use std::error::Error;

use smart_home::home_part::{device::SmartDevice, home::*, room::Room, errors::CommonError};

fn example() {
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
    let mut bedroom = Room::new("Bedroom");

    // Append SmartDevice to room
    dinner.append_room_device(&new_device);
    dinner.append_room_device(&new_device_1);
    bedroom.append_room_device(&new_device_1);

    // Delete device
    dinner.remove_device("Smart Socket/WE23_134");

    // Append rooms to SmartHome
    smart_home.update_rooms(&dinner);
    smart_home.update_rooms(&bedroom);

    // Delete rooms
    smart_home.remove_rooms("Dinner");

    // Get rooms and SmartDevice in it
    let rooms = smart_home.get_rooms();
    println!("{:#?}", rooms);
    println!("{:#?}", smart_home);

    // Create report
    let device_info = smart_home
        .get_device_info(&dinner, "Smart Socket/WE23_134")
        .unwrap_err();

    println!("{}", device_info);

    let dinner_devices_info = smart_home.get_room_devices_info(&dinner).unwrap();
    let reports = smart_home.create_report(&dinner_devices_info);
    println!("{}", reports);

    println!("{:#?}", smart_home);
}

fn example_from_cli() {
    // Initialize SmartHome
    let mut smart_home = SmartHome::new_from_cli();
    // println!("{:?}", smart_home);

    // Initializ SmartDevice
    let mut new_device = SmartDevice::new_from_cli();
    let stats: Vec<(String, String)> = SmartDevice::request_for_status_info();
    new_device.update_status_info_from_cli(stats);

    // Initialize rooms
    let mut dinner = Room::new_from_cli();

    // Append SmartDevice to room
    dinner.append_room_device(&new_device);

    // Append rooms to SmartHome
    smart_home.update_rooms(&dinner);
    println!("{:#?}", smart_home);
}

fn example_error() {
    let dont_exist_room: Result<i32, CommonError> = Err(CommonError::DontExistRoom);
    let dont_exist_device: Result<i32, CommonError> = Err(CommonError::DontExistDevice);
    let dont_exist_info: Result<i32, CommonError> = Err(CommonError::DontExistInfo);

    println!("\t*Errors in lib*\n{}\n{}\n{}\n", dont_exist_room.unwrap_err(), dont_exist_device.unwrap_err(), dont_exist_info.unwrap_err())
}

fn main() {
    example_error();
    example();
    example_from_cli();
}

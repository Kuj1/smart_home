use core::panic;

#[derive(Debug)]
#[allow(dead_code)]
struct SmartDevice {
    name: String,
    room_name: String,
    vendor_id: String,
    status: bool,
}

#[derive(Debug)]
#[allow(dead_code)]
struct Room {
    name: String,
    smart_devices: Vec<SmartDevice>,
}

impl Room {
    fn new(name: String) -> Self {
        Self {
            name,
            smart_devices: Vec::new(),
        }
    }

    fn append_room_device(&mut self, name_device: String, vendor_id_device: String) -> &mut Room {
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

trait DeviceInfo<'a> {
    fn get_device_info(&self, room_name: &'a Room, device_name: String) -> String;
}

impl<'a> DeviceInfo<'a> for String {
    fn get_device_info(&self, _room_name: &'a Room, device_name: String) -> String {
        device_name
    }
}

#[derive(Debug)]
#[allow(dead_code)]
struct SmartHome<'a> {
    title: String,
    rooms: Vec<&'a Room>,
}

impl<'a> SmartHome<'a> {
    fn new(title: String) -> Self {
        Self {
            title,
            rooms: Vec::new(),
        }
    }

    fn update_rooms(&mut self, room: &'a Room) {
        self.rooms.push(room);
    }

    fn get_rooms(&self) -> &Vec<&'a Room> {
        &self.rooms
    }

    fn create_report<D: DeviceInfo<'a>>(&self, device_info: &'a D) -> &D {
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

        panic!("Dont exist device")
    }
}

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

// + Дом имеет название и содержит несколько помещений.
// + Библиотека позволяет запросить список помещений в доме.
// + Помещение имеет уникальное название и содержит названия нескольких устройств.
// + Устройство имеет уникальное в рамках помещения имя.
// + Библиотека позволяет получать список устройств в помещении.
// + Библиотека имеет функцию, возвращающую текстовый отчёт о состоянии дома.
//      Эта функция принимает в качестве аргумента обобщённый тип, позволяющий получить текстовую информацию
//      о состоянии устройства, для включения в отчёт. Эта информация должна предоставляться
//      для каждого устройства на основе данных о положении устройства в доме: имени комнаты и имени устройства.
//      Если устройство не найдено в источнике информации, то вместо текста о состоянии вернуть сообщение об ошибке.
//      Привести пример типа, предоставляющего текстовую информацию об устройствах в доме для составления отчёта.

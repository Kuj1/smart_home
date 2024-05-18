use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;

use super::models::{House, Room, SmartDevice};
use super::DB;
use crate::errors::DataResult;

impl DB {
    pub async fn new() -> Self {
        dotenv().unwrap();
        let conn = env::var("DATABASE_URL").unwrap();
        let pool = PgPoolOptions::new()
            .max_connections(10)
            .connect(&conn)
            .await
            .unwrap();
        Self { pool }
    }

    pub async fn create_house(&self, data: &House) -> DataResult<House> {
        let id = sqlx::query!(
            "
            INSERT INTO houses (house_name)
                VALUES ($1) RETURNING id;
            ",
            &data.name
        )
        .fetch_one(&self.pool)
        .await?;

        self.get_house(id.id).await
    }

    pub async fn create_room(&self, data: &Room, house_id: i32) -> DataResult<Room> {
        let id = sqlx::query!(
            "
            INSERT INTO rooms (house_id, room_name)
                VALUES ($1, $2) RETURNING id;
            ",
            &house_id,
            &data.name
        )
        .fetch_one(&self.pool)
        .await?;

        self.get_room(id.id, house_id).await
    }

    pub async fn create_smart_device(
        &self,
        data: &SmartDevice,
        house_id: i32,
        room_id: i32,
    ) -> DataResult<SmartDevice> {
        let id = sqlx::query!(
            "
            INSERT INTO smart_devices (house_id, room_id, vendor_id, device_name, is_on, voltage, power)
                VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING id;
            ",
            &house_id, &room_id, data.vendor_id, data.name, data.is_on, data.voltage, data.power
        ).fetch_one(&self.pool).await?;

        self.get_smart_device_info(house_id, room_id, id.id).await
    }

    pub async fn get_house(&self, house_id: i32) -> DataResult<House> {
        let raw_query = sqlx::query!(
            "
            SELECT * FROM houses WHERE id=$1;
            ",
            &house_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(House {
            id: Some(house_id),
            name: raw_query.house_name,
        })
    }

    pub async fn get_houses(&self) -> DataResult<Vec<House>> {
        let raw_query = sqlx::query!(
            "
            SELECT * FROM houses;
            "
        )
        .fetch_all(&self.pool)
        .await
        .unwrap();

        let houses = raw_query
            .into_iter()
            .map(|x| House {
                id: Some(x.id),
                name: x.house_name,
            })
            .collect::<Vec<House>>();

        Ok(houses)
    }

    pub async fn get_room(&self, room_id: i32, house_id: i32) -> DataResult<Room> {
        let raw_query = sqlx::query!(
            "
            SELECT rooms.id, houses.house_name, rooms.room_name 
            FROM rooms 
            JOIN houses ON houses.id=rooms.house_id
            WHERE houses.id=$1 AND rooms.id=$2;
            ",
            &house_id,
            &room_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(Room {
            id: Some(room_id),
            house_name: Some(raw_query.house_name),
            name: raw_query.room_name,
        })
    }

    pub async fn get_rooms(&self, house_id: i32) -> DataResult<Vec<Room>> {
        let raw_query = sqlx::query!(
            "
            SELECT rooms.id, houses.house_name, rooms.room_name 
            FROM rooms 
            JOIN houses ON houses.id=rooms.house_id
            WHERE rooms.house_id=$1;
            ",
            &house_id
        )
        .fetch_all(&self.pool)
        .await
        .unwrap();

        let rooms = raw_query
            .into_iter()
            .map(|x| Room {
                id: Some(x.id),
                house_name: Some(x.house_name),
                name: x.room_name,
            })
            .collect::<Vec<Room>>();
        Ok(rooms)
    }

    pub async fn get_smart_device_info(
        &self,
        house_id: i32,
        room_id: i32,
        device_id: i32,
    ) -> DataResult<SmartDevice> {
        let raw_device = sqlx::query!(
            "
            SELECT smart_devices.id, houses.house_name, rooms.room_name, smart_devices.device_name, smart_devices.vendor_id, smart_devices.is_on, smart_devices.voltage, smart_devices.power
            FROM smart_devices 
            JOIN houses ON houses.id=smart_devices.house_id
            JOIN rooms ON rooms.id=smart_devices.room_id
            WHERE smart_devices.house_id=$1 AND smart_devices.room_id=$2 AND smart_devices.id=$3;
            ",
            &house_id, &room_id, &device_id
        ).fetch_one(&self.pool).await.unwrap();

        Ok(SmartDevice {
            id: Some(raw_device.id),
            house_name: Some(raw_device.house_name),
            room_name: Some(raw_device.room_name),
            name: Some(raw_device.device_name),
            vendor_id: Some(raw_device.vendor_id),
            is_on: Some(raw_device.is_on),
            voltage: Some(raw_device.voltage),
            power: Some(raw_device.power),
        })
    }

    pub async fn get_house_smart_devices_info(
        &self,
        house_id: i32,
    ) -> DataResult<Vec<SmartDevice>> {
        let raw_devices = sqlx::query!(
            "
            SELECT smart_devices.id, houses.house_name, rooms.room_name, smart_devices.device_name, smart_devices.vendor_id, smart_devices.is_on, smart_devices.voltage, smart_devices.power
            FROM smart_devices
            JOIN rooms ON rooms.id=smart_devices.room_id
            JOIN houses ON houses.id=smart_devices.house_id
            WHERE smart_devices.house_id=$1;
            ",
            &house_id
        ).fetch_all(&self.pool).await?;

        let devices = raw_devices
            .into_iter()
            .map(|x| SmartDevice {
                id: Some(x.id),
                house_name: Some(x.house_name),
                room_name: Some(x.room_name),
                name: Some(x.device_name),
                vendor_id: Some(x.vendor_id),
                is_on: Some(x.is_on),
                voltage: Some(x.voltage),
                power: Some(x.power),
            })
            .collect::<Vec<SmartDevice>>();

        Ok(devices)
    }

    pub async fn get_room_smart_devices_info(
        &self,
        house_id: i32,
        room_id: i32,
    ) -> DataResult<Vec<SmartDevice>> {
        let raw_devices = sqlx::query!(
            "
            SELECT smart_devices.id, houses.house_name, rooms.room_name, smart_devices.device_name, smart_devices.vendor_id, smart_devices.is_on, smart_devices.voltage, smart_devices.power
            FROM smart_devices
            JOIN houses ON houses.id=$1
            JOIN rooms ON rooms.id=smart_devices.room_id
            WHERE smart_devices.room_id=$2;
            ",
            &house_id,
            &room_id
        ).fetch_all(&self.pool).await?;

        let devices = raw_devices
            .into_iter()
            .map(|x| SmartDevice {
                id: Some(x.id),
                house_name: Some(x.house_name),
                room_name: Some(x.room_name),
                name: Some(x.device_name),
                vendor_id: Some(x.vendor_id),
                is_on: Some(x.is_on),
                voltage: Some(x.voltage),
                power: Some(x.power),
            })
            .collect::<Vec<SmartDevice>>();

        Ok(devices)
    }

    pub async fn delete_smart_device(
        &self,
        house_id: i32,
        room_id: i32,
        device_id: i32,
    ) -> DataResult<SmartDevice> {
        let device = self
            .get_smart_device_info(house_id, room_id, device_id)
            .await?;
        sqlx::query!(
            "
            DELETE FROM smart_devices WHERE house_id=$1 
                                            AND room_id=$2
                                            AND id=$3;
            ",
            &house_id,
            &room_id,
            &device_id
        )
        .execute(&self.pool)
        .await?;

        Ok(device)
    }

    pub async fn delete_room(&self, house_id: i32, room_id: i32) -> DataResult<Room> {
        let room = self.get_room(room_id, house_id).await?;
        sqlx::query!(
            "
            DELETE FROM rooms WHERE house_id=$1
                                    AND id=$2;
            ",
            &house_id,
            &room_id,
        )
        .execute(&self.pool)
        .await
        .unwrap();

        Ok(room)
    }

    pub async fn delete_house(&self, house_id: i32) -> DataResult<House> {
        let house = self.get_house(house_id).await?;
        sqlx::query!(
            "
            DELETE FROM houses WHERE id=$1;
            ",
            &house_id,
        )
        .execute(&self.pool)
        .await
        .unwrap();

        Ok(house)
    }

    pub async fn update_smart_device_info(
        &self,
        house_id: i32,
        room_id: i32,
        device_id: i32,
        data: &SmartDevice,
    ) -> DataResult<SmartDevice> {
        sqlx::query!(
            "
            UPDATE smart_devices
            SET is_on=$4, voltage=$5, power=$6
            WHERE smart_devices.house_id=$1 AND smart_devices.room_id=$2 AND id=$3
            ",
            &house_id,
            &room_id,
            &device_id,
            data.is_on,
            data.voltage,
            data.power
        )
        .execute(&self.pool)
        .await?;

        self.get_smart_device_info(house_id, room_id, device_id)
            .await
    }

    pub async fn update_smart_device_name(
        &self,
        house_id: i32,
        room_id: i32,
        device_id: i32,
        data: &SmartDevice,
    ) -> DataResult<SmartDevice> {
        sqlx::query!(
            "
            UPDATE smart_devices
            SET device_name=$4
            WHERE smart_devices.house_id=$1 AND smart_devices.room_id=$2 AND id=$3
            ",
            &house_id,
            &room_id,
            &device_id,
            data.name
        )
        .execute(&self.pool)
        .await?;

        self.get_smart_device_info(house_id, room_id, device_id)
            .await
    }

    pub async fn update_room_name(
        &self,
        house_id: i32,
        room_id: i32,
        data: &SmartDevice,
    ) -> DataResult<Room> {
        sqlx::query!(
            "
            UPDATE rooms
            SET room_name=$3
            WHERE rooms.house_id=$1 AND id=$2
            ",
            &house_id,
            &room_id,
            data.name
        )
        .execute(&self.pool)
        .await?;

        self.get_room(room_id, house_id).await
    }

    pub async fn update_house_name(&self, house_id: i32, data: &SmartDevice) -> DataResult<House> {
        sqlx::query!(
            "
            UPDATE houses
            SET house_name=$2
            WHERE id=$1
            ",
            &house_id,
            data.name
        )
        .execute(&self.pool)
        .await?;

        self.get_house(house_id).await
    }
}

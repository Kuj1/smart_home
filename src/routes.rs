use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};

use crate::sql::{
    models::{House, Room, SmartDevice},
    DB,
};

pub async fn create_house(State(db): State<DB>, Json(data): Json<House>) -> impl IntoResponse {
    db.create_house(&data).await.map(Json)
}

pub async fn create_room(
    Path(house_id): Path<i32>,
    State(db): State<DB>,
    Json(data): Json<Room>,
) -> impl IntoResponse {
    db.create_room(&data, house_id).await.map(Json)
}

pub async fn create_smart_device(
    Path((house_id, room_id)): Path<(i32, i32)>,
    State(db): State<DB>,
    Json(data): Json<SmartDevice>,
) -> impl IntoResponse {
    db.create_smart_device(&data, house_id, room_id)
        .await
        .map(Json)
}

pub async fn get_house(Path(house_id): Path<i32>, State(db): State<DB>) -> impl IntoResponse {
    db.get_house(house_id).await.map(Json)
}

pub async fn get_houses(State(db): State<DB>) -> impl IntoResponse {
    db.get_houses().await.map(Json)
}

pub async fn get_room(
    Path((house_id, room_id)): Path<(i32, i32)>,
    State(db): State<DB>,
) -> impl IntoResponse {
    db.get_room(room_id, house_id).await.map(Json)
}

pub async fn get_rooms(Path(house_id): Path<i32>, State(db): State<DB>) -> impl IntoResponse {
    db.get_rooms(house_id).await.map(Json)
}

pub async fn get_smart_device_info(
    Path((house_id, room_id, device_id)): Path<(i32, i32, i32)>,
    State(db): State<DB>,
) -> impl IntoResponse {
    db.get_smart_device_info(house_id, room_id, device_id)
        .await
        .map(Json)
}

pub async fn get_house_smart_devices_info(
    Path(house_id): Path<i32>,
    State(db): State<DB>,
) -> impl IntoResponse {
    db.get_house_smart_devices_info(house_id).await.map(Json)
}

pub async fn get_room_smart_devices_info(
    Path((house_id, room_id)): Path<(i32, i32)>,
    State(db): State<DB>,
) -> impl IntoResponse {
    db.get_room_smart_devices_info(house_id, room_id)
        .await
        .map(Json)
}

pub async fn delete_smart_device(
    Path((house_id, room_id, device_id)): Path<(i32, i32, i32)>,
    State(db): State<DB>,
) -> impl IntoResponse {
    db.delete_smart_device(house_id, room_id, device_id)
        .await
        .map(Json)
}

pub async fn delete_room(
    Path((house_id, room_id)): Path<(i32, i32)>,
    State(db): State<DB>,
) -> impl IntoResponse {
    db.delete_room(house_id, room_id).await.map(Json)
}

pub async fn delete_house(Path(house_id): Path<i32>, State(db): State<DB>) -> impl IntoResponse {
    db.delete_house(house_id).await.map(Json)
}

pub async fn update_smart_device_info(
    Path((house_id, room_id, device_id)): Path<(i32, i32, i32)>,
    State(db): State<DB>,
    Json(data): Json<SmartDevice>,
) -> impl IntoResponse {
    db.update_smart_device_info(house_id, room_id, device_id, &data)
        .await
        .map(Json)
}

pub async fn update_smart_device_name(
    Path((house_id, room_id, device_id)): Path<(i32, i32, i32)>,
    State(db): State<DB>,
    Json(data): Json<SmartDevice>,
) -> impl IntoResponse {
    db.update_smart_device_name(house_id, room_id, device_id, &data)
        .await
        .map(Json)
}

pub async fn update_room_name(
    Path((house_id, room_id)): Path<(i32, i32)>,
    State(db): State<DB>,
    Json(data): Json<SmartDevice>,
) -> impl IntoResponse {
    db.update_room_name(house_id, room_id, &data)
        .await
        .map(Json)
}

pub async fn update_house_name(
    Path(house_id): Path<i32>,
    State(db): State<DB>,
    Json(data): Json<SmartDevice>,
) -> impl IntoResponse {
    db.update_house_name(house_id, &data).await.map(Json)
}

use axum::routing::{get, patch, post};
use axum::Router;

use smart_home::routes::{
    create_house, create_room, create_smart_device, delete_house, delete_room, delete_smart_device,
    get_house, get_house_smart_devices_info, get_houses, get_room, get_room_smart_devices_info,
    get_rooms, get_smart_device_info, update_house_name, update_room_name,
    update_smart_device_info, update_smart_device_name,
};
use smart_home::sql::DB;

#[tokio::main]
async fn main() {
    let db = DB::new().await;

    tracing_subscriber::fmt().init();

    let app = Router::new()
        .route("/houses", post(create_house).get(get_houses))
        .route("/houses/:house_id", get(get_house).delete(delete_house))
        .route("/houses/:house_id/rooms", post(create_room).get(get_rooms))
        .route(
            "/houses/:house_id/rooms/:room_id",
            get(get_room).delete(delete_room),
        )
        .route(
            "/houses/:house_id/rooms/:room_id/devices",
            post(create_smart_device).get(get_room_smart_devices_info),
        )
        .route(
            "/houses/:house_id/rooms/:room_id/devices/:device_id",
            get(get_smart_device_info)
                .delete(delete_smart_device)
                .patch(update_smart_device_info),
        )
        .route(
            "/houses/:house_id/rooms/:room_id/devices/rename/:device_id",
            patch(update_smart_device_name),
        )
        .route(
            "/houses/:house_id/rooms/rename/:room_id",
            patch(update_room_name),
        )
        .route("/houses/rename/:house_id", patch(update_house_name))
        .route(
            "/houses/:house_id/devices",
            get(get_house_smart_devices_info),
        )
        .with_state(db);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:5000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

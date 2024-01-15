#[derive(Debug)]
#[allow(dead_code)]
pub struct SmartDevice {
    pub(crate) name: String,
    pub(crate) room_name: String,
    pub(crate) vendor_id: String,
    pub(crate) status: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_create_device() {
        SmartDevice {
            name: "Smart Device".to_string(),
            room_name: "Dinner".to_string(),
            vendor_id: "DS!1$124e".to_string(),
            status: true,
        };
    }
}

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
    // use super::*;
    #[test]
    fn test_create_device() {
        "Smart Device".to_string();"Dinner".to_string();"DS!1$124e".to_string();
    }
}

use uuid::Uuid;

pub fn generate_uuid_from_str(value: &str) -> Option<Uuid> {
    match Uuid::parse_str(value) {
        Ok(r) => Some(r),
        Err(_e) => None,
    }
}
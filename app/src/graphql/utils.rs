use uuid::Uuid;


pub fn generate_uuid_from_str(value: &str) -> Option<Uuid> {
    match Uuid::parse_str(value) {
        Ok(r) => Some(r),
        Err(_e) => None,
    }
}


// pub struct Timer<'a> {
//     name: &'a str,
//     instant: Instant
// }
//
// impl<'a> Timer<'a> {
//     pub fn new(name: &'a str) -> Timer<'a> {
//         Timer { name, instant: Instant::now() }
//     }
//     pub fn clock(&self) {
//         println!("Timer with name: {}, spent: {} milliseconds", self.name, self.instant.elapsed().as_millis() )
//     }
// }
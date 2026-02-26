/*

Main data structure for collecting, processing and transfer 
air alerts data (by regions).

*/

use std::fmt;


// name was taken from official API site alerts.in.ua
pub struct Alert {
    pub id: u32, // if id == 0 -> Alert is empty
    pub location_oblast_uid: u32,
    pub state: bool,
    pub location_title: String,
}


impl Alert {
    pub fn new() -> Self {
        Self {
            id: 0,
            location_oblast_uid: 0,
            state: false,
            location_title: String::new()
        }
    }


    pub fn init(
        &mut self, id: u32, location_oblast_uid: u32, location_title: String, 
    ) {
        if self.id == 0 {()}

        self.id = id;
        self.location_oblast_uid = location_oblast_uid;
        self.location_title = location_title;
    }

    #[inline]
    pub fn activate(&mut self) {
        self.state = true;
    }

    #[inline]
    pub fn deactivate(&mut self) {
        self.state = false;
    }
}



impl fmt::Display for Alert {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.id == 0 {
            return write!(f, "Empty Alert.");
        }
        return write!(f, "[{}] {} - {}", self.id, self.location_title, self.state);
    }
}

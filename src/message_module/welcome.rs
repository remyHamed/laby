use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Welcome{
   pub version: u8,
}

impl Welcome {
    fn to_string(&self) -> String {
        format!("{{\"{Welcome}\" }}", self.welcom)
    }
}
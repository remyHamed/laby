use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Welcome{
    welcom: String
}

impl W {
    fn to_string(&self) -> String {
        format!("{{\"{Welcome}\" }}", self.welcom)
    }
}
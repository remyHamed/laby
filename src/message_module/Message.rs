use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct S {
    subscribe: String
}

impl S {
    fn to_string(&self) -> String {
        format!("{{ subscribe:{\"name\": {s}}}", s =  self.subscribe)
    }
    fn size(&self) -> u32 {
        return self.subscribe.len() as u32;
    }
}
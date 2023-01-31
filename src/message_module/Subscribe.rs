use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Subscribe{
    name: String
}

impl S {
    fn to_string(&self) -> String {
        format!("{{ s:\"{s}\", i:{0}, f:{float} }}", self.i, s = self.s, float = self.f)
    }
}
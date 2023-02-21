use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Subscribe{
    name: String
}

impl Subscribe {
    fn get_name(&self) -> String { &self.name}
    fn to_string(&self) -> String {
        format!("{ \"Subscribe\" :{ \"name\" :\"{n}\"} }}", n = self.name)
    }
}


#[cfg(test)]
mod test{
    // use crate::messages::message_a::MessageA;
    use super::*;

    #[test]
    fn test() {
        // crate::messages::message_a::tests::test
        let m = Subscribe { name: "42".to_string() };
        assert_eq!(*m.get_name(), 42);
        assert_eq!(m.get_name(), &42);
    }
}
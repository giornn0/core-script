use std::fmt::Display;
#[derive(Clone, Copy, Debug)]
pub enum Property {
    String,
    Number,
    Date,
    Related,
}
const PROPERTIES: [Property; 4] = [
    Property::String,
    Property::Number,
    Property::Date,
    Property::Related,
];
impl Display for Property {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Property::Date => write!(f, "Date"),
            Property::Number => write!(f, "Number"),
            Property::String => write!(f, "String"),
            Property::Related => write!(f, "Related"),
        }
    }
}
impl Property {
    pub fn get_vec() -> Vec<Property> {
        let mut v = Vec::new();
        for prop in PROPERTIES.iter() {
            v.push(*prop)
        }
        v
    }
}

use std::hash::Hash;

#[derive(Hash, Eq, PartialEq)]
pub enum Elements {
    Text {
        text: Vec<u8>
    },
    BoldText {
        text: Vec<u8>
    }
}

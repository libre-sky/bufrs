use crate::message::section::{Section0, Section1, Section2, Section3, Section4, Section5};

pub mod section;
pub mod table;

#[derive(Default, Debug)]
pub struct Message {
    s0: Section0,
    s1: Section1,
    s2: Option<Section2>,
    s3: Section3,
    s4: Section4,
    s5: Section5,
}

impl Message {}

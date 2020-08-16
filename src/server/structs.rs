use crate::color::Color;

#[derive(Clone, Copy)]
pub struct Team {
    pub name: &'static str,
    pub color: Color,
}

impl Team {
    pub fn new(name: &'static str, color: Color) -> Self {
        Self { name, color }
    }
}

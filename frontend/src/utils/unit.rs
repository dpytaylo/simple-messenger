use std::fmt;

#[derive(Clone, Debug)]
pub enum Unit {
    None,
    Auto,
    Rem(f32),
}

impl Default for Unit {
    fn default() -> Self {
        Self::Rem(0.0)
    }
}

impl fmt::Display for Unit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Unit::None => write!(f, "none"),
            Unit::Auto => write!(f, "auto"),
            Unit::Rem(val) => write!(f, "{val}rem"),
        }
    }
}

pub struct Rem(f32);

impl Into<Unit> for Rem {
    fn into(self) -> Unit {
        Unit::Rem(self.0)
    }
}

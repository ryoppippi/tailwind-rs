use super::*;

#[derive(Clone, Debug)]
pub(super) enum Animation {
    None,
    Spin,
    Ping,
    Pulse,
    Bounce,
    Arbitrary(String),
}

impl Display for Animation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => write!(f, "none"),
            Self::Spin => write!(f, "spin"),
            Self::Ping => write!(f, "ping"),
            Self::Pulse => write!(f, "pulse"),
            Self::Bounce => write!(f, "bounce"),
            Self::Arbitrary(s) => write!(f, "{}", s),
        }
    }
}

impl Animation {
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        todo!()
    }
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self::Arbitrary(arbitrary.to_string()))
    }
}
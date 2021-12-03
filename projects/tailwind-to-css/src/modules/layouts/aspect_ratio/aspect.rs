use super::*;

#[derive(Clone, Debug)]
pub enum Aspect {
    Radio(usize, usize),
    Standard(String),
    Arbitrary(String),
}

impl Display for Aspect {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Radio(a, b) => write!(f, "{}/{}", a, b),
            Self::Standard(s) => write!(f, "{}", s),
            Self::Arbitrary(s) => write!(f, "[{}]", s),
        }
    }
}

impl Aspect {
    pub fn parse(kind: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        let out = match kind {
            ["square"] => Self::Radio(1, 1),
            ["video"] => Self::Radio(16, 9),
            [s] if Self::check_valid(s) => Self::Standard(s.to_string()),
            [n] => {
                let (a, b) = TailwindArbitrary::from(*n).as_fraction()?;
                Self::Radio(a, b)
            },
            [] => Self::parse_arbitrary(arbitrary)?,
            _ => return syntax_error!("unknown aspect-ratio elements"),
        };
        Ok(out)
    }
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary) -> Result<Self> {
        debug_assert!(arbitrary.is_some());
        Ok(Self::Arbitrary(arbitrary.to_string()))
    }
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec!["auto", "inherit", "initial", "revert", "unset"]);
        set.contains(mode)
    }
    pub fn get_properties(&self) -> String {
        match self {
            Self::Radio(a, b) => format!("{}/{}", a, b),
            Self::Standard(s) => s.to_string(),
            Self::Arbitrary(s) => s.to_string(),
        }
    }
}
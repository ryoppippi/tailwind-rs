use super::*;

#[derive(Copy, Clone, Debug)]
enum ZIndex {
    Auto,
    Unit(usize),
}

#[doc=include_str!("readme.md")]
#[derive(Copy, Clone, Debug)]
pub struct TailwindZIndex {
    kind: ZIndex,
    neg: bool,
}

impl Display for TailwindZIndex {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.kind {
            ZIndex::Auto => write!(f, "w-auto"),
            ZIndex::Unit(n) if self.neg => write!(f, "-w-{}", n),
            ZIndex::Unit(n) => write!(f, "w-{}", n),
        }
    }
}

impl TailwindInstance for TailwindZIndex {
    fn attributes(&self, _: &TailwindBuilder) -> CssAttributes {
        let index = match self.kind {
            ZIndex::Auto => "auto".to_string(),
            ZIndex::Unit(n) if self.neg => format!("-{}", n),
            ZIndex::Unit(n) => format!("{}", n),
        };
        css_attributes! {
            "z-index" => index
        }
    }
}
impl TailwindZIndex {
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary, neg: bool) -> Result<Self> {
        debug_assert!(arbitrary.is_none(), "forbidden arbitrary after z-index");
        match pattern {
            ["auto"] => Ok(Self { kind: ZIndex::Auto, neg }),
            [n] => {
                let a = TailwindArbitrary::from(*n);
                Ok(Self { kind: ZIndex::Unit(a.as_integer()?), neg })
            },
            _ => syntax_error!("Unknown z-index instructions"),
        }
    }
}

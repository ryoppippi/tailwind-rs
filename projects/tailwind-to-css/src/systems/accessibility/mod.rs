use super::*;

#[doc = include_str!("screen-reader.md")]
#[derive(Debug)]
pub struct TailwindScreenReader {
    inner: bool,
}

impl TailwindInstance for TailwindScreenReader {
    fn id(&self) -> String {
        match self.inner {
            true => "sr-only",
            false => "not-sr-only",
        }
        .to_string()
    }
    fn selectors(&self, _: &TailwindBuilder) -> String {
        self.id()
    }
    fn attributes(&self, ctx: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        todo!()
    }
}

impl TailwindScreenReader {
    pub fn parse(input: bool) -> Box<dyn TailwindInstance> {
        let out = match input {
            true => Self { inner: true },
            false => Self { inner: false },
        };
        Box::new(out)
    }
}

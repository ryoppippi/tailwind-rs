


pub struct CssFormatter<'a> {
    buffer: &'a mut (dyn Write + 'a),
}

pub trait CssDisplay {
    fn display(&self, f: &mut CssFormatter) -> String;
}


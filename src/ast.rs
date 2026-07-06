use std::fmt;

#[derive(Debug, Clone)]
pub enum Reg {
    Empty,
    Eps,
    Chr(char),
    Alt(Box<Reg>, Box<Reg>),//union of Reg
    Seq(Box<Reg>, Box<Reg>),//concatenation of Reg
    Star(Box<Reg>),
}

impl fmt::Display for Reg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Reg::Empty => write!(f, "\u{2205}"),
            Reg::Eps => write!(f, "\u{03B5}"),
            Reg::Chr(c) => write!(f, "{}", c),
            Reg::Alt(r, s) => write!(f, "[{}|{}]", r, s),
            Reg::Seq(r, s) => write!(f, "{}{}", r, s),
            Reg::Star(r) => write!(f, "({})*", r),
        }
    }
}

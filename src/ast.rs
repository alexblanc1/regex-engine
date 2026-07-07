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
//Bon là je suis crevé. Plusieurs options :
//--Soit je fais la simplication dans fmt::Display : simple à faire, mais coûteux à la fin (on fera pas ça)
//--Soit j'implémente des règles de calculs basique sur les regx qui vont faire qu'en permanance ils vont se simplifier seuls
//--Soit je fais une fonction qui simplifie une bonne fois pour toute et je la mets de partout. A la limité ça pourrait donner lui un nouveau type de structure, des regex "simplifiés"/"normalisés" et peut-être me reposer sur le fait qu'on a des histoires de stabilité quand on les manipule.
impl Simplication for Reg {
    fn simplification(re: &Reg) -> Reg {
        match re {
            Reg::Eps => Reg::Eps,
            Reg::Empty => Reg::Eps,
            Reg::Alt(r, Reg::Empty) => r,
            Reg::Alt(r, s) if s == r.clone() => r,
            Reg::Seq(r, Reg::Eps) => r,
            Reg::Seq(Reg::Eps, r) => r,
            Reg::Seq(r,Reg::Empty) => r,
            Reg::Seq(Reg::Empty, r) => Reg::Empty,
            Reg::Seq(r,Reg::Empty) => Reg::Empty,
            Reg::Star(Reg::Star(r)) => Reg::Star(r)
        }
    }
}




use std::fmt;
use std::ops::{Add, BitOr};


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
// ---------------------------------------------------------------------------
// Smart constructors.
//
// Invariant: if the operands are already in normal form, the result is in
// normal form too, and only the top of the tree has to be inspected (O(1),
// no re-traversal). Outside this module, Alt/Seq/Star should never be built
// by hand: always go through alt/seq/star so the invariant is preserved.
// ---------------------------------------------------------------------------

/// Alternation `r | s`, simplified and canonicalized.
///
/// Rules: `r + ∅ = r`, `∅ + r = r`, `r + r = r`, plus commutativity:
/// the operands are stored in a canonical order (the derived total order
/// on `Reg`), so `a + b` and `b + a` build the exact same term.
pub fn alt(r: Reg, s: Reg) -> Reg {
    match (r, s) {
        (Reg::Empty, s) => s,
        (r, Reg::Empty) => r,
        (r, s) if r == s => r,
        (r, s) if r < s => Reg::Alt(Box::new(r), Box::new(s)),
        (r, s) => Reg::Alt(Box::new(s), Box::new(r)),
    }
}

/// Concatenation `r s`, simplified.
///
/// Rules: `∅ r = ∅`, `r ∅ = ∅`, `ε r = r`, `r ε = r`.
pub fn seq(r: Reg, s: Reg) -> Reg {
    match (r, s) {
        (Reg::Empty, _) | (_, Reg::Empty) => Reg::Empty,
        (Reg::Eps, s) => s,
        (r, Reg::Eps) => r,
        (r, s) => Reg::Seq(Box::new(r), Box::new(s)),
    }
}

/// Kleene star `r*`, simplified.
///
/// Rules: `(r*)* = r*`, plus the two edge cases `∅* = ε` and `ε* = ε`.
pub fn star(r: Reg) -> Reg {
    match r {
        Reg::Empty | Reg::Eps => Reg::Eps,
        s @ Reg::Star(_) => s,
        r => Reg::Star(Box::new(r)),
    }
}

impl Reg {
    /// Number of nodes of the syntax tree.
    ///
    /// Handy to check the whole point of the smart constructors: repeated
    /// derivatives must stay bounded instead of blowing up.
    pub fn size(&self) -> usize {
        match self {
            Reg::Empty | Reg::Eps | Reg::Chr(_) => 1,
            Reg::Alt(r, s) | Reg::Seq(r, s) => 1 + r.size() + s.size(),
            Reg::Star(r) => 1 + r.size(),
        }
    }
}

// Operator sugar: `r | s` (regex flavor) and `r + s` (the union notation of
// the maths notes) both go through `alt`, so operator-built expressions are
// simplified exactly like the others. Both operators take `self` by value:
// they consume their operands (clone an operand if it has to be reused).
impl BitOr for Reg {
    type Output = Reg;

    fn bitor(self, rhs: Reg) -> Reg {
        alt(self, rhs)
    }
}

impl Add for Reg {
    type Output = Reg;

    fn add(self, rhs: Reg) -> Reg {
        alt(self, rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::Reg::*;

    #[test]
    fn alt_simplifies() {
        // r + ∅ = r and ∅ + r = r
        assert_eq!(alt(Chr('a'), Empty), Chr('a'));
        assert_eq!(alt(Empty, Chr('a')), Chr('a'));
        // r + r = r
        assert_eq!(alt(Chr('a'), Chr('a')), Chr('a'));
    }

    #[test]
    fn alt_canonicalizes_commutativity() {
        // a + b and b + a must build the exact same term
        assert_eq!(alt(Chr('a'), Chr('b')), alt(Chr('b'), Chr('a')));
    }

    #[test]
    fn seq_simplifies() {
        // ε r = r and r ε = r
        assert_eq!(seq(Eps, Chr('a')), Chr('a'));
        assert_eq!(seq(Chr('a'), Eps), Chr('a'));
        // ∅ r = ∅ and r ∅ = ∅
        assert_eq!(seq(Empty, Chr('a')), Empty);
        assert_eq!(seq(Chr('a'), Empty), Empty);
    }

    #[test]
    fn star_simplifies() {
        // (r*)* = r*
        assert_eq!(star(star(Chr('a'))), star(Chr('a')));
        // ∅* = ε and ε* = ε
        assert_eq!(star(Empty), Eps);
        assert_eq!(star(Eps), Eps);
    }

    #[test]
    fn operators_use_smart_constructors() {
        assert_eq!(Chr('a') | Empty, Chr('a'));
        assert_eq!(Chr('a') + Chr('a'), Chr('a'));
        // operands end up in canonical order, whatever the syntax used
        assert_eq!(Chr('b') | Chr('a'), alt(Chr('a'), Chr('b')));
    }
}


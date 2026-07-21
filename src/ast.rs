use std::fmt;
use std::ops::{Add, BitOr};


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Reg {
    Empty,
    Eps,
    Chr(char),
    Class(CharClass),        // `.`, [a-z], [^0-9], … — a set of characters
    Alt(Box<Reg>, Box<Reg>),//union of Reg
    Seq(Box<Reg>, Box<Reg>),//concatenation of Reg
    Star(Box<Reg>),
}


pub enum RangeModifier {
    ZeroOrOne,
    ZeroOrMore,
    OneOrMore,
    Repeat(u8),
    RepeatAtLeast(u8),
    RepeatBetween(u8,u8),
}

/// A set of characters, used to represent `.` (any character), classes like
/// `[a-z]`, and negated classes like `[^0-9]`.
///
/// The set is stored as a list of **inclusive** ranges kept **sorted** and
/// **non-overlapping** (see [`CharClass::new`]). Membership can then be tested
/// in `O(log n)` with a binary search instead of scanning every range.
///
/// The `negated` flag flips membership, so a single representation covers both
/// `[…]` and `[^…]`. As a nice consequence, the wildcard `.` is simply *"the
/// negation of the empty set"*: it excludes nothing, hence matches everything
/// (see [`CharClass::any`]).
///
/// It derives the same traits as [`Reg`] (`PartialOrd, Ord, Hash`) because it
/// is now a field of `Reg::Class`, and `Reg`'s own derives require it.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CharClass {
    ranges: Vec<(char, char)>,
    negated: bool,
}

impl CharClass {
    /// Build a class from arbitrary ranges, normalized: invalid ranges
    /// (`lo > hi`) are dropped, the rest are sorted and overlapping ones are
    /// merged. Two classes denoting the same set thus compare equal — which
    /// matters for state deduplication in the Phase 4 DFA.
    pub fn new(ranges: Vec<(char, char)>, negated: bool) -> Self {
        Self { ranges: normalize(ranges), negated }
    }

    /// The singleton class `{c}` (equivalent to `Reg::Chr(c)`, but as a set).
    pub fn single(c: char) -> Self {
        Self { ranges: vec![(c, c)], negated: false }
    }

    /// The inclusive range `[lo-hi]`.
    pub fn range(lo: char, hi: char) -> Self {
        Self::new(vec![(lo, hi)], false)
    }

    /// The wildcard `.` — every character. Empty range list, negated: the
    /// complement of nothing is everything.
    pub fn any() -> Self {
        Self { ranges: Vec::new(), negated: true }
    }

    /// Whether `c` belongs to the class. `O(log n)` via binary search:
    /// `partition_point` finds the first range that could contain `c`, then a
    /// single bound check settles it. The final `^ self.negated` flips the
    /// answer for negated classes.
    pub fn contains(&self, c: char) -> bool {
        let idx = self.ranges.partition_point(|&(lo, _)| lo <= c);
        let in_ranges = idx > 0 && self.ranges[idx - 1].1 >= c;
        in_ranges ^ self.negated
    }
}

/// Sort ranges and merge overlaps so the list is sorted and disjoint, which is
/// what makes [`CharClass::contains`]'s binary search valid.
fn normalize(mut ranges: Vec<(char, char)>) -> Vec<(char, char)> {
    ranges.retain(|&(lo, hi)| lo <= hi);
    ranges.sort_unstable();
    let mut merged: Vec<(char, char)> = Vec::with_capacity(ranges.len());
    for (lo, hi) in ranges {
        // Sorting guarantees `lo >= last.0`, so ranges overlap iff `lo <= last.1`.
        // The let-chain keeps `last`'s mutable borrow confined to this block, so
        // it is released before the `push` below (NLL) — no borrow-checker clash.
        if let Some(last) = merged.last_mut()
            && lo <= last.1
        {
            // Overlapping: extend the previous range if this one reaches further.
            if hi > last.1 {
                last.1 = hi;
            }
            continue;
        }
        merged.push((lo, hi));
    }
    merged
}




impl fmt::Display for Reg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Reg::Empty => write!(f, "\u{2205}"),
            Reg::Eps => write!(f, "\u{03B5}"),
            Reg::Chr(c) => write!(f, "{}", c),
            Reg::Class(cc) => write!(f, "{}", cc),
            Reg::Alt(r, s) => write!(f, "[{}|{}]", r, s),
            Reg::Seq(r, s) => write!(f, "{}{}", r, s),
            Reg::Star(r) => write!(f, "({})*", r),
        }
    }
}

impl fmt::Display for CharClass {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // `.` is the negation of the empty set: matches everything.
        if self.negated && self.ranges.is_empty() {
            return write!(f, ".");
        }
        write!(f, "[")?;
        if self.negated {
            write!(f, "^")?;
        }
        for &(lo, hi) in &self.ranges {
            if lo == hi {
                write!(f, "{}", lo)?;
            } else {
                write!(f, "{}-{}", lo, hi)?;
            }
        }
        write!(f, "]")
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

// Quantifier desugaring. These build nothing new at the AST level: bounded
// repetition is just concatenation of copies of `r`, so it goes through `seq`
// / `alt` / `star` and inherits their simplification for free.

/// `r^n` — `n` copies of `r` concatenated (`r^0 = ε`).
///
/// Built with `seq`, which collapses `seq(r, ε) = r`, so no stray `ε` is left
/// dangling in the tree.
fn repeat(r: &Reg, n: usize) -> Reg {
    let mut acc = Reg::Eps;
    for _ in 0..n {
        acc = seq(r.clone(), acc);
    }
    acc
}

/// `r{n,m}` — `n` mandatory copies followed by `m - n` optional ones.
///
/// `(r?)^{m-n}` accepts `r^j` for `0 ≤ j ≤ m-n`, so prefixing `r^n` yields
/// exactly `{ r^k | n ≤ k ≤ m }`.
fn repeat_between(r: &Reg, n: usize, m: usize) -> Reg {
    debug_assert!(n <= m, "invalid bounded quantifier: n={n} > m={m}");
    let mut acc = Reg::Eps;
    for _ in 0..(m - n) {
        acc = seq(alt(r.clone(), Reg::Eps), acc); // prepend one `r?`
    }
    for _ in 0..n {
        acc = seq(r.clone(), acc); // prepend one mandatory `r`
    }
    acc
}

impl Reg {
    /// Number of nodes of the syntax tree.
    ///
    /// Handy to check the whole point of the smart constructors: repeated
    /// derivatives must stay bounded instead of blowing up.
    pub fn size(&self) -> usize {
        match self {
            Reg::Empty | Reg::Eps | Reg::Chr(_) | Reg::Class(_) => 1,
            Reg::Alt(r, s) | Reg::Seq(r, s) => 1 + r.size() + s.size(),
            Reg::Star(r) => 1 + r.size(),
        }
    }

    /// The wildcard `.` as a regex node.
    pub fn any() -> Reg {
        Reg::Class(CharClass::any())
    }

    /// Apply a quantifier to `self`, desugaring it into core nodes through the
    /// smart constructors. Consumes `self`; clones internally when a copy of
    /// the operand is needed more than once (`r+`, `r{n}`, …).
    pub fn apply(self, modifier: RangeModifier) -> Reg {
        use RangeModifier::*;
        match modifier {
            ZeroOrOne => alt(self, Reg::Eps),            // r? = r | ε
            ZeroOrMore => star(self),                    // r*
            OneOrMore => {
                let tail = star(self.clone());
                seq(self, tail)                          // r+ = r · r*
            }
            Repeat(n) => repeat(&self, n as usize),      // r{n}
            RepeatAtLeast(n) => {
                let tail = star(self.clone());
                seq(repeat(&self, n as usize), tail)     // r{n,} = r{n} · r*
            }
            RepeatBetween(n, m) => repeat_between(&self, n as usize, m as usize), // r{n,m}
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

    #[test]
    fn class_membership() {
        let lower = CharClass::range('a', 'z');
        assert!(lower.contains('a'));
        assert!(lower.contains('m'));
        assert!(lower.contains('z'));
        assert!(!lower.contains('A')); // 'A' < 'a'
        assert!(!lower.contains('0'));
    }

    #[test]
    fn class_negation() {
        // [^0-9]: everything except the digits
        let not_digit = CharClass::new(vec![('0', '9')], true);
        assert!(not_digit.contains('a'));
        assert!(!not_digit.contains('5'));
    }

    #[test]
    fn any_matches_everything() {
        let any = CharClass::any();
        assert!(any.contains('x'));
        assert!(any.contains('\n'));
        assert!(any.contains('🦀'));
    }

    #[test]
    fn ranges_are_normalized() {
        // Overlapping / out-of-order ranges collapse to a sorted disjoint set.
        let cc = CharClass::new(vec![('c', 'e'), ('a', 'c'), ('x', 'x')], false);
        assert_eq!(cc.to_string(), "[a-ex]");
        assert!(cc.contains('b'));
        assert!(!cc.contains('f'));
        assert!(cc.contains('x'));
    }

    #[test]
    fn display_forms() {
        assert_eq!(Reg::any().to_string(), ".");
        assert_eq!(CharClass::range('a', 'z').to_string(), "[a-z]");
        assert_eq!(CharClass::new(vec![('0', '9')], true).to_string(), "[^0-9]");
        assert_eq!(CharClass::single('x').to_string(), "[x]");
    }
}


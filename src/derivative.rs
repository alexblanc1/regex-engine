use crate::ast::Reg;
use crate::ast::Reg::{Empty, Eps, Alt, Seq, Star};

//A regular expression is said nullable if it contains Eps
pub fn nullable(re: &Reg) -> bool {
    match re {
        Reg::Empty => false,
        Reg::Eps => true,
        Reg::Chr(_) => false,
        Reg::Alt(r, s) => nullable(r) || nullable(s),
        Reg::Seq(r, s) => nullable(r) && nullable(s),
        Reg::Star(_) => true,
    }
}


//------------------ --------------------------------------
//*a*^−1^*a*         = *ε*
//*a*^−1^*b*         = ∅
//*a*^−1^*ε*         = ∅
//*a*^−1^∅           = ∅
//*a*^−1^(*R*\*)     = (*a*^−1^*R*)*R*\*
//*a*^−1^(*RS*)      = (*a*^−1^*R*)*S* ∨ ν(*R*)*a*^−1^*S*
//*a*^−1^(*R*∧*S*)   = (*a*^−1^*R*) ∧ (*a*^−1^*S*)
//*a*^−1^(*R*∨*S*)   = (*a*^−1^*R*) ∨ (*a*^−1^*S*)
//*a*^−1^(¬*R*)      = ¬(*a*^−1^*R*)
//------------------ --------------------------------------

pub fn derivative(re: &Reg, chr: &char) -> Reg {
    match re {
        Reg::Empty => Empty,
        Reg::Eps => Empty,
        // a^-1 a = eps, but a^-1 b = empty: the symbol is only consumed if it matches
        Reg::Chr(c) => if c == chr { Eps } else { Empty },
        Reg::Alt(r, s) => Alt(Box::new(derivative(r, chr)), Box::new(derivative(s, chr))),
        // a^-1(RS) = (a^-1 R)S  ∨  ν(R)(a^-1 S)
        // The second branch matters when R can match the empty string:
        // the symbol may then be consumed by S directly.
        Reg::Seq(r, s) => Alt(
            Box::new(Seq(Box::new(derivative(r, chr)), s.clone())),
            Box::new(Seq(Box::new(eta(r)), Box::new(derivative(s, chr)))),
        ),
        // a^-1(R*) = (a^-1 R)R*
        Reg::Star(r) => Seq(Box::new(derivative(r, chr)), Box::new(Star(r.clone()))),
    }
}

// ν(R): Eps if L(R) contains the empty string, Empty otherwise.
// Used in the Seq rule: Seq(Empty, X) matches nothing, Seq(Eps, X) behaves like X.
pub fn eta(re: &Reg) -> Reg {
    if nullable(re) { Eps } else { Empty }
}

// The key theorem: w ∈ L(R) ⇔ ε ∈ L(w^-1 R).
// So to test membership, we derive by each symbol of the word,
// then check nullability of what remains.
pub fn matches(re: &Reg, word: &str) -> bool {
    let mut current = re.clone();
    for c in word.chars() {
        current = derivative(&current, &c);
    }
    nullable(&current)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::Reg::*;

    // Convenience constructor to keep test expressions readable.
    fn chr(c: char) -> Box<Reg> {
        Box::new(Chr(c))
    }

    #[test]
    fn nullable_basics() {
        assert!(!nullable(&Empty));
        assert!(nullable(&Eps));
        assert!(!nullable(&Chr('a')));
        // a* always contains the empty string
        assert!(nullable(&Star(chr('a'))));
        // a|ε is nullable, ab is not
        assert!(nullable(&Alt(chr('a'), Box::new(Eps))));
        assert!(!nullable(&Seq(chr('a'), chr('b'))));
    }

    #[test]
    fn derivative_of_chr() {
        // a^-1 a = ε (nullable), b^-1 a = ∅ (not nullable)
        assert!(nullable(&derivative(&Chr('a'), &'a')));
        assert!(!nullable(&derivative(&Chr('a'), &'b')));
    }

    #[test]
    fn matches_star() {
        // a* accepts "", "a", "aaa", but not "ab"
        let re = Star(chr('a'));
        assert!(matches(&re, ""));
        assert!(matches(&re, "a"));
        assert!(matches(&re, "aaa"));
        assert!(!matches(&re, "ab"));
    }

    #[test]
    fn matches_seq() {
        // ab accepts exactly "ab"
        let re = Seq(chr('a'), chr('b'));
        assert!(matches(&re, "ab"));
        assert!(!matches(&re, ""));
        assert!(!matches(&re, "a"));
        assert!(!matches(&re, "ba"));
        assert!(!matches(&re, "abb"));
    }

    #[test]
    fn matches_seq_with_nullable_prefix() {
        // a*b: this is the case that breaks if the Seq rule
        // forgets the ν(R) branch — "b" must be accepted even
        // though the a* prefix consumes nothing.
        let re = Seq(Box::new(Star(chr('a'))), chr('b'));
        assert!(matches(&re, "b"));
        assert!(matches(&re, "ab"));
        assert!(matches(&re, "aaab"));
        assert!(!matches(&re, ""));
        assert!(!matches(&re, "a"));
        assert!(!matches(&re, "bb"));
    }

    #[test]
    fn matches_alt() {
        // ab | c* accepts "ab", "", "ccc", but not "a" or "abc"
        let re = Alt(
            Box::new(Seq(chr('a'), chr('b'))),
            Box::new(Star(chr('c'))),
        );
        assert!(matches(&re, "ab"));
        assert!(matches(&re, ""));
        assert!(matches(&re, "ccc"));
        assert!(!matches(&re, "a"));
        assert!(!matches(&re, "abc"));
    }
}

use regex_engine::RangeModifier::*;
use regex_engine::Reg::{self, Chr};
use regex_engine::{alt, derivative, matches, seq, star, CharClass};

/// A literal string as a regex: fold its chars with `seq`.
/// `lit("ab")` builds `Seq(Chr('a'), Chr('b'))`; `lit("")` is just ε.
fn lit(s: &str) -> Reg {
    s.chars().fold(Reg::Eps, |acc, c| seq(acc, Chr(c)))
}

/// Print a regex (through its `Display` impl), then the verdict of `matches`
/// for each candidate string.
fn demo(re: &Reg, inputs: &[&str]) {
    println!("{}", re);
    for input in inputs {
        let verdict = if matches(re, input) { "✓ match" } else { "✗ no match" };
        println!("    {:<12} {}", format!("{:?}", input), verdict);
    }
    println!();
}

fn main() {
    // ----- Phase 1: the core (Chr / Alt / Seq / Star) -----

    // a* | ba, built through the smart constructors
    demo(
        &alt(star(Chr('a')), seq(Chr('b'), Chr('a'))),
        &["", "aaa", "ba", "b", "ab"],
    );

    // a*b — needs the ν(R) branch of the Seq rule: the a* prefix may match nothing
    demo(&seq(star(Chr('a')), Chr('b')), &["b", "aaab", "a", ""]);

    // ----- Phase 2: character classes and quantifiers -----

    // [a-z]+ — a run of lowercase letters
    demo(
        &Reg::Class(CharClass::range('a', 'z')).apply(OneOrMore),
        &["hello", "Hello", ""],
    );

    // .{3} — any three characters (chars, not bytes: a crab counts as one)
    demo(&Reg::any().apply(Repeat(3)), &["abc", "🦀🦀🦀", "ab"]);

    // colou?r — both spellings, one regex
    demo(
        &seq(lit("colo"), seq(Chr('u').apply(ZeroOrOne), Chr('r'))),
        &["color", "colour", "colouur"],
    );

    // #[0-9a-f]{6} — a CSS hex color
    let hex_digit = Reg::Class(CharClass::new(vec![('0', '9'), ('a', 'f')], false));
    demo(
        &seq(Chr('#'), hex_digit.apply(Repeat(6))),
        &["#ff8800", "#ff88", "ff8800", "#ff88zz"],
    );

    // ----- Derivatives at work -----

    // b^-1(a* | ba) comes out fully simplified: just `a`
    let expr = alt(star(Chr('a')), seq(Chr('b'), Chr('a')));
    println!("d/db {}  =  {}", expr, derivative(&expr, &'b'));
}

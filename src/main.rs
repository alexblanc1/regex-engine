use regex_engine::Reg::Chr;
use regex_engine::{alt, derivative, nullable, seq, star};

fn main() {
    // a* | ba, built through the smart constructors
    let expr = alt(star(Chr('a')), seq(Chr('b'), Chr('a')));
    println!("{}", nullable(&expr));
    // b^-1(a* | ba) now comes out fully simplified: just `a`
    println!("{}", derivative(&expr, &'b'));
}

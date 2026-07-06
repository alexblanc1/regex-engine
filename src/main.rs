use regex_engine::Reg::{Chr, Alt, Seq, Star};
use regex_engine::{derivative, nullable};

fn main() {
    let a = Box::new(Chr('a'));
    let b = Box::new(Chr('b'));
    // a* | ba
    let expr = Alt(Box::new(Star(a.clone())), Box::new(Seq(b, a.clone())));
    println!("{}", nullable(&a));
    println!("{}", derivative(&expr, &'b'));
}

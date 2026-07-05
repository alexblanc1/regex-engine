pub enum Reg {
    Empty,
    Eps,
    Chr(char),
    Alt(Box<Reg>, Box<Reg>),
    Seq(Box<Reg>, Box<Reg>),
    Star(Box<Reg>),
}



fn nullable(re: &Reg) -> bool {
    match re {
        Reg::Empty => false,
        Reg::Eps => true,
        Reg::Chr(_) => false,
        Reg::Alt(r, s) => nullable(r) || nullable(s),
        Reg::Seq(r, s) => nullable(r) && nullable(s),
        Reg::Star(_) => true,
    }
}



fn main() {
    let a = Reg::Empty;
    println!("{}", nullable(&a));
}

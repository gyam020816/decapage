#[derive(Debug)]
enum Something {
    Blah,
    Derp(u32),
}

impl Something {
    fn sq(&self) -> Something {
        match self {
            Something::Blah => Something::Blah,
            Something::Derp(4) => Something::Blah,
            Something::Derp(1) => Something::Derp(1),
            Something::Derp(a) => Something::Derp(a / 2),
        }
    }
}

fn main() {
    let mut x = Something::Derp(256);
    for value in 0..100 {
        println!("{:?}", x);
        x = x.sq();
        match x {
            Something::Blah => break,
            Something::Derp(1) => break,
            _ => ()
        }
    }
    println!("{:?}", x);
}

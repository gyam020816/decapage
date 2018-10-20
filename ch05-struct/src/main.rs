#[derive(Debug)]
struct Surface {
    width: u32,
    height: u32,
}

impl Surface {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect = Surface {
        width: 30,
        height: 50,
    };

    println!("rect is {:#?}", &rect);
    println!(
        "The area is {} sq units",
        &rect.area()
    );
}

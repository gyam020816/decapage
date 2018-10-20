struct Surface {
    width: u32,
    height: u32,
}

fn main() {
    let rect = Surface {
        width: 30,
        height: 50,
    };

    println!(
        "The area is {} sq units",
        area(&rect)
    );
}

fn area(surface: &Surface) -> u32 {
    surface.width * surface.height
}

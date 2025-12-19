// A function `age` which returns a `u32`.
fn age() -> u32 {
    67
}

fn main() {
    println!("Tell me what type of person you are");

    match age() {
        0             => println!("I haven't celebrated my first birthday yet"),
        // The `@` operator binds the matched value to a variable while also matching a pattern.
        // Here, `n @ 1..=12` matches any value between 1-12 AND binds that value to `n`,
        // allowing us to use the actual age in the message.
        n @ 1  ..= 12 => println!("I'm a child of age {:?}", n),
        n @ 13 ..= 19 => println!("I'm a teen of age {:?}", n),
        // Catch-all pattern: matches any remaining value and binds it to `n`
        n             => println!("I'm an old person of age {:?}", n),
    }
}
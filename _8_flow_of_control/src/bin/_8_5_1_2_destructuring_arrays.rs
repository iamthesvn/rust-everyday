fn main() {
    let array = [100, -2, 6, 1, 2, 3];

    match array {
        [0, second, third, _, _, _] =>
            println!("array[0] = 0, array[1] = {}, array[2] = {}", second, third),

        [1, _, third, _, _, _] => println!(
            "array[0] = 1, array[2] = {} and array[1] was ignored",
            third
        ),

        [-1, second, ..] => println!(
            "array[0] = -1, array[1] = {} and all the other ones were ignored",
            second
        ),

        [3, second, tail @ ..] => println!(
            "array[0] = 3, array[1] = {} and the other elements were {:?}",
            second, tail
        ),

        [first, middle @ .., last] => println!(
            "array[0] = {}, middle = {:?}, array[2] = {}",
            first, middle, last
        ),
    }
}
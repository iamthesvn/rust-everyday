fn main(){
    let x = 5u32;

    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;
        //since the following does not end with a semicolon, it is an expression and will be assigned to y
        x_cube + x_squared + x
    };

    let z = {
        //since the following ends with a semicolon, it is a statement and will not be assigned to z
        2 * x;
    };

    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);
}
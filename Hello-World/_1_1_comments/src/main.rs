fn main() {
    //Trying out the comments feature

    /* Trying the multiline comment feature
    aka block comments. Block comments are extremely useful for temporarily disabling chunks of code. */

    //Both these types of comments are not read by the compiler

    //* <- add another '/' before the 1st one to uncomment the whole block

    println!("Now");
    println!("everything");
    println!("executes!");
    // line comments inside are not affected by either state

    // */
    let x = 5 + /* 90 + */ 5;
    println!("Is `x` 10 or 100? x = {}", x);
}

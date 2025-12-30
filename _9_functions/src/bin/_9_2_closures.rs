fn main() {
    let outer_var = 42;

    //fn function(i: i32) -> i32 { i + outer_var }
    // TODO: uncommenting the line above will throw an error. The compiler suggests that we define a closure instead.

    let closure_annotated = |i: i32| -> i32 { i + outer_var };
    let closure_inferred  = |i     |          i + outer_var  ;

    println!("closure_annotated: {}", closure_annotated(1));
    println!("closure_inferred: {}", closure_inferred(1));

    let one = || 1;
    println!("closure returning one: {}", one());

}
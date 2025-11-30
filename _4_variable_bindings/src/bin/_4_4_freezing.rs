fn main(){
    let mut _mutable_integer = 7i32;

    {
        let _mutable_integer = _mutable_integer;

        //_mutable_integer = 50; //this fails to compie as _mutable_integer is frozen
        println!("inner block: {}", _mutable_integer); 
    }
    _mutable_integer = 3;
    println!("outer block: {}", _mutable_integer);
}
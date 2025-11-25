fn main(){
    let elem = 5u8;

    let mut vec = Vec::new();
    vec.push(elem);
    //the compiler now knows that vec is a vector of u8

    println!("{:?}", vec);
}
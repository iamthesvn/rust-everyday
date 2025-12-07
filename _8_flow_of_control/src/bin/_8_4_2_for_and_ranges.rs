fn main(){
    //n takes on values 1, 2, 3, ..., 100 for each iteration
    for n in 1..=100{
        if n % 15 == 0{
            println!("FizzBuzz");
        } else if n % 3 == 0{
            println!("Fizz");
        } else if n % 5 == 0{
            println!("Buzz");
        } else {
            println!("{}", n);
        }
    }
}
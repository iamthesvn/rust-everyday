fn main(){
    let mut counter = 1;

    while counter < 101{
        if counter % 15 == 0{
            println!("FizzBuzz");
        } else if counter % 3 == 0{
            println!("Fizz");
        } else if counter % 5 == 0{
            println!("Buzz");
        } else {
            println!("{}", counter);
        }

        counter += 1;
    }
}
#[allow(dead_code)]
enum Temperature{
    Celcius(i32),
    Fahrenheit(i32),
}

fn main(){
    let temperature = Temperature::Fahrenheit(32);

    match temperature{
        Temperature::Celcius(t) if t > 30 => println!("{}C is above 30 degrees", t),
        Temperature::Celcius(t) => println!("{}C is equal to or below 30 degrees", t),
        Temperature::Fahrenheit(t) if t > 86 => println!("{}F is above 86 degrees", t),
        Temperature::Fahrenheit(t) => println!("{}F is equal to or below 86 degrees", t),
    }
}
use std::io;

mod converter;
mod test;

fn main()  {
    println!("Please enter your weight ");
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
                converter::weight_converter::convert_weight(input);
            },
        Err(error) => {
            println!("{}", error.to_string());
        }
    }
}



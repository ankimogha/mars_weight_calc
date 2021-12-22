use std::io;

pub fn convert_weight () {
    println!("Please enter your weight ");
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            match input.trim().parse::<f32>() {
                Ok(weight_on_earth) => println!("Weight on earth is : {} kg and Weight on mars is  : {} kg", weight_on_earth as f32, calculate_weight_on_mars(weight_on_earth as f32)),
                Err(error) => println!("{}",error),
            };
        }
        Err(error) => {
            println!("{}",error.to_string());
        }
    }
}

fn calculate_weight_on_mars(weight_on_earth:f32) -> f32{
    (weight_on_earth / 9.81) * 3.711
}
///  convert_weight function checks value returned from calculate_weight_on_mars function
///
/// # Arguments
///
/// * `input` - A user input value.
///
/// # Return
///
/// A String.
pub fn convert_weight (input:String) -> String{
    let mut result = String::new();
            match input.trim().parse::<f32>() {
                Ok(weight_on_earth) => {
                    println!("Weight on earth is : {} kg and Weight on mars is  : {} kg", weight_on_earth as f32, calculate_weight_on_mars(weight_on_earth as f32));
                    result.push_str(&*input);
                },
                Err(error) =>  {
                    println!("{}",error.to_string());
                    result.push_str(&*error.to_string());
                },
            }
            result
        }

///  calculate_weight_on_mars function converts weight on Earth to weight of Mars
///
/// # Arguments
///
/// * `weight_on_earth` - Weight of person on Earth.
///
/// # Return
///
/// Weight of person on Mars.
pub fn calculate_weight_on_mars(weight_on_earth:f32) -> f32{
    (weight_on_earth / 9.81) * 3.711
}

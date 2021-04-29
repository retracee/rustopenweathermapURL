use std::io::Write;
use reqwest;
use reqwest::Error;
fn main(){
    print!("\nEnter your city: "); std::io::stdout().flush().expect("Err");
    let mut city = String::new(); std::io::stdin().read_line(&mut city).unwrap();
    print!("Enter your API Key: "); std::io::stdout().flush().expect("Err");
    let mut apikey = String::new(); std::io::stdin().read_line(&mut apikey).unwrap();
    corrector(city,apikey);
}

fn corrector(city: String, apikey: String){
    println!("\n\nCity: {}",city);
    println!("API Key: {}",apikey);
    print!("Is this correct? [y/n]: "); std::io::stdout().flush().expect("Err");
    let url = format!("http://api.openweathermap.org/data/2.5/weather?q={}&units=imperial&appid={}",city.trim(),apikey.trim());
    let mut yn = String::new(); std::io::stdin().read_line(&mut yn).unwrap();
    if yn.trim() == "y" {
       requester(url).unwrap();
    } else if yn.trim() == "n" {
        println!("\n\n\n"); main();
    }else{
        println!("\n\nInput not understood, please use Y or N."); corrector(city,apikey);
    }
}

fn requester(url: String)  -> Result<reqwest::blocking::Response, Error>{
    let api = reqwest::blocking::get(&url)?;
    if api.status().is_success(){
        println!("\nSuccess! The API was reached.");
        println!("\n\n Your URL: {}",url)
    }   else{
        println!("\n[ERROR] With your credentials the API was unreachable. Please check what you have entered and try again.");
    }
       Ok(api)
}

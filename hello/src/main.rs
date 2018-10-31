use std::io::Write;
use std::str::FromStr;

fn pirate_share(total: u64, crew_size: usize ) -> u64 {
    let half = total / 2;
    half / crew_size as u64
}

//fn get_weather(location: LatLng) -> Result<WeatherReport, io::Error> {
//}

fn main() {
    println!("Hello, world!");
    let mut numbers:Vec<u64> = Vec::new();
    //numbers.push(11);
    for arg in std::env::args().skip(1){
        numbers.push( u64::from_str(&arg).expect("error parsing argument dayo") )
    }
    if numbers.len() == 0 {
        //writeln!( std::io::stderr(), "Usage: gcd Number ....").unwrap();
        writeln!( std::io::stderr(), "Usage: gcd Number ....").is_ok();
        std::process::exit(1);
    }
    println!("ok, world!");    
}

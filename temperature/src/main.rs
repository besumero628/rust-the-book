fn main() {
    println!("Hello, world!");
    let temperature: f64 =300.15; //絶対温度
    trans_celsius(temperature);
    trans_fahrenheit(temperature);
}

fn trans_celsius(temperature: f64) {
    let c: f64 = ((temperature -273.15) * 100.0).round() / 100.0;
    println!("{}[°C]", c);
}

fn trans_fahrenheit(temperature: f64) {
    let c: f64 = temperature -273.15;
    let f :f64 = ((c * 1.8 + 32.0) * 100.0).round() / 100.0;
    println!("{}[°F]", f);
}
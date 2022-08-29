use std::io;

fn main() {
    println!("Please enter your weight (Kg): ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();  

    let wieght: f32 = input.trim().parse().unwrap();


    let mars_wieght = weight_on_mars(wieght);
    println!("Weight on mars: {}", mars_wieght);
}

fn weight_on_mars(a: f32) -> f32{
    (a/9.81) * 3.711
}

use std::io;

fn main() {
    println!("Enter your weight (kg)");
    let mut input_weight = String::new();
    io::stdin().read_line(&mut input_weight).unwrap();
    let weight:f32 = input_weight.trim().parse().unwrap();
    dbg!(weight);
   
    println!("Weight on mars! : {} kg",calc_weight_on_mars(weight));
}
fn calc_weight_on_mars(weight:f32) -> f32{
   (weight / 9.81) * 3.711
}
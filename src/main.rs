use std::io;

fn main() {
    let mut input_weight = String::new();
    let mut s = input_weight;
    io::stdin().read_line(&mut input_weight);
    let weight = calc_weight_on_mars(input_weight);
    println!("Weight on mars! : {} kg",weight);
}
fn calc_weight_on_mars(weight:f32) -> f32{
   (weight / 9.81) * 3.711
}

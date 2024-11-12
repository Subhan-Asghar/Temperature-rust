use std::io;
fn main() {
    println!("Enter the Fahrenheit ");
    let mut fah=String::new();
    io::stdin().read_line(&mut fah).expect("Failed");
    let fah:f32=match fah.trim().parse(){
        Ok(num)=>num,
        Err(_)=>{
            println!("Error");
            0.0
        }
    };
    let cel:f32=(fah-32.0)*5.0/9.0;
    println!("The Celsius is {}",cel);
    
}

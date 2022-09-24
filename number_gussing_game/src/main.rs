use std::io;
fn main() {
    println!("Guss the number !");
    println!("Please input your gusss.");
    let mut guss = String::new();
    io::stdin().read_line(&mut guss)
            .expect("Failed to read the line");
    println!("You Gussed : {} ",guss)
}

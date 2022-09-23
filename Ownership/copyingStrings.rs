fn main() {
    let s = String::from("Hello");
    // let s1 = s; This line makes the s invalid instead we can do the followng
    let s1 = s.clone(); 
    println!("{} {}" ,s,s1);//Gives the error because s is no longer valid after the copy

}
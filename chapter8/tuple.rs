//Storing sun objects in ojects
fn main() {
    let data = (1003, 23.434,"hello");
    //type of tuple can be made explicit like below
    //let data:(i32,f64,char)
    let copy_of_data = data;
    println!("{}, {}, {}", data.0, copy_of_data.1, data.2);
}
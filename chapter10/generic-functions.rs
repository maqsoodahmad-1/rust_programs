fn main() {
let a :i16 = f::<i16>('a',32,34);
let b :f64 = f::<f64>('a',32.2,33.2);
println!("{} {}",a,b)
}
fn f <T> (ch:char , num1:T, num2:T)->T{
    if ch == 'a'{ num1 }
    else{ num2 }
}

fn main() {
    let mut x = [4.;5000];
    x[200] = 3.1415;
    print!("{} {} {}", x[100],x[1000], x[200]);
}

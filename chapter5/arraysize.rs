fn main()
{
    let mut x = [4;5000];
    x[1000] = 100;
    println!("{} {}",x[100],x[1000]);
}

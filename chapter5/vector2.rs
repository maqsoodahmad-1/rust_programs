fn main()
{
    let length = 5000;
    let mut y = vec![4.; length];
    y[6] = 3.14;
    y.push(4.89);
    println!("{}, {}, {}", y[6], y[4999], y[5000]);
}

fn main()
{
    let mut x = ["a", "b","c"];
    print!("{} {} {}.",x[0], x[1],x[2]);
    x = ["x", "y", "z"];
    print!("{} {} {}.",x[0],x[1],x[2]);
    let y = ["1", "2", "3"];
    x = y;
    println!("{} {} {}",x[0],x[1],x[2]);
}

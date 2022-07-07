fn main()
{
    let mut x = vec!["This", "is", "a", "sentence"];
    println!("{} ",x[1]);
    x.insert(1,"line");
    println!("{} ",x[1]);
    println!("{} ",x[2]);
    x.insert(2, "contains");
    println!("{} ",x[2]);
    println!("{} ",x[3]);
    x.remove(3);
    println!("{} ",x[3]);
    x.push("about Rust");
    println!("{} ",x[4]);
    x.pop();
    println!("---------------------");
    for i in 0..x.len()
    {
	print!("{} ",x[i]);
    }
    println!();
}

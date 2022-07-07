fn main()
{
    let mut limit = 4;
    for _i in 1..limit
    {
	limit -= 1;
	print!("{} ", limit);
    }
    println!(":{}", limit);
}

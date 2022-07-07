fn main()
{
    let mut i = 0;
    while i < 50
    {
      i+=1;
      if i % 3 != 0 
      {	
	 if i * i <= 400 
	 { 
	    print!("{} ",i * i);
	 }
      }
    }
    println!();
}

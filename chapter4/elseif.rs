fn main()
{
    let n = 4 ;
    if n > 100
    {
         println!("{} is big",n);
    }
    else if n > 0 
    {
         println!("{} is small",n);
    }
    else if n < 0 
    {
        println!("{} is negative ",n);
    }
    else
    {
        println!("{} is neither positive nor negative",n);
    }
}


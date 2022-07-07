fn main()
{
   /*1.! has the highest priority
     2.&& has has the second highest priority
     3.|| has the third highest priority
   */
    println!("{} {} ",true || true && !true, (true || true) && !true);
}

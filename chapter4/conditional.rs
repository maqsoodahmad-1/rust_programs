fn main()
{
    let n = 4;
    println!("{}",
          if n > 1000{
	   	"big"
		}
          else if n > 0 {
             "small"
	  } 
          else if n < 0 {
              "negative"
          }
          else {
	     "zero"
          }
      );
}

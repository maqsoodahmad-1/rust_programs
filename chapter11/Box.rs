let a = 7,  mut a_box:Box<i32>, a_ref:&i32 = &a;
println!("{} {};",a, *a_ref);
a_box = Box::new(a + 2 );
a_ref = &*a_box;
println!("{} {} {}",a, *a_ref, *a_box);

fn main()
{
    let truth = true;
    let falsity = false;
    println!("{} {}",!truth, !falsity);
    println!("{} {} {} {}",truth && truth, falsity && falsity, truth && falsity ,falsity && truth );
    println!("{} {} {} {} ",truth || falsity, truth || truth, falsity || truth, falsity || falsity);
}


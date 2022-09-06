fn main() {
    for n in -2..5 {
        println!("{} is {}.",n,match n {
            0 => "Zero",
            1 => "one",
            _ if (n < 0) => "negative",
            _ => "Plural",
        });
    }
}
// // instead of writing the following code 
// const EUROPE:u8 = 0;
// const ASIA: u8 = 1;
// const AFRICA: u8 = 2;
// const AMERICA:u8 = 3; 
// const OCEANIA:u8 = 4;

// let continent = ASIA;
// if continent == EUROPE {print!("E");}
// else if continent == ASIA {print!("As");}
// else if continent == AFRICA {print!("Af");}
// else if continent == AMERICA {print!("Am");}
// else if continent == OCEANIA {print!("O");}

//it is better to write 
fn main() {
enum Continent {
    Europe,
    Asia,
    Africa,
    America,
    Oceania,
}

let contin = Continent:: Asia;
let _contin1 = Continent:: Africa;
let _contin2 = Continent:: America;
let _contin3 = Continent:: Europe;
let _contin4 = Continent:: Oceania;

match contin {
    Continent::Europe => println!("E"),
    Continent::Asia => println!("As"),
    Continent::Africa => println!("Af"),
    Continent::America => println!("Am"),
    Continent::Oceania => println!("O"),
}
}
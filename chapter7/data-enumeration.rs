fn main (){
enum Result {
    Success(f64),
    Failure(u16, char),
    Uncertainity,
}

let _outcome1 = Result::Success(23.123);
let outcome = Result::Failure(1200,'X');
let _outcome2 = Result::Uncertainity;

match outcome {
    Result::Success(value) => println!("Result: {} ", value),
    Result::Failure(error_code, module) => println!("Error n. {} i module {}",error_code,module),
    Result::Uncertainity => {},
}
}
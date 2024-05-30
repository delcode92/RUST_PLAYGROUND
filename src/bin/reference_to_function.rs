fn display_country(c:String){
    println!("{}", c);
}

fn main() {
    let x = String::from("indonesia");
    
    let y = &x;
    let z = &x;
  
    println!("{} {}", y, z);
    //display_country(x);
    
    //let s:&String = &x;

    // could not pass x as argument,  because it as reference has already being used
    // must use .clone() on it
    // display_country(x);
}

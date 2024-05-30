fn display_country(c:String){
    println!("{}", c);
}

fn display_country2(c:&String){
    println!("from display_country2 {}", c);
}

fn display_country3(c:String){
    println!("from display_country3 {}", c);
}

fn main() {
    let x = String::from("indonesia");
    
    //let y = &x;
    //let z = &x;
  
    //println!("{} {}", y, z);
    //display_country(x);
   
    // if x pass to display_country3() it's mean move `x` to that function  
    //display_country3(x);
    
    // so it can't be print `x` value below because it has been moved
    //println!("x value:{}", x);

    // but if your pass it as reference, it still can be access on this scope, 
    // because we just pass the reference not the real value
    display_country2(&x);
    display_country2(&x);

    println!("x value: {}", x);

    //display_country3(x);

    //let s:&String = &x;

    // could not pass x as argument,  because it as reference has already being used
    // must use .clone() on it
    // display_country(x);
}

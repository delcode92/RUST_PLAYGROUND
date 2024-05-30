fn main() {
   let mut my_number = 8;
    let num_ref = &mut my_number;

    // can't do 
    //num_ref+=10;
    
    // but like this:
    *num_ref += 10;

    println!("{}", num_ref);
}


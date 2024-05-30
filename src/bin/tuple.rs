// this fucnction actually return ()
fn do_something1(){
    
}

// it is the shorthand for function below
fn do_something2()->(){
    
}


fn main() {

    // just random tuple:
    let random_tuple = (1,2,4, "test", "coba");
    println!("random tuple: {} {} {}", random_tuple.0, random_tuple.3, random_tuple.2);

    let v:Vec<i32> = vec![1,2,3,4];

    // destructuring with only three variables 
    let(a,b,c) = ( v[0], v[1], v[2] );

    // destructuring with all variables name
    let(a1,b1,c1,d1) = ( v[0], v[1], v[2], v[3] );
    
    // destructuring just two variables
    let(a2,b2,_,_) = ( v[0], v[1], v[2], v[3] );

    println!("{}", a);
    println!("{}", b);
    //println!("{}", c);
    //println!("{}", d);
}

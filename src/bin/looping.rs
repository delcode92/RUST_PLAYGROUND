fn main() {
    
    let mut i:i32 = 0;

    // using loop
    println!("====== using loop ======");
    loop{
        println!("{}", i);
        i+=1;

        if i==10 {
            break;
        }
    }


    // using for
    println!("====== using for ======");
    for n in 0..5{
        println!("{}", n);
    }

    // using while
    println!("====== using while ======");
    let mut j = 0;

    while j<=5 {
        println!("{}", j);
        j+=1;
    }
}   

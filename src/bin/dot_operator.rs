struct Number{
    num:u8
}


fn main() {
    let n = 8;
    let n_reference = &n;

    println!("{}", *n_reference);
    println!("{}", *n_reference);


    // init struct
    let x = Number{ num:120 };
    let x_reference = &x;

    // when destructuring struct reference , more easy and readable if using dot operator
    println!("{}", x_reference.num);

}

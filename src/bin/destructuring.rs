struct Person{
    name:String,
    addr:String,
    age:u8,
    hp:i32
}

fn main() {
    let person1 = Person{
        name: String::from("budi"),
        addr: String::from("lampaseh"),
        age: 20,
        hp: 123345
    };


    // destructuring that struct above
    // put `let` keyword on backward
    /*
    let Person{
        name: a,
        addr: b,
        age: c,
        hp: d
    } = person1;
    */

    // could use this shorthand,  without add more variable name
    let Person{
        name,
        addr,
        age,
        hp
    } = person1;

    println!("name: {}, address: {}", name, addr);
}

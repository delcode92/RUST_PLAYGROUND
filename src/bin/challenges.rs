fn main() {
    // reverse_array();
    let mut arr = [1, 2, 3, 4, 5];


    reverse_array2( &mut arr );

    println!("{:?}", arr);
}

/*
fn reverse_array(){
    let arr1: [i32; 4] = [1,2,3,4];
    let length = arr1.len()-1;
    let mut v = Vec::new();
    // println!("{:?}", arr1.len());
    // let v
    for (index,_element) in arr1.iter().enumerate(){
        // println!("{:?}", arr1[length-index]);
        v.push( arr1[length-index] );   
    }

    println!("{:?}", v);
}
*/

fn reverse_array2<T>(arr: &mut [T]) {
    let len = arr.len();
    for i in 0..len / 2 {
        arr.swap(i, len - 1 - i);
    }
}

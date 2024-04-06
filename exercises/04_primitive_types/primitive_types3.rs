// primitive_types3.rs
//
// Create an array with at least 100 elements in it where the ??? is.
//
// Execute `rustlings hint primitive_types3` or use the `hint` watch subcommand
// for a hint.


fn main() {
    // let mut a = Vec::new();
    // for i in 0..100 {
    //     a.push(i);
    // }
    let a = ["wow";100];

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
        // println!("{}", a[0]); // ["wow", ... , "wow"]
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
        panic!("Array not big enough, more elements needed")
    }
}

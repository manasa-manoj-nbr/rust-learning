fn main() {
    let a = random_number().wrapping_add(57); //wrapping add does the mod
    println!("Value of a is {a}");
    // let b = match random_number().checked_add(57){ //will check whether addition ispossible or not 
    //     Some(num)=>num,
    //     None =>{
    //         println!("Cannot add");
    //             return;
    //     }
    // };
    // println!("Value of b is {b}");
    let (c,d):(u8,bool) = random_number().overflowing_add(57);  //d boolean give true if value is overflowed val
    println!("Value of c is {c} d is {d}");

    //tuple can consist element of diff values
    let manasa = ("Manasa", 20, 1_00_00_000);
    let(x,y,z) = manasa;
    println!("x is {x} y is {y} zis {z}");
    println!("0th value is {}", manasa.0);
    println!("1th value is {}", manasa.1);
    println!("2th value is {}", manasa.2);

    //arrays . should have same datatype
    let a : [i32;6] = [1,2,3,4,5,6];
    let b : [i32;5] = [10;5]; //5 size fill with 5 10
    println!("fiest el in a  {} and in b {}", a[0], b[0]);
}

fn random_number()-> u8 {
    200
}

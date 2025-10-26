fn main() {
    let age = 24; //in rust variables are by deafult immutable
    let mut age2 = 19; //add mut to make it mutable
    println!("My Age is {age2}");
    age2 = 20;
    println!("My Age is {age}");
    println!("My Age is {age2}");

    const PI: u8 = 10; //always give type for constant
    println!("PI value is {PI}");

    const THREE_HOUR_IN_SEC: u32 = 60*60*3;
//    const THREE_HOUR_IN_SEC_2: u32 = 60*60*3+age; give error as non-constant value is used. either make age const then no error
    println!("{THREE_HOUR_IN_SEC}");

    //shadowing
    let apple = 20;
    println!("apple = {apple}");
    let apple = apple +20;
    println!("apple = {apple}");

    shadow();
}
 
 fn shadow(){
    let x = 5;
    let x=x+1;
    {
        let x=x*2;
        println!("The value of x in inner scope is {x}");
    }
    println!("The value of x is {x}");
 }
fn main() {
    let x: i32 = 5; //imutabble variable
    let _y: i32; //unused varible _y

    assert_eq!(x, 5);
    println!("success");

    let mut x = 5; //mutabble variable
    x += 1; //x = x +1

    assert_eq!(x, 6);
    println!("success");

    let a: i32 = 4;
    let y: i32 = 7;

    {
        println!("The value of  a is {} and the value of is is {}", a, y)
    }

    println!("The value of a is {} and the value of is {}", a, y); //error: cannot find value `y` in this scope cause variable is only valid inside the function scope but to solve the problem, the y variable should be a global variable

    let z: i32 = 5;

    {
        let z = 20;
        assert_eq!(z, 20); //returns 20 cause the z variable is only valid inside the function scope
    }
    assert_eq!(z, 5); //retuns 5 cause the z variable is only valid inside the main function scope

    let z: i32 = 42; //shadowing the z variable....
    println!("{}", z);

    let mut g: i32 = 3;
    g = 5; //x = 5, reassigning a value to d variable...
    //shadiwng and re-binding
    let mut g: i32 = 4; //the variable is not longer mutable, the solution is to make it mutable
    g = g + 1; //error: cannot assign twice to immutable variable `d` cause the d variable is immutable and cannot be reassigned a value

    let f = 12;
    let f = "I can also be bound to text!"; //shadowing the f variable
    println!("successfully bound"); 

    let(mut i, j) = (1, 2);
    i += 3;
    assert_eq!(i , 4);
    assert_eq!(j, 2);

    println!("success"); 



    define_x(); //invoking the define_x function cause the starting point of every rust program is the main function.
}



fn define_x() {
    let x: &str = "hello";
    println!("{},world", x) //returns hello world cause we provided x the the print line macro and we are appending to it comma world
}

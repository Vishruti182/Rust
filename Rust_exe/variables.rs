// exercise 1

// fn main(){
//     let x=5;
//     //x=10;
//     println!("{}",x);
//     let mut y=3;
//     println!("{}",y);
//     y=7;
//     println!("{}",y);
//     let z=x+y;
//     println!("{}",z);
// }


// exercise 2

// fn main(){
//     let x=5;
//     println!("{}",x);
//     let x=x*2;
//     println!("{}",x);
//     let x="five";
//     println!("{}",x);
// //     let y="5";
// //     let y=y+10;
// //     println!("{}",y);
// }


// exercise 3

// const MAX_POINT: u32 = 100_000;

// fn main() {
//     // 1. Print the global constant
//     println!("{}", MAX_POINT);

//     // Constants cannot be reassigned
//     // MAX_POINT = 111_111; // ERROR!

//     // 2. Declare a local constant
//     const MIN_POINT: u32 = 10;
//     println!("{}", MIN_POINT);

//     // 3. Declare a local variable using snake_case
//     let max_point = 111_111;

//     println!("{}", max_point);
// }


// exercise 4

// fn main(){
//     let a : i32 = 42;
//     let b : f64 = 3.14;
//     let c : bool = true;
//     let d : char = 'R';
//     //let e : i32 = 3.14;
//     //let f : f64 = 42;
//     println!("{ }",a);
//     println!("{ }",b);
//     println!("{ }",c);
//     println!("{ }",d);
//     //println!("{ }",e);
//     //println!("{ }",f);
// }


// exercise 5

// fn main(){

//     let outer : i32 = 10;
//     println!("{ }",outer);

//     {
//         let inner : i32 = 5;
//         println!("{ }",inner);
        
//         let outer = "ten";
//         println!("{ }",outer);
//     }

//     println!("{ }",outer);
//     //println!("{ }",inner);  !ERROR
// }


// exercise 6

fn main(){
    let count : i32 = 0;
    println!("{ }",count);

    let count = count + 3;
    println!("{ }",count);

    let mut count = count;
    count = count +2;
    println!("{ }",count);

    let count = "five";
    println!("{ }",count);
}
// exercise 1

// fn main(){
    
//     let triple = | x | x * 3;

//     println!("5 tripled: { }",triple(5));
//     println!("5 tripled: { }",triple(-2));
// }


// exercise 2

// fn main(){
//     let n=10;

//     let add_n = | x | x + n ; 

//     println!("5 + {} = {}", n, add_n(5));

//     let n=-3;
//     let add_n = | x | x + n ; 
//     println!("5 + {} = {}", n, add_n(5));
// }


// exercise 3

// fn apply_operation(f: impl Fn(i32) -> i32, x: i32) -> i32 {
//     f(x)
// }

// fn main() {
//     let add_five = |x| x + 5;
//     let double = |x| x * 2;

//     println!("{}", apply_operation(add_five, 10));

//     println!("{}", apply_operation(double, 10));
// }


// exercise 4

// fn main() {
//     let result: i32 = (1..=20)
//         .filter(|x| x % 2 == 0)
//         .map(|x| x * x)
//         .sum();

//     println!("Sum of squares of even numbers: {}", result);
// }


// exercise 5

fn main() {
    let name = String::from("Alice");
    
    let greet_once = move || {
        println!("Hello {}", name);
        name 
    };

    let recovered_name = greet_once(); 

    println!("Goodbye {}", recovered_name);

    let mut counter=0;
    let mut increment = || {
        counter+=1;
        println!("Counter: {}",counter);
    };
    increment();
    increment();

    let read_only=|x| x+counter;
    println!("Read: {}",read_only(5));
}

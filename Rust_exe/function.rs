// exercise 1

// fn main(){
//     greetings("John");
// }
// fn greetings(value:&str){
//     println!("Hello {}!",value);
// }


// exercise 2

// fn main(){
//     println!("Area: {}",calculate_area(3.5,4.2));
//     println!("Area (negative): {}",calculate_area(-1.0,5.0))
// }
// fn calculate_area(height:f64, width:f64)->f64{
//     if height<0.0 || width<0.0 {
//         return 0.0;
//     }
//     height*width
// }


// exercise 3

// fn main(){
//     println!("Is 4 even?: {}",is_even(4));
//     println!("Is 7 even?: {}",is_even(7));
// }

// fn is_even(num:i32)->bool{
//     num%2==0
// }


// exercise 4

// fn main(){
//     println!("5! = {}",factorial(5));
//     println!("0! = {}",factorial(0));
// }

// fn factorial(num:i32)->i32{
//     if num<=1{
//         return 1;
//     }
//     return factorial(num-1)*num;
// }


// exercise 5

// fn main(){
//     let numbers=vec![34,12,78,3];
//     let words=vec!["apple","banana","cherry"];

//     println!("Max number: {:?}", find_max(&numbers));
//     println!("Max word: {:?}", find_max(&words));
    
//     // Testing the empty array constraint
//     let empty_vec: Vec<i32> = vec![];
//     println!("Max empty: {:?}", find_max(&empty_vec)); // Prints: None
// }

// // Takes a slice of any type T that implements PartialOrd, returns Option<&T>
// fn find_max<T: PartialOrd>(slice: &[T]) -> Option<&T> {
//     // If the slice is empty, returns None
//     if slice.is_empty() {
//         return None;
//     }

//     // Assume the first element is the maximum to begin with
//     let mut max = &slice[0];

//     // Iterate through the rest of the slice elements
//     for item in slice.iter() {
//         if item > max {
//             max = item;
//         }
//     }

//     // Wrap the reference in Some()
//     Some(max)
// }


// exercise 6

fn main(){
    let increment=|x| x+1;
    println!("Apply twice: {}",apply_twice(increment,5));
}

fn apply_twice<F>(f: F, value: i32) -> i32
where
    F: Fn(i32) -> i32,
{
    f(f(value))
}

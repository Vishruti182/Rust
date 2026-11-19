// exercise 1

// fn main(){
//     let mut counter=0;
//     loop{
//         println!("{}",counter);
//         counter+=2;
//         if counter>10{
//             break;
//         }
//     }
// }


// exercise 2

// // fn main(){
// //     let mut count = 5;
// //     while count>=1 {
// //         println!("{}",count);
        
// //         if count==1 {
// //             println!("Liftoff!");
// //         }

// //         count-=1;
// //     }
// // }
// fn main() {
//     for number in (1..6).rev() {
//         println!("{number}");
//     }
//     println!("LIFTOFF!!!");
// }


// exercise 3

// fn main(){
//     for num in 1..100 {
//         if num % 3 == 0 && num % 5 == 0 {
//             println!("FizzBuzz");
//         }
//         else if num % 3 ==0 {
//             println!("Fizz");
//         }
//         else if num % 5 ==0 {
//             println!("Buzz");
//         }
//         else {
//             println!("{}",num);
//         }
//     }
// }


// exercise 4

// fn main(){
//     'Number: for i in [1,2,3]{
//         for j in ['a','b','c']{
//             println!("{} {}",i,j);
//             if i==2 && j=='b'{
//                 break 'Number;
//             }
//         }
//     }
// }


// exercise 5

// use std::io;

// fn main() {
//     loop {
//         let mut input = String::new();

//         println!("Enter a number:");

//         io::stdin()
//             .read_line(&mut input)
//             .expect("Failed to read input");

//         let number: i32 = input.trim().parse()
//             .expect("Please enter a valid number");

//         if number == 0 {
//             println!("Good Bye!");
//             break;
//         }

//         if number % 2 == 0 {
//             println!("{} is even", number);
//         } else {
//             println!("{} is odd", number);
//         }
//     }
// }


// exercise 6

fn main(){
    let mut count = 0;
    for i in 1..20{
        if i%2==0{
            if count>2 && count<5{
                println!("{}",i);
            }
            count+=1;
        }

    }
}
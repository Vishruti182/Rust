// Exercise 1
// fn main(){
//     let city=String::from ("Mumbai");
//     let location=&city;

//     println!("City: {}",city);
//     println!("Location: {}", location);
// }


// Exercie 2
// fn display_message(msg : &String){
//     println!("Message: {}",msg);
// }

// fn main(){
//     let msg=String::from("Rust is fun!");
//     display_message(&msg);

//     println!("Again: {}",msg);
// }

// Exercise 3
// fn create_name()->String{
//     String::from("Tejas")
// }

// fn main(){
//     let name;

//     name=create_name();

//     println!("Name: {}",name);
// }

// Exercise 4
fn main(){
    let x=10;
    let y=x;

    println!("x: {}, y: {}",x,y);

    let s1=String::from("Ownership");
    let s2=s1.clone();

    println!("s1: {},s2: {}",s1,s2);
}
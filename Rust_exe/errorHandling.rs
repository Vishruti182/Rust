// Exercise 1

use std::fs::File;
//use std::fs;

// fn main() {
//     let open_file = File::open("closure.rs");
//     let file = match open_file {
//         Ok(file) => file,
//         Err(error) => panic!("Problem opening the file: {:?}",error),
//     };
// }


//Exercise 2


// fn main() {
//     let open_file = File::open("closure.rs");
//     let file = match open_file {
//         Ok(file) => file,
//         Err(error) => {
//             File::create("Exercise2.txt");
//             panic!("Problem with opening file, {:?}",error)
//         },
//     };
// }


//Exercise 3

// fn main(){
//     let read_file=match fs::read_to_string("Exercise.txt"){
//         Ok(read_file)=>{
//             println!("File content: {:?}",read_file)
//         },
//         Err(error)=>{
//             File::create("Exercise3.txt");
//             panic!("Problem with opening file, {:?} so created new one",error)
//         }
//     };

// }

//Exercise 4
//use std::io;

// fn main()->io::Result<()>{
//     let read_file=match fs::read_to_string("Exercise.txt"){
//         Ok(read_file)=>{
//                 println!("File content: {:?}",read_file)
//         },
//         Err(error) if error.kind()==io::ErrorKind::NotFound =>{
//             File::create("Exercise4.txt")?;
//             panic!("File not found, {:?} so created new one", error)
//         }
//         Err(e)=>return Err(e),
//     };
//     Ok(())
// }


// Exercise 5

fn main(){
    let open_file=File::open("Exercise2.txt").unwrap_or_else(|err|{
        println!("File not accessible due to {}",err);
        File::create("Exercise5.txt").unwrap()
    });
}
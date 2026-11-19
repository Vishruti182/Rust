// Exercise 1

// fn smallest<T: PartialOrd>(a: T, b: T)->T{
//     if a<b{
//         a
//     }
//     else{
//         b
//     }
// }
// fn main(){
//     println!("Smallest is {}",smallest(5,10));
//     println!("Smallest is {}",smallest(5.0,1.0));
//     println!("Smallest is {}",smallest('v','d'));

// }


//Exercise 2

// fn API(Payload)->Result<Response<Data>,Error>{

// }
// fn main(){

// }


// Exercise 3

trait Trackable{}
struct Vehicle;

impl Trackable for Vehicle{}

fn process<T: Trackable>(value:T){
    println!("Marker trait");
}

fn main(){
    process(Vehicle);
}
//trait Printable {}

// struct User;
// struct Admin;

// impl Printable for Admin {}
// impl Printable for User {}

// fn process<T: Printable>(value:T){
//     println!("Admin");
// }

// fn main() {
//     process(User);  // OK
//     process(Admin); // Compile error
// }
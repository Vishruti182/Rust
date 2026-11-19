// static reference

// fn main(){
//     let s: &'static str="Hello, rust!";
//     println!("{}",s);
// }


// fn message() -> &'static str {
//     "Welcome!"
// }
// fn main(){
//     println!("{}",message());
// }

// fn message() -> &'static str {
//     let name = String:from("Alice");
//     &name
// }
// fn main(){
//     println!("{}",message());
// }         ERROR - BCZ local variable cannot be returned as 'static

// static MESSAGE: &str = "Hello";

// fn main(){
//     println!("{}",MESSAGE);
// } 

// struct User {
//     name: String,
// }

// impl User {
//     fn get_name(&self) -> &str {
//         &self.name
//     }
// }

// // impl User {
// //     fn get_name<'a>(&'a self) -> &'a str {
// //         &self.name
// //     }
// // }

// fn main() {
//     let user = User {
//         name: String::from("Alice"),
//     };

//     let name = user.get_name();
//     println!("{}", name);
// }

// fn get_name(name: &str) -> &str {
//     name
// }

// fn main() {
//     let name = String::from("Alice");
//     let result = get_name(&name);

//     println!("{}", result);
// }




// use std::rc::Rc;

// fn main(){
//     let a=Rc::new(String::from("hello rust"));
//     let b=Rc::clone(&a);
//     println!("a = {a}");
//     println!("b = {b}");
//     println!("Owners = {}",Rc::strong_count(&a));
// }

// use std::sync::Arc;
// use std::thread;

// fn main(){
//     let data=Arc::new(String::from("Hello"));
//     let data2=Arc::clone(&data);
//     let handle=thread::spawn(move||{println!("{}",data2);});
//     println!("{}",data);
//     handle.join().unwrap();
// }

// await is executor of funtion


// use tokio::{
//     runtime::Runtime,
//     time::{sleep, Duration},
// };

// async fn do_something() {
//     println!("Doing something..");

//     sleep(Duration::from_secs(2)).await;

//     println!("Done!");
// }

// fn main() {
//     let rt = Runtime::new().unwrap();
//     rt.block_on(do_something());
// }


// Actix web

// use actix_web::{get, App, HttpServer, Responder};

// #[get("/")]
// async fn home() -> impl Responder {
//     "Hello from Actix Web!"
// }

// #[get("/hello/{name}")]
// async fn greet(
//     name: actix_web::web::Path<String>,
// ) -> impl Responder {
//     format!("Hello, {}!", name)
// }

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| {
//         App::new()
//             .service(home)
//             .service(greet)
//     })
//     .bind(("127.0.0.1", 8080))?
//     .run()
//     .await
// }


// use tokio::time::{sleep, Duration, Instant};

// async fn function1() -> String {
//     sleep(Duration::from_secs(2)).await;
//     "Function 1 finished after 2 seconds".to_string()
// }

// async fn function2() -> String {
//     sleep(Duration::from_secs(3)).await;
//     "Function 2 finished after 3 seconds".to_string()
// }

// #[tokio::main]
// async fn main() {
//     let start = Instant::now();

//     let (result1, result2) = tokio::join!(
//         function1(),
//         function2()
//     );

//     println!("{result1}");
//     println!("{result2}");
//     println!("Total time: {:.2?}", start.elapsed());
// }



// use std::thread;
// use std::time::{Duration, Instant};

// fn function1() -> String {
//     thread::sleep(Duration::from_secs(2));
//     "Function 1 finished after 2 seconds".to_string()
// }

// fn function2() -> String {
//     thread::sleep(Duration::from_secs(3));
//     "Function 2 finished after 3 seconds".to_string()
// }

// fn main() {
//     let start = Instant::now();

//     let handle1 = thread::spawn(function1);
//     let handle2 = thread::spawn(function2);

//     let result1 = handle1.join().unwrap();
//     let result2 = handle2.join().unwrap();

//     println!("{result1}");
//     println!("{result2}");
//     println!("Total time: {:.2?}", start.elapsed());
// }

// fn main(){
//     println!("{:.2}", 3.14159);
// }


trait Hello{
    fn hello();
}
struct Person;
fn main(){
    Person::hello();
}
impl Hello for Person{
    fn hello(){
        println!("Hello, I'm Person!");
    }
}
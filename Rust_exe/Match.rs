// Exercise 1
// enum TrafficLight{
//     Red,
//     Yellow,
//     Green
// }
// fn light_duration(light:TrafficLight)->u8{
//     match light{
//         TrafficLight::Red=>{
//             println!("Red: 30");
//             30
//         }
//         TrafficLight::Yellow=>{
//             println!("Yellow: 5");
//             5
//         }
//         TrafficLight::Green=>{
//             println!("Green: 45");
//             45
//         }
//     }
// }
// fn main(){
//     let light_red=TrafficLight::Red;
//     let light_yellow=TrafficLight::Yellow;
//     let light_green=TrafficLight::Green;
//     println!("{}",light_duration(light_red));
//     println!("{}",light_duration(light_yellow));
//     println!("{}",light_duration(light_green));
// }



//Exercise 2

// fn file_type(extension: &str)->&'static str{
//     match extension{
//         "txt"=>"Text",
//         "rs"=>"rs",
//         "jpg"=>"Image",
//         _=>"Unknown"
//     }
// }
// fn main(){
//     println!("{}",file_type("rs"));
//     println!("{}",file_type("txt"));
//     println!("{}",file_type("pdf"));
//     println!("{}",file_type("image"));
//     println!("{}",file_type("jpg"));
// }



// Exercise 3

// enum PaymentMethod{
//     CreditCard(String, u8, u16),
//     PayPal(String),
//     Crypto(String),
//     Cash,
// }
// fn process_payment(method: PaymentMethod, amount:f64)->String{
//     match method{
//         PaymentMethod::CreditCard(num,exp_month,exp_year)=>
//             format!("Credit card number: {}, expire month: {},expiry year: {}, amount: {}", num,exp_month,exp_year,amount),
//         PaymentMethod::PayPal(email)=>
//             format!("PayPal Email id: {}, amount: {}",email,amount),
//         PaymentMethod::Crypto(addr)=>
//             format!("Crypto Payment address: {}, amount: {}", addr,amount),
//         PaymentMethod::Cash=>
//             format!("Payment done by cash, amount {}",amount),
//     }
// }
// fn main(){
//     println!("{}",process_payment(PaymentMethod::Cash,500.0));
//     println!("{}",process_payment(PaymentMethod::CreditCard("112521".to_string(),11,2026),1000.0));
//     println!("{}",process_payment(PaymentMethod::PayPal("abc@mail.com".to_string()),400.0));
//     println!("{}",process_payment(PaymentMethod::Crypto("vd42feee".to_string()),9870.0));
// }



// Exercise 4

// enum Direction{
//     Up,
//     Down,
//     Left,
//     Right,
// }
// struct Position{
//     x:i32,
//     y:i32,
// }
// fn move_robot(current: Position, dir: Direction) -> Position {
//     match dir {
//         Direction::Up => Position { x: current.x, y: current.y + 1 },
//         Direction::Down => Position { x: current.x, y: current.y - 1 },
//         Direction::Left => Position { x: current.x - 1, y: current.y },
//         Direction::Right => Position { x: current.x + 1, y: current.y },
//     }
// }

// fn main() {
//     let position = Position { x: 0, y: 0 };
//     let position1 = move_robot(position, Direction::Up);
//     println!("Position: {}, {}", position1.x, position1.y);
//     let position2 = move_robot(position1, Direction::Right);
//     println!("Position: {}, {}", position2.x, position2.y);
//     let position3 = move_robot(position2, Direction::Down);
//     println!("Position: {}, {}", position3.x, position3.y);
//     let position4 = move_robot(position3, Direction::Left);
//     println!("Position: {}, {}", position4.x, position4.y);
// }



// Exercise 5

enum UserRole{
    Guest,
    User,
    Moderator,
    Admin,
}
fn get_permissions(role:UserRole)->Vec<&'static str>{
    match role{
        UserRole::Guest=>vec!["Welcome guest!","Enjoy."],
        UserRole::User=>vec!["Welcome User!","Enjoy."],
        UserRole::Moderator=>vec!["Welcome Moderator!","Mange."],
        UserRole::Admin=>vec!["Welcome Admin!","Manage."],

        
    }
}
fn main(){
    println!("Permission: {:?}",get_permissions(UserRole::Guest));
    println!("Permission: {:?}",get_permissions(UserRole::User));
    println!("Permission: {:?}",get_permissions(UserRole::Moderator));
    println!("Permission: {:?}",get_permissions(UserRole::Admin));
}

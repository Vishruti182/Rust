// Exercise 1
// fn safe_divide(a: f64, b: f64) -> Option<f64> {
//     if b != 0.0{
//         Some(a/b)
//     }
//     else{
//         None
//     }
// }
// fn main(){
//     println!("{:?}",safe_divide(10.0,5.0));
//     println!("{:?}",safe_divide(111.0,111.0));
//     println!("{:?}",safe_divide(10.0,00.0));
// }



// Exercise 2

// fn first_char(s: &str)->Option<char>{
//     if s.is_empty(){
//         None
//     }
//     else{
//         let character=s.chars().nth(0);
//         Some(character?)
//     }
// }
// fn main(){
//     println!("{:?}",first_char("Hello"));
//     println!("{:?}",first_char(""));
// }


// Exercise 3
// #[derive(Debug,Clone)]
// struct User{
//     id: u32,
//     name: String,
// }

// fn find_user(users: Vec<User>, target_id: u32) ->Option<User>{
//     for i in users{
//         if target_id==i.id{
//             return Some(i);
//         }
//     }
//     None
// }
// fn main(){
//     let users=vec![User{id:1, name:"ALice".to_string()},
//                 User{id:2, name:"Bob".to_string()}
//                 ];
//     println!("{:?}",find_user(users.clone(),1));
//     println!("{:?}",find_user(users.clone(),2));
//     println!("{:?}",find_user(users.clone(),3));

// }


// Exercise 4

fn parse_config_value(value: &str)->Option<ConfigValue>{
    #[derive(Debug, PartialEq)]
    enum ConfigValue{
        Int(i32),
        Float(f64),
        Bool(bool),
        String(String)
    }
}
fn main(){
    println!(parse_config_value("42"));
    println!(parse_config_value("true"));
    println!(parse_config_value("3.14"));
    println!(parse_config_value("hello"));
    println!(parse_config_value("not_a_number"));
}
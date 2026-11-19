#[tokio::main]
async fn immediate_fututre()->i32{
    42
}
fn main(){
    let a=immediate_fututre().await;
    println!("{}",a);
}
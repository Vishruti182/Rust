// Exercise 1
//#[tokio::main]
// async fn main(){
//     let a=immediate_fututre().await;
//     println!("{}",a);
// }

// async fn immediate_fututre()->i32{
//     42
// }


// Exercise 2

// #[tokio::main]
// async fn main(){
//     let f1=std::future::ready(5);
//     let f2=std::future::ready(4);
//     let res = sum_futures(f1,f2).await;
//     println!("Sum: {}",res);
// }
// async fn sum_futures(fut1: impl Future<Output = i32>, 
//     fut2: impl Future<Output = i32>)->i32{
//         let (val1,val2)=tokio::join!(fut1,fut2);
//         val1+val2
// }

// Exercise 3
// use std::time::Duration;
// #[tokio::main]
// async fn main(){
//     let res=slow_add(5,4,2000).await;
//     println!("Slow addition: {}",res);
// }
// async fn slow_add(a: i32, b:i32, delay_ms: u64) -> i32{
//     tokio::time::sleep(Duration::from_millis(delay_ms)).await;
//     a+b
// }


// Exercise 4
// use std::future::Future;

// async fn process_number(fut: impl Future<Output=i32)->String{
//     fut.then(|n| async move {n*2}).map(|n| n.to_string()).await
// }
// fn main(){
//     ptintln!("{}",process_number(11));
// }p

// use std::future::Future;

// async fn process_number(fut: impl Future<Output=i32)->String{
//     let n=fut.await;
//     (n*2).to_string()
// }
// fn main(){
//     let res=futures::executor::block_on(process_number(async { 11 }));
//     ptintln!("{}",res);
// }   // tokio::main can be used instead of futures::executor::block_on


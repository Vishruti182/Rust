// use std::task::{Context, Poll};

// struct MyFuture{
//     count: u32,
// }

// impl Future for MyFuture{
//     type Output=();

//     fn poll(self: Pin<&mut Self>, cx: &mut Context<' >)->Poll<()>{
//         if self.count>=10{
//             Poll::Ready(())
//         }
//         else{
//             cx.waker().wake_by_ref();
//             Poll::Pending
//         }
//     }
// }

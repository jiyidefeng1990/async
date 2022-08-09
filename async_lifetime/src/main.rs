use futures::executor;
use std::future::Future;

async fn foo(x:&u8)->u8 {
    *x
}

fn foo_expand<'a>(x:&'a u8)-> impl Future<Output=u8> + 'a {
    async{
        *x
    }
}

// fn bad() -> impl Future<Output=u8>{
//     let x = 5;
//     foo_expand(&x)
// }

fn good() -> impl Future<Output=u8>{
    async {
        let x = 5;
        foo_expand(&x).await
    }
    
}

fn main() {
    let x = 5;
    let f = foo(&x);
    let t = foo_expand(&x);
    executor::block_on(f);
    executor::block_on(t);
}

use futures::executor::block_on;


fn main() {
    let future = do_something();
    block_on(future);
}

async fn do_something() {
    println!("gogogo!")
}

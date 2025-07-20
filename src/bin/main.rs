use std::time::Duration;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    tokio::spawn(p1());
    p2().await;
    // loop {
    //     println!("mainnnnn");
    //     tokio::time::sleep(Duration::from_secs(1)).await;
    // }
}

async fn p1() {
    println!("p1111");
    tokio::time::sleep(Duration::from_secs(2)).await;
    println!("p1111 again");
}

async fn p2() {
    println!("p2222");
    tokio::time::sleep(Duration::from_secs(3)).await;
    println!("p2222 again");
}
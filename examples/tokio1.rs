use std::thread;
use std::time::Duration;
use tokio::fs;
use tokio::runtime::Builder;
use tokio::time::sleep;

fn main() {
    // let a = 10;
    // let b = 20;
    //  println!("{} {} {}" ,a+b, a,b);
    let handle = thread::spawn(|| {
        let rt = Builder::new_current_thread().enable_all().build().unwrap();
        rt.spawn(async {
            println!("Future 1");
            let content = fs::read_to_string("Cargo.toml").await.unwrap();
            println!("content length {}", content.len());
        });
        rt.spawn(async {
            println!("Future 2");
            let ret = expensive_blocking_task("Future 2".to_string());
            println!("ret Hash is {}", ret);
        });
        // rt.block_on(
        //     async {
        //         println!("hello world");
        //     }
        // );
        rt.block_on(async {
            println!("Future 3");
            sleep(Duration::from_millis(900)).await;
            println!("block_on执行结束");
        });
    });
    handle.join().unwrap();
    println!("主线程");
}
fn expensive_blocking_task(s: String) -> String {
    thread::sleep(Duration::from_millis(800));
    blake3::hash(s.as_bytes()).to_string()
}

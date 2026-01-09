use std::{
    sync::{Arc, LazyLock},
    time::SystemTime,
};

use arc_swap::ArcSwap;

static HOBBIES: LazyLock<ArcSwap<Vec<String>>> =
    LazyLock::new(|| ArcSwap::new(Arc::new(vec!["吃饭".into(), "睡觉".into()])));

#[tokio::main]
async fn main() {
    // 读取
    let hobbies = HOBBIES.load();
    println!("{:?}", *hobbies);

    let mut new_hobbies = vec![];
    new_hobbies.extend_from_slice(hobbies.as_slice());
    new_hobbies.push("打豆豆".into());
    // 写入
    HOBBIES.store(Arc::new(new_hobbies));

    let hobbies = HOBBIES.load();
    println!("{:?}", *hobbies);

    // 异步读写
    let async_handler_write = tokio::spawn(async_write());
    let async_handler_read = tokio::spawn(async_read());
    let _ = tokio::join!(async_handler_write, async_handler_read);
}

async fn async_read() {
    for hobby in HOBBIES.load().iter() {
        println!("【async_read】{}", hobby);
    }
}

async fn async_write() {
    // 当前时间戳
    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();
    let h = format!("打豆豆-{}", now.as_secs());
    HOBBIES.store(Arc::new(vec![h]));
}

use std::{
    sync::{LazyLock, Mutex},
    time::SystemTime,
};

static HOBBIES: LazyLock<Mutex<Vec<String>>> =
    LazyLock::new(|| Mutex::new(vec!["吃饭".into(), "睡觉".into()]));
#[tokio::main]
async fn main() {
    // 写入
    {
        HOBBIES.lock().unwrap().push("打豆豆".into());
    }

    // 读取
    {
        for hobby in HOBBIES.lock().unwrap().iter() {
            println!("{}", hobby);
        }
    }

    // 异步读写
    let async_handler_write = tokio::spawn(async_write());
    let async_handler_read = tokio::spawn(async_read());
    let _ = tokio::join!(async_handler_write, async_handler_read);
}

async fn async_read() {
    for hobby in HOBBIES.lock().unwrap().as_slice() {
        println!("【async_read】{}", hobby);
    }
}

async fn async_write() {
    // 当前时间戳
    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();

    let h = format!("打豆豆-{}", now.as_secs());
    HOBBIES.lock().unwrap().push(h);
}

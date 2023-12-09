use std::sync::{Arc, Mutex};
use std::thread;

// Arc<Mutex<T>> 可以安全地在多个线程间共享可变数据
fn arc_mutex_is_send_sync() {
  let a = Arc::new(Mutex::new(1)); // 初始化一个 Mutex 包裹的值
  let b = a.clone(); // 克隆 Arc，增加引用计数
  let c = a.clone(); // 再次克隆 Arc

  let handle = thread::spawn(move || {
    let mut g = c.lock().unwrap(); // 锁定 Mutex 并获取其内部的值
    *g += 1; // 修改值
  });

  {
    let mut g = b.lock().unwrap(); // 在主线程中也锁定 Mutex 并修改值
    *g += 1;
  }

  handle.join().unwrap(); // 等待新线程完成
  println!("a: {:?}", a); // 打印修改后的值
}

fn main() {
  arc_mutex_is_send_sync(); // 调用函数
}

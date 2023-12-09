use std::alloc::{GlobalAlloc, Layout, System};

struct MyAllocator;

unsafe impl GlobalAlloc for MyAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let data = System.alloc(layout);
        eprintln!("ALLOC: {:p}, size {}", data, layout.size());
        data
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        System.dealloc(ptr, layout);
        eprintln!("FREE: {:p}, size {}", ptr, layout.size());
    }
}

#[global_allocator]
static GLOBAL: MyAllocator = MyAllocator;

#[allow(dead_code)]
struct Matrix {
    // 假设不限制其实际大小 50乘50 可以是 dbg! 的打印信息会更清晰
    data: [u8; 50],
}

impl Default for Matrix {
    fn default() -> Self {
        Self { data: [0; 50] }
    }
}

fn main() {
    // 在这里我们不注重矩阵内存分配
    let data = Box::new(Matrix::default());

    // 输出堆中一个 1024 个小内存块地址, 用 eprintln! 输出标准
    eprintln!(
        "!!! allocated memory: {:p}, len: {}",
        &*data,
        std::mem::size_of::<Matrix>()
    );

    // data 在这里被 drop, 因此它里面的空间会被 FREE
    // 这里不用显式的写出来释放
}

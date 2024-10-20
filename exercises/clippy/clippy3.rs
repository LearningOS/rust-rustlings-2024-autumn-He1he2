// clippy3.rs
// 
// Here's a couple more easy Clippy fixes, so you can see its utility.
//
// Execute `rustlings hint clippy3` or use the `hint` watch subcommand for a hint.



#[allow(unused_variables)]
fn main() {
    let my_option: Option<()> = None;
    if my_option.is_none() {
        println!("my_option is None");
    }

    let my_arr = &[
        -1, -2, -3, // 添加逗号
        -4, -5, -6,
    ];
    println!("My array! Here it is: {:?}", my_arr);

    let my_empty_vec: Vec<i32> = Vec::new(); // 创建一个空向量
    println!("This Vec is empty, see? {:?}", my_empty_vec);

    let mut value_a = 45;
    let mut value_b = 66;
    // Let's swap these two!
    let temp = value_a; // 使用临时变量进行交换
    value_a = value_b;
    value_b = temp;
    println!("value a: {}; value b: {}", value_a, value_b);
}

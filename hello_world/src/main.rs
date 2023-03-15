fn greet_world() {
    let southern_germany = "Grüß Gott!";
    let chinese = "世界，你好";
    let english = "World, hello";
    let regions = [southern_germany, chinese, english];
    for region in regions.iter() {
        println!("{}", &region);
    }
    //只有在debug模式下会运行
    if cfg!(debug_assertions) {
        println!("debug: it is debug mode");
    }
}

//
fn check() -> ! {
    loop {}
}

fn main() {
    greet_world();

    let _x = 5;
    let _y = 10; //_ 忽略为使用
    let heart_eyed_cat = '😻';
    println!("{},{}", _x, heart_eyed_cat);

    println!(
        "{},{},{},{}",
        (0.1_f64 + 0.2_f64).abs(),
        0.1 + 0.2,
        0.3_f64,
        (0.1_f64 + 0.2 - 0.3).abs() <= 0.000001 //float 是有精度问题的
    );

    check();
}

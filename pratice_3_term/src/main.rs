use std::io::{stdout};
use crossterm::{execute, style::{Color, Print, ResetColor, SetForegroundColor}};
use crossterm::style::SetBackgroundColor;

fn main() {
    // 设置文本颜色为红色
    let red_font_color = SetForegroundColor(Color::Red);
    // 设置文本颜色为绿色
    let green_font_color = SetForegroundColor(Color::Green);

    let green_background = SetBackgroundColor(Color::Green);
    let red_background = SetBackgroundColor(Color::Red);

    // 执行终端操作
    execute!(
        stdout(),
        // 打印红色文本
        red_font_color, green_background, Print("Hello"),
        // 打印空格
        ResetColor, Print(" "),
        // 打印绿色文本
        green_font_color, red_background, Print("World!"),
        // 重置文本颜色
        ResetColor
    ).unwrap();
    test_fill_zero();
}

fn test_fill_zero() {
    println!("====== test_fill_zero ======");
    let num1 = 43i32;
    let num2 = 1234i32;
    let num1_string = format!("{:0>4}", num1);
    let num2_string = format!("{:0>4}", num2);
    println!("num1_string = {}", num1_string);
    println!("num2_string = {}", num2_string);
}

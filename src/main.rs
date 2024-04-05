use crate::{area::Area, signal::Signal};

mod area;
mod signal;
mod sort;
mod sum;

fn main() {
    let mut num = vec![1, 5, 99, 80, 123, 23, 0];
    println!("排序前:{:?}", num);
    sort::number_sort(&mut num);
    println!("排序后:{:?}", num);

    println!("交通信号灯");
    let red_signal = signal::SignalLights::RED(10);
    let value = red_signal.signal_time();
    println!("red time {}", value);

    println!("整数集合求和");
    let array = vec![1, 2, 3, 4, 5, 6];
    let value = sum::sum(&array);
    match value {
        Some(nu) => println!("value:{}", nu),
        None => println!("none"),
    }

    println!("图形面积的函数");
    let square = area::Square { width: 10 };
    let rectangle = area::Rectangle {
        width: 10,
        height: 8,
    };
    area::graph_area(&square);
    area::graph_area(&rectangle);
}

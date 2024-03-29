mod sort;

fn main() {
    let mut num = vec![1, 5, 99, 80, 123, 23, 0];
    println!("排序前:{:?}", num);
    sort::number_sort(&mut num);
    println!("排序后:{:?}", num);
}

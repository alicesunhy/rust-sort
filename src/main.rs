use crate::sort::Person;

mod sort;

fn main() {
    let mut num = vec![-1, 5, 99, 80, 123, 23, 0];
    println!("排序前:{:?}", num);
    sort::number_sort(&mut num);
    println!("排序后:{:?}", num);

    let person1 = Person {
        name: String::from("a"),
        age: 1,
    };

    let person2 = Person {
        name: String::from("b"),
        age: 2,
    };

    let person3 = Person {
        name: String::from("c"),
        age: 3,
    };

    let person4 = Person {
        name: String::from("d"),
        age: 4,
    };

    let mut num = vec![person1, person4, person2, person3];
    println!("排序前： {:?}", num);
    num.sort();
    println!("排序前： {:?}", num);
}

use std::cmp::Ordering;

enum ComparisonOrder {
    Regular,
    Reverse,
}

fn my_f<T: std::cmp::PartialOrd>(value: T, comparison_order: ComparisonOrder, target: T) -> bool {
    match comparison_order {
        ComparisonOrder::Regular => value.partial_cmp(&target) == Some(Ordering::Greater),
        ComparisonOrder::Reverse => value.partial_cmp(&target) == Some(Ordering::Less),
    }
}

fn main() {
    println!("{}", my_f(30, ComparisonOrder::Regular, 20)); // true
    println!("{}", my_f(10, ComparisonOrder::Regular, 20)); // false
    println!("{}", my_f(30, ComparisonOrder::Reverse, 20)); // false
    println!("{}", my_f(10, ComparisonOrder::Reverse, 20)); // true
}

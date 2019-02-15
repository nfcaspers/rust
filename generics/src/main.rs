#![allow(dead_code)]

//only works with slices etc. of type i32
fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0]; //set first element of list as largest
    println!("largest is {:?}", largest);
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

//only works with slices etc. of type char
fn largest_char(list: &[char]) -> char {
    let mut largest = list[0]; //set first element of list as largest
    println!("largest is {:?}", largest);
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

//works with everything that implements Partial0rd and Copy
fn largest_generic<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0]; //set first element of list as largest
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

//works with everything that implements Partial0rd and Clone
fn largest_clone<T: PartialOrd + Clone>(list: &[T]) -> T {
    let mut largest = list[0].clone(); //set clone of first element of list as largest
    for item in list.iter() {
        if *item > largest {
            largest = item.clone();
        }
    }
    largest
}

//does not need copy or clone, because it only uses references
fn largest_return_ref<T: PartialOrd>(list: &[T]) -> &T {
    //return ref to slice val
    let mut largest = &list[0]; //set ref to first element of list as largest
    for item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_generic(&number_list);
    println!("The largest number is {}", result);

    let result = largest_generic(&char_list);
    println!("The largest char is {}", result);

    let result = largest_clone(&number_list);
    println!("The largest number is {}", result);

    let result = largest_clone(&char_list);
    println!("The largest char is {}", result);

    let result = largest_return_ref(&number_list);
    println!("The largest number is {}", result);

    let result = largest_return_ref(&char_list);
    println!("The largest char is {}", result);
}

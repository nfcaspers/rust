#![allow(dead_code)]
fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0]; //list already existed and had values
    println!("largest is {:?}", largest);
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest 
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0]; //list already existed and had values
    println!("largest is {:?}", largest);
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest 
}

fn largest_generic<T: PartialOrd + Copy >(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
} 

fn largest_clone<T: PartialOrd + Clone>(list: &[T]) -> T {
    let mut largest = list[0].clone();
    for item in list.iter() {
        if *item > largest {
            largest = item.clone();
        }
    }
    largest 
}

fn largest_return_ref<T: PartialOrd>(list: &[T]) -> &T { //return ref to slice val
    let mut largest = &list[0];
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

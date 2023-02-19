fn fixed_array() {
    println!("\nfixed arrays");

    let arr = [5, 4, 3, 2, 1];

    let mapped_arr = arr.map(|num: i32| num + 2);

    for i in 0..mapped_arr.len() {
        println!("index: {}, value: {}", i, mapped_arr[i]);
    }
}

fn dynamic_array() {
    println!("\ndynamic arrays");

    let mut vec: Vec<i32> = (1..4).collect();

    let mut sub_vec = vec![4, 5, 6];

    vec.pop();

    vec.append(&mut sub_vec);

    vec.push(7);

    for num in vec.iter() {
        println!("num: {}", num);
    }
}

pub fn arrays() {
    println!("\narrays");

    fixed_array();
    dynamic_array();
}

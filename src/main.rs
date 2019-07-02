use std::collections::HashMap;

fn main() {
    
    println!("{:?} ", avg_med_mod(vec![11, 12, 13, 10, 1, 2, 3, 4, 5, 6, 7, 8, 9]));
}

fn avg_med_mod(mut vec :Vec<i32>) -> (f32 , i32, i32) {
    let mut avg = 0.0;
    let mut med :i32;
    let mut map = HashMap::new();
    for element in vec.iter() {
        let count = map.entry(element).or_insert(0);
        *count += 1;
    }
    vec.sort();
    med = vec[vec.len() / 2];
    println!("{:?}", vec);
    for element in vec.iter() {
        avg = avg + *element as f32;
    }
    (avg / vec.len() as f32, med, 22)
}
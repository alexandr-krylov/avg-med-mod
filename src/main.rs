use std::collections::HashMap;

fn main() {
    
    println!("{:?} ", avg_med_mod(vec![
        11, 12, 13, 10,  1,  2,  3, 4,  5,  6,
         7,  8,  9, 12, 22, 11, 12, 7, 11, 22
         ]));
}

fn avg_med_mod(mut vec :Vec<i32>) -> (f32 , i32, i32) {
    let mut avg = 0.0;
    let med :i32;
    let mut moda = 0;
    let mut map = HashMap::new();
    for element in vec.iter() {
        let count = map.entry(element).or_insert(0);
        *count += 1;
    }
    let mut max_element = 0;
    for map_element in map.iter() {
        if map_element.1 > &max_element {
            max_element = *map_element.1;
            moda = **map_element.0;
        }
    }
    println!("{:?}", vec);
    vec.sort();
    med = vec[vec.len() / 2];
    for element in vec.iter() {
        avg = avg + *element as f32;
    }
    (avg / vec.len() as f32, med, moda)
}
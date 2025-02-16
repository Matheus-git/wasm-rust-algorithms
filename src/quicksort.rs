use rand::Rng;

fn get_random_index<T>(vec: &Vec<T>) -> usize {
    let mut rng = rand::thread_rng();
    rng.gen_range(0..vec.len())
}

pub fn calc_quicksort(mut list: Vec<i32>) -> Vec<i32> {
    if list.len() <= 1 {
        return list;
    }

    let pivot: i32 = list.remove(get_random_index(&list));

    let mut smaller: Vec<i32> = Vec::new();
    for val in list.iter() {
        if pivot > *val {
            smaller.push(*val);
        }
    }

    let mut greater: Vec<i32> = Vec::new();
    for val in list.iter() {
        if pivot < *val {
            greater.push(*val);
        }
    }

    let mut result: Vec<i32> = calc_quicksort(smaller);
    result.push(pivot);
    result.extend(calc_quicksort(greater));

    result
}

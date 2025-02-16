use rand::Rng;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn get_random_index<T>(vec: &Vec<T>) -> usize {
    let mut rng = rand::thread_rng();
    rng.gen_range(0..vec.len())
}

fn quicksort(mut list: Vec<i32>) -> Vec<i32> {
    if list.len() <= 1 {
        return list;
    }

    let pivo: i32 = list.remove(get_random_index(&list));

    let mut menores: Vec<i32> = Vec::new();
    for val in list.iter() {
        if pivo > *val {
            menores.push(*val)
        }
    }

    let mut maiores: Vec<i32> = Vec::new();
    for val in list.iter() {
        if pivo < *val {
            maiores.push(*val)
        }
    }

    let mut result = quicksort(menores);
    result.push(pivo);
    result.extend(quicksort(maiores));

    result
}

fn quicksort_benchmark(c: &mut Criterion) {
    c.bench_function("quicksort 10k itens aleatórios", |b| {
        b.iter(|| {
            let mut rng = rand::thread_rng();
            let random_vec: Vec<i32> = (0..10000).map(|_| rng.gen_range(1..101)).collect();
            black_box(quicksort(random_vec));
        })
    });
}

criterion_group!(benches, quicksort_benchmark);
criterion_main!(benches);

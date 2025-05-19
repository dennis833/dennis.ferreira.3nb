use rand::Rng;

fn main() {
    // gera os 20 vetores com números aleatorios entre 1 e 100
    let mut rng = rand::thread_rng();
    let mut vec: Vec<i32> = (0..20).map(|_| rng.gen_range(1..=100)).collect();

    // faz um print do vetor original
    println!("Vetor original: {:?}", vec);

    // ordena o vetor
    vec.sort();

    // faz um print do vetor ordenado!!!
    println!("Vetor ordenado: {:?}", vec);
}

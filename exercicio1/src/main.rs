use std::collections::VecDeque;

fn main() {
    let mut fila: VecDeque<i32> = VecDeque::new();
    println!("implementando fila com VecDeque");
    fila.push_back(12);
    fila.push_back(20);
    fila.push_back(22);
    fila.push_back(44);
    fila.push_back(28);
    fila.push_back(06);
    println!("Tamanho atual a fila: {}", fila.len());

    if let Some(elemento) = fila.front() {
        println!("o que está na frente da fila: {}", elemento);
    }

    println!("removendo da lista");
    while let Some(elemento) = fila.pop_front() {
        println!("Removido: {}", elemento);
    }

    println!("a está vazia? {}", fila.is_empty());

}
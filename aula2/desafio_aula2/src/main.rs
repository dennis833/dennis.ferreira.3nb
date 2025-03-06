use std::collections::VecDeque;

struct Fila {
    clientes: VecDeque<String>,
}

impl Fila {
    // Cria uma nova fila vazia
    fn new() -> Fila {
        Fila {
            clientes: VecDeque::new(),
        }
    }

    // Enfileira um cliente (adiciona ao final da fila)
    fn enfileirar(&mut self, cliente: String) {
        self.clientes.push_back(cliente);
    }

    // Desenfileira um cliente (remove o primeiro da fila)
    fn desenfileirar(&mut self) -> Option<String> {
        if self.esta_vazia() {
            println!("A fila está vazia. Não há clientes para atender.");
            None
        } else {
            self.clientes.pop_front()
        }
    }

    // Verifica se a fila está vazia
    fn esta_vazia(&self) -> bool {
        self.clientes.is_empty()
    }

    // Retorna o primeiro cliente da fila sem removê-lo
    fn primeiro(&self) -> Option<&String> {
        self.clientes.front()
    }

    // Exibe todos os clientes na fila
    fn exibir_fila(&self) {
        if self.esta_vazia() {
            println!("A fila está vazia.");
        } else {
            println!("Clientes na fila: {:?}", self.clientes);
        }
    }
}

fn main() {
    // Criando uma nova fila
    let mut fila = Fila::new();

    // Enfileirando clientes
    fila.enfileirar("Cliente 1".to_string());
    fila.enfileirar("Cliente 2".to_string());
    fila.enfileirar("Cliente 3".to_string());

    // Exibindo a fila
    fila.exibir_fila();

    // Desenfileirando (atendendo) o primeiro cliente
    println!("Atendendo: {}", fila.desenfileirar().unwrap());

    // Exibindo a fila após o atendimento
    fila.exibir_fila();

    // Verificando o próximo cliente
    if let Some(primeiro) = fila.primeiro() {
        println!("Próximo cliente: {}", primeiro);
    }

    // Tentando desenfileirar quando a fila está vazia
    fila.desenfileirar();
    fila.desenfileirar();
    fila.desenfileirar(); // A fila já está vazia
}

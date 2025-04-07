// Estrutura do nó da árvore
struct Node {
    value: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(value: i32) -> Self {
        Node {
            value,
            left: None,
            right: None,
        }
    }
}

// Estrutura da árvore binária de busca (BST)
pub struct BST {
    root: Option<Box<Node>>,
}

impl BST {
    // Criar uma nova árvore vazia
    pub fn new() -> Self {
        BST { root: None }
    }

    // Verificar se a árvore está vazia
    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    // Inserir um valor na árvore
    pub fn insert(&mut self, value: i32) {
        let new_node = Box::new(Node::new(value));

        if let Some(ref mut root) = self.root {
            Self::insert_node(root, new_node);
        } else {
            self.root = Some(new_node);
        }
    }

    // Método auxiliar para inserir um nó recursivamente
    fn insert_node(node: &mut Box<Node>, new_node: Box<Node>) {
        if new_node.value < node.value {
            if let Some(ref mut left) = node.left {
                Self::insert_node(left, new_node);
            } else {
                node.left = Some(new_node);
            }
        } else {
            if let Some(ref mut right) = node.right {
                Self::insert_node(right, new_node);
            } else {
                node.right = Some(new_node);
            }
        }
    }

    // Buscar um valor na árvore
    pub fn search(&self, value: i32) -> bool {
        Self::search_node(&self.root, value)
    }

    // Método auxiliar para buscar um valor recursivamente
    fn search_node(node: &Option<Box<Node>>, value: i32) -> bool {
        match node {
            Some(n) => {
                if value == n.value {
                    true
                } else if value < n.value {
                    Self::search_node(&n.left, value)
                } else {
                    Self::search_node(&n.right, value)
                }
            }
            None => false,
        }
    }
}

#[cfg(test)]
mod bst_tests {
    // Importe sua implementação de BST aqui
    use super::BST;

    #[test]
    fn test_bst_new_and_empty() {
        // Teste 1: Criar uma nova árvore e verificar se está vazia
        let bst = BST::new();
        assert!(bst.is_empty());
    }

    #[test]
    fn test_bst_insert_and_search() {
        // Teste 2: Inserir elementos e verificar se estão na árvore
        let mut bst = BST::new();

        // Inserir alguns valores
        bst.insert(10);
        bst.insert(5);
        bst.insert(15);

        // Verificar se os valores inseridos estão na árvore
        assert!(bst.search(10));
        assert!(bst.search(5));
        assert!(bst.search(15));

        // Verificar que um valor não inserido não está na árvore
        assert!(!bst.search(20));

        // A árvore não deve mais estar vazia
        assert!(!bst.is_empty());
    }
}

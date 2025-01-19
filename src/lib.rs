use std::{
    collections::{BTreeMap as Map, BTreeSet as Set},
    fs::File,
    io::BufReader,
};

pub mod knapsack;
pub mod tsp;
pub mod utils;

/// Nós são identificados pelo tipo `u32`, que é um inteiro de 32 bits positivo.
///
/// Equivale a um typedef em C++.
pub type Node = u32;
/// Definimos pesos das arestas como sendo inteiros de 32bits positivos.
pub type Weight = u32;
/// Definimos nossas arestas como sendo uma tupla de dois nós e um peso.
pub type Edge = (Node, Node, Weight);

/// Um `trait` que define os métodos que todo grafo deve implementar.
///
/// `Trait`s são análogos a classes abstratas em C++, ou interfaces em outras linguagens
pub trait Graph {
    fn add_node(&mut self) -> Node;
    fn add_edge(&mut self, a: Node, b: Node, weight: Weight);
    fn edges(&self) -> Set<Edge>;
    fn node_count(&self) -> usize;

    //
    fn get_node_edges(&self, a: Node) -> Set<Edge>;

    fn get_edge_weight(&self, a: Node, b: Node) -> Option<Weight>;
}

/// Struct que representa um grafo implementado por meio de lista de adjacência.
///
/// A diretiva `derive` implementa traits (interfaces) automaticamente, sendo eles:
///     - Default: Permite a inicialização com valores padrões para todos os campos
///     - Debug: Mostra o tipo e seus campos de forma intuitiva para debug
///     - Clone: Permite criar cópias da struct.
#[derive(Default, Debug, Clone)]
pub struct GraphAdj {
    next_node: Node,
    node_edges: Map<Node, Vec<Edge>>,
}

impl Graph for GraphAdj {
    fn edges(&self) -> Set<Edge> {
        self.node_edges.values().flatten().copied().collect()
    }
    fn add_node(&mut self) -> Node {
        let node = self.next_node;
        self.node_edges.insert(node, vec![]);

        self.next_node += 1;
        node
    }
    fn node_count(&self) -> usize {
        self.node_edges.len()
    }
    fn add_edge(&mut self, a: Node, b: Node, weight: Weight) {
        for (a, b) in [(a, b), (b, a)] {
            let a_edges = self
                .node_edges
                .get_mut(&a)
                .unwrap_or_else(|| panic!("Tried to add edge to inexistent node {a}"));
            match a_edges.iter_mut().find(|e| e.1 == b) {
                Some(existing_edge) => existing_edge.2 = weight,
                None => a_edges.push((a, b, weight)),
            }
        }
    }

    fn get_node_edges(&self, a: Node) -> Set<Edge> {
        self.node_edges[&a].iter().copied().collect()
    }

    fn get_edge_weight(&self, a: Node, b: Node) -> Option<Weight> {
        self.node_edges[&a]
            .iter()
            .find(|(_, neighbour, _)| *neighbour == b)
            .map(|(_, _, w)| *w)
    }
}

/// Struct que representa um grafo implementado por matriz de adjacência.
#[derive(Default, Debug, Clone)]
pub struct GraphMat {
    node_count: usize,
    links: Vec<Weight>,
}

impl Graph for GraphMat {
    fn add_node(&mut self) -> Node {
        let new_node = self.node_count as Node;

        let new_node_count = self.node_count + 1;
        // Cria novo vetor cujo tamanho é `(node_count+1) ^ 2`
        let mut new_links = vec![0; new_node_count.pow(2)];

        // Caso haja nós no vetor, precisamos copiar as informações para o novo.
        if self.node_count > 0 {
            // Cria um iterador que agrupa `new_node_count` elementos por vez do novo vetor.
            // Ou seja, temos um vetor que representa uma linha do vetor a cada iteração.
            let new_lines = new_links.chunks_mut(new_node_count);
            // Cria um iterador que agroupa `node_count` elementos por vez do vetor antigo.
            let old_lines = self.links.chunks_mut(self.node_count);
            for (new_line, old_line) in new_lines.zip(old_lines) {
                // Limita as linhas do novo vetor para que tenham exatamente `node_count` elementos
                new_line[..self.node_count]
                    // Copia os pesos das arestas do antigo vetor para o novo.
                    .copy_from_slice(old_line);
            }
        }

        self.links = new_links;
        self.node_count += 1;

        new_node
    }
    fn node_count(&self) -> usize {
        self.node_count
    }
    fn get_edge_weight(&self, a: Node, b: Node) -> Option<Weight> {
        let idx = a as usize * self.node_count + b as usize;
        let w = *self.links.get(idx)?;
        if w == 0 {
            None
        } else {
            Some(w)
        }
    }
    fn edges(&self) -> Set<Edge> {
        self.links
            .iter()
            // Iteramos sobre cópias em vez de referências
            .copied()
            // Adicionamos um contador à cada elemento
            .enumerate()
            // Filtra links cujo peso é 0
            .filter(|(_, weight)| *weight > 0)
            // Transforma uma tupla de posição e peso em `Edge`.
            .map(|(i, weight)| {
                let y = i / self.node_count;
                let x = i % self.node_count;
                (x as Node, y as Node, weight)
            })
            .collect()
    }
    fn add_edge(&mut self, a: Node, b: Node, weight: Weight) {
        // Converte nós em `usizes` para simplificar a indexação.
        let a = a as usize;
        let b = b as usize;
        // Registra a ligação para o nó `a`
        self.links[a * self.node_count + b] = weight;
        // Registra a ligação para o nó `b`
        self.links[b * self.node_count + a] = weight;
    }

    fn get_node_edges(&self, a: Node) -> Set<Edge> {
        let a = a as usize;
        self.links[(a * self.node_count)..((a + 1) * self.node_count)]
            .iter()
            .enumerate()
            .filter(|(_, w)| **w != 0)
            .map(|(b, w)| (a as Node, b as Node, *w))
            .collect()
    }
}

/// Dado um vetor de linhas no formato "a b w", onde a e b são vértices e w é o peso da aresta
/// entre eles, preenche o grafo `graph`.
pub fn fill_graph(input_data: &[Vec<u32>], graph: &mut dyn Graph) {
    // Separa o vetor entre o primeiro elemento e o resto.
    let (head, tail) = input_data.split_first().expect("Vetor veio vazio");
    // Tenta desestruturar o vetor `head` em dois valores, executando o `else`
    // caso não seja possível.
    let [vertex_count, edge_count] = head[..] else {
        panic!("Esperava que a primeira linha contivesse exatamente dois valores.");
    };

    // Cria `vertex_count` nós.
    for _ in 0..vertex_count {
        // Para simplificar essa parte, pressupõe-se que os nós retornados são criados em órdem
        // crescente com incremento de 1, sendo o primeiro nó `0`.
        graph.add_node();
    }

    // Converte `edge_count` para `usize` para indexação.
    //
    // `usize` é um inteiro positivo cujo tamanho é definido pela arquitetura,
    // comummente utilizado para indexação.
    let edge_count = edge_count as usize;

    // Adiciona `edge_count` arestas ao grafo
    for edge_data in &tail[..edge_count] {
        let [a, b, weight] = edge_data[..] else {
            panic!("Esperava que cada linha de aresta tivesse exatamente três valores.");
        };
        // Adiciona uma aresta entre o nó `a` e o nó `b`
        //
        // Como dito anteriormente, os nós são crescentes e começam em 0, portanto, precisamos
        // subtrair 1 dos identificadores das entradas.
        graph.add_edge(a - 1, b - 1, weight);
    }
}

/// Printa as arestas do grafo
pub fn print_edges(graph: &dyn Graph) {
    let edges = graph.edges();
    for edge in edges {
        // Como os nós começam em 0, somamos 1 para ficar igual à entrada.
        println!("{} {} {}", edge.0 + 1, edge.1 + 1, edge.2);
    }
}

pub fn open_file() -> BufReader<File> {
    let file = File::open(
        std::env::args_os()
            .nth(1)
            .expect("Esperava nome do arquivo de entrada"),
    )
    .expect("Falha ao abrir arquivo de entrada");
    BufReader::new(file)
}

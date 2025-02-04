use std::{fs::File, io::BufReader, ops::Range};

pub mod knapsack;
pub mod tsp;
pub mod utils;

pub type Set<T> = Vec<T>;

/// Nós são identificados pelo tipo `u32`, que é um inteiro de 32 bits positivo.
///
/// Equivale a um typedef em C++.
pub type Node = u32;
/// Definimos pesos das arestas como sendo float de 32bits.
pub type Weight = ordered_float::OrderedFloat<f64>;
/// Definimos nossas arestas como sendo uma tupla de dois nós e um peso.
#[derive(Clone, Copy, Default, Debug)]
pub struct Edge(pub Node, pub Node, pub Weight);

impl Edge {
    fn as_edge(self) -> (Node, Node) {
        if self.1 > self.0 {
            (self.1, self.0)
        } else {
            (self.0, self.1)
        }
    }
}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        self.as_edge().eq(&other.as_edge())
    }
}

impl Eq for Edge {}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.as_edge().partial_cmp(&other.as_edge())
    }
}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.as_edge().cmp(&other.as_edge())
    }
}

/// Um `trait` que define os métodos que todo grafo deve implementar.
///
/// `Trait`s são análogos a classes abstratas em C++, ou interfaces em outras linguagens
pub trait Graph {
    fn add_nodes(&mut self, node_count: usize) -> Vec<Node> {
        (0..node_count)
            .into_iter()
            .map(|_| self.add_node())
            .collect()
    }
    fn add_node(&mut self) -> Node;
    fn add_edge(&mut self, a: Node, b: Node, weight: Weight);
    fn edges(&self) -> Set<Edge>;
    fn node_count(&self) -> usize;

    //
    fn get_node_edges(&self, a: Node) -> Set<Edge>;
    fn get_edge_weight_ref(&self, a: Node, b: Node) -> Option<&Weight>;
    fn get_edge_weight_mut(&mut self, a: Node, b: Node) -> Option<&mut Weight>;
    fn get_edge_weight(&self, a: Node, b: Node) -> Option<Weight> {
        self.get_edge_weight_ref(a, b).copied()
    }
    fn nodes(&self) -> Range<Node> {
        0..(self.node_count() as Node)
    }
}

impl std::ops::Index<(Node, Node)> for dyn Graph + '_ {
    type Output = Weight;
    fn index(&self, (a, b): (Node, Node)) -> &Self::Output {
        self.get_edge_weight_ref(a, b).unwrap()
    }
}

impl std::ops::IndexMut<(Node, Node)> for dyn Graph + '_ {
    fn index_mut(&mut self, (a, b): (Node, Node)) -> &mut Self::Output {
        self.get_edge_weight_mut(a, b).unwrap()
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
        let mut new_links = vec![0.into(); new_node_count.pow(2)];

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
    fn get_edge_weight_ref(&self, a: Node, b: Node) -> Option<&Weight> {
        let idx = a as usize * self.node_count + b as usize;
        let w = self.links.get(idx)?;
        if *w == 0.0 {
            None
        } else {
            Some(w)
        }
    }

    fn get_edge_weight_mut(&mut self, a: Node, b: Node) -> Option<&mut Weight> {
        let idx = a as usize * self.node_count + b as usize;
        let w = self.links.get_mut(idx)?;
        if *w == 0.0 {
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
            .filter(|(_, weight)| *weight > 0.0.into())
            // Transforma uma tupla de posição e peso em `Edge`.
            .map(|(i, weight)| {
                let y = i / self.node_count;
                let x = i % self.node_count;
                Edge(x as Node, y as Node, weight)
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
            .filter(|(_, w)| **w != 0.0)
            .map(|(b, w)| Edge(a as Node, b as Node, *w))
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
        graph.add_edge(a - 1, b - 1, weight.into());
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

use std::fmt::Debug;
use std::io::Write;
pub fn debug_to_kw(val: &dyn Debug) -> String {
    let mut buf = Vec::new();
    write!(&mut buf, "{val:#?}").unwrap();
    let text = String::from_utf8(buf).unwrap();
    let p = text.split("\n").map(|s| s.trim()).collect::<Vec<_>>();
    p[1..p.len() - 1]
        .join(";")
        .replace(": ", "=")
        .trim()
        .replace(",", "")
}

use std::collections::{HashMap, HashSet, VecDeque};

struct Graph {
    adj_list: HashMap<usize, Vec<usize>>,
}

impl Graph {
    fn new(edges: &[(usize, usize)]) -> Self {
        let mut adj_list = HashMap::new();
        for &(u, v) in edges {
            adj_list.entry(u).or_insert_with(Vec::new).push(v);
        }
        Graph { adj_list }
    }

    fn tarjan_scc(&self) -> Vec<Vec<usize>> {
        let mut index = 0;
        let mut stack = VecDeque::new();
        let mut in_stack = HashSet::new();
        let mut indices = HashMap::new();
        let mut low_links = HashMap::new();
        let mut sccs = Vec::new();

        for &node in self.adj_list.keys() {
            if !indices.contains_key(&node) {
                self.dfs(
                    node,
                    &mut index,
                    &mut stack,
                    &mut in_stack,
                    &mut indices,
                    &mut low_links,
                    &mut sccs,
                );
            }
        }

        sccs
    }

    fn dfs(
        &self,
        u: usize,
        index: &mut usize,
        stack: &mut VecDeque<usize>,
        in_stack: &mut HashSet<usize>,
        indices: &mut HashMap<usize, usize>,
        low_links: &mut HashMap<usize, usize>,
        sccs: &mut Vec<Vec<usize>>,
    ) {
        indices.insert(u, *index);
        low_links.insert(u, *index);
        *index += 1;
        stack.push_front(u);
        in_stack.insert(u);

        if let Some(neighbors) = self.adj_list.get(&u) {
            for &v in neighbors {
                if !indices.contains_key(&v) {
                    self.dfs(v, index, stack, in_stack, indices, low_links, sccs);
                    if let (Some(&low_u), Some(&low_v)) = (low_links.get(&u), low_links.get(&v)) {
                        low_links.insert(u, low_u.min(low_v));
                    }
                } else if in_stack.contains(&v) {
                    if let (Some(&idx_v), Some(&low_u)) = (indices.get(&v), low_links.get(&u)) {
                        low_links.insert(u, low_u.min(idx_v));
                    }
                }
            }
        }

        if indices[&u] == low_links[&u] {
            let mut component = Vec::new();
            while let Some(w) = stack.pop_front() {
                in_stack.remove(&w);
                component.push(w);
                if w == u {
                    break;
                }
            }
            sccs.push(component);
        }
    }
}

fn main() {
    let graph = Graph::new(&[(0, 1), (1, 2), (2, 0), (1, 3), (3, 4)]);
    let sccs = graph.tarjan_scc();
    println!("{:?}", sccs);
}

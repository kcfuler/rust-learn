pub mod find_province {
    struct Solution1;

    impl Solution1 {
        pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
            let cities = is_connected.len();
            let mut provinces = 0;
            let mut visited = vec![false; cities];

            for i in 0..cities {
                if !visited[i] {
                    Solution1::dfs(&is_connected, &mut visited, cities, i);
                }
            }

            provinces
        }

        pub fn dfs(
            connection: &Vec<Vec<i32>>,
            visited: &mut Vec<bool>,
            cities: usize,
            current_index: usize,
        ) {
            for j in 0..cities {
                if connection[current_index][j] == 1 && !visited[j] {
                    visited[j] = true;
                    Solution1::dfs(connection, visited, cities, current_index);
                }
            }
        }
    }

    struct Solution2;

    impl Solution2 {
        pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
            let cities = is_connected.len();
            let mut parents = vec![0; cities];

            for i in 0..cities {
                parents[i] = i;
            }

            for i in 0..cities {
                for j in i + 1..cities {
                    if is_connected[i][j] == 1 {
                        Solution2::union(&mut parents, i, j);
                    }
                }
            }

            let mut provinces = 0;

            for i in 0..cities {
                if Solution2::find(&mut parents, i) == i {
                    provinces += 1;
                }
            }

            provinces
        }

        pub fn union(parents: &mut Vec<usize>, i: usize, j: usize) {
            let pa = Solution2::find(parents, i);
            parents[pa] = Solution2::find(parents, j);
        }

        pub fn find(parent: &mut Vec<usize>, index: usize) -> usize {
            if parent[index] != index {
                parent[index] = Solution2::find(parent, parent[index]);
            }
            parent[index]
        }
    }
}

pub mod eternal_safe_node {
    use std::collections::VecDeque;

    pub struct Solution;
    #[derive(Clone, Copy)]
    enum State {
        Ready,
        Started,
        Finish(bool),
    }

    pub fn eventual_safe_nodes(graph: Vec<Vec<i32>>) -> Vec<i32> {
        let n = graph.len();
        let mut states: Vec<State> = vec![State::Ready; n];

        fn dfs(g: &Vec<Vec<i32>>, states: &mut Vec<State>, cur: usize) -> bool {
            match states[cur] {
                State::Started => false,
                State::Finish(result) => result,
                State::Ready => {
                    states[cur] = State::Started;
                    let result = g[cur].iter().all(|&son| dfs(g, states, son as usize));
                    states[cur] = State::Finish(result);
                    result
                }
            }
        }

        (0..n as i32)
            .filter(|&i| dfs(&graph, &mut states, i as usize))
            .collect()
    }

    // use top_sort to resolve
    pub fn top_sort(graph: Vec<Vec<i32>>) -> Vec<i32> {
        let n = graph.len();
        let mut g = vec![vec![]; n]; // incoming node
        let mut d = vec![0; n]; // out degree
        let mut queue = vec![]; // safe nodes

        for (i, v) in graph.into_iter().enumerate() {
            d[i] = v.len(); // caculate out degree
            if d[i] == 0 {
                queue.push(i); // get safe nodes
            }

            // make incoming graph
            for x in v.into_iter().map(|x| x as usize) {
                g[x].push(i);
            }
        }

        let mut idx = 0;
        while idx < queue.len() {
            for &y in &g[queue[idx]] {
                d[y] -= 1;
                if d[y] == 0 {
                    queue.push(y);
                }
            }

            idx += 1;
        }

        queue.sort();
        queue.into_iter().map(|x| x as i32).collect::<Vec<_>>()
    }
}

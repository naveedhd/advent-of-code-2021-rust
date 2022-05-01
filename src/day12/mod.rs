use std::collections::HashMap;
use std::collections::HashSet;


type Graph = HashMap<String, Vec<String>>;


pub fn solve(file_string: &str) {
    let data = parse(&file_string);

    println!("{}", solve1(&data));
    println!("{}", solve2(&data));
}

fn parse(file_string: &str) -> Graph {
    let mut graph = Graph::new();

    for line in file_string.lines() {
        let (a, b) = line.split_once('-').unwrap();

        graph.entry(a.to_owned()).or_default().push(b.to_owned());
        graph.entry(b.to_owned()).or_default().push(a.to_owned());
    }

    graph
}

fn is_small_node(s: &str) -> bool {
    s.chars().nth(0).unwrap().is_lowercase()
}

fn dfs(graph: &Graph, node: String, visited: &mut HashSet<String>) -> usize {
    if node == "end" {
        return 1;
    }

    if visited.contains(&node) {
        return 0;
    }

    let is_this_small_node = is_small_node(&node);

    if is_this_small_node {
        visited.insert(node.clone());
    }

    let score = graph
        .get(&node)
        .unwrap()
        .iter()
        .map(|conn| dfs(graph, conn.clone(), visited))
        .sum();

    if is_this_small_node {
        visited.remove(&node);
    }

    score
}

fn dfs2(graph: &Graph, node: String, visited: &mut HashMap<String, usize>) -> usize {

    if node == "start" {
        return 0;
    }

    if node == "end" {
        return 1;
    }

    if is_small_node(&node) {

        let nodes_visited_more_than_once = visited.values().filter(|&v| *v > 1).count();

        // what are the conditions to not visit this node?
        // 0. this node should not be visited for more than twice
        if visited.contains_key(&node) && *visited.get(&node).unwrap() > 1 {
            return 0;
        }

        // 1. some node is already visited twice and this node already has 1 visit
        if nodes_visited_more_than_once >= 1
           && (visited.contains_key(&node) && *visited.get(&node).unwrap() > 0) {
            return 0;
        }

        let e = visited.entry(node.clone()).or_default();
        *e += 1;
    }

    let score = graph
        .get(&node)
        .unwrap()
        .iter()
        .map(|conn| dfs2(graph, conn.clone(), visited))
        .sum();

    if is_small_node(&node) {
        *visited.get_mut(&node).unwrap() -= 1;
    }

    score
}

fn solve1(graph: &Graph) -> usize {
    let mut visited = HashSet::new();
    dfs(graph, "start".to_owned(), &mut visited)
}

fn solve2(graph: &Graph) -> usize {
    let mut visited = HashMap::new();
    graph
        .get("start")
        .unwrap()
        .iter()
        .map(|conn| dfs2(graph, conn.clone(), &mut visited))
        .sum()
}

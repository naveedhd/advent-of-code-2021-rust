use std::collections::HashMap;
use std::collections::HashSet;

type Node = (usize, usize);
type Cost = usize;
type Graph = HashMap<Node, Cost>;
type Table = HashMap<Node, Cost>;
type Visited = HashSet<Node>;

pub fn solve(filestring: &str) {
    let (mut graph, mut bounds) = parse(filestring);

    lowest_risk_path(&graph, &bounds);

    extend_graph(&mut graph, &mut bounds);

    lowest_risk_path(&graph, &bounds);
}

fn parse(filestring: &str) -> (Graph, Node) {
    let mut graph = Graph::new();

    let parse_char = |c: char| c.to_digit(10).unwrap() as usize;

    for (y, line) in filestring.lines().enumerate() {
        for (x, cost_char) in line.chars().enumerate() {
            let node = (x, y);
            let cost = parse_char(cost_char);
            graph.insert(node, cost);
        }
    }

    let max_x = filestring.lines().count();
    let max_y = filestring.lines().nth(0).unwrap().chars().count();

    (graph, (max_x, max_y))
}

fn extend_graph(graph: &mut Graph, bounds: &mut Node) {
    let mut max_x = bounds.0;
    let mut max_y = bounds.1;

    let wrap = |cost, increase| (cost + increase) % 10;

    // extend first the down column
    // we want to repeat this 4 times
    for time in 1..5 {
        for y in 0..max_y {
            for x in 0..max_x {
                let original_node = (x, y);
                let node_to_create: Node = (x, time * max_y + y);
                let node_cost = graph.get(&original_node).unwrap().clone();
                let create_value = wrap(node_cost, time);

                graph.insert(node_to_create, create_value);
            }
        }
    }

    // increase the max_y
    max_y *= 5;

    // then extends all the rows rightward
    for time in 1..5 {
        for y in 0..max_y {
            for x in 0..max_x {
                let original_node = (x, y);
                let node_to_create: Node = (time * max_x + x, y);
                let node_cost = graph.get(&original_node).unwrap().clone();
                let create_value = wrap(node_cost, time);

                graph.insert(node_to_create, create_value);
            }
        }
    }

    max_x *= 5;

    bounds.0 = max_x;
    bounds.1 = max_y;
}

fn min_cost_node(table: &Table, visited: &Visited) -> Option<Node> {
    let mut min_node: Option<Node> = None;
    let mut min_cost: Cost = usize::MAX;

    for (node, cost) in table {
        if visited.contains(node) {
            continue;
        }

        if *cost < min_cost {
            min_node = Some(*node);
            min_cost = *cost;
        }
    }

    min_node
}

fn update_neighbour(neighbour_node: Node, node_cost: Cost, table: &mut Table, graph: &Graph) {
    let up_node_cost = table.get_mut(&neighbour_node).unwrap();
    let cost_to_go_from_this_node = node_cost + graph.get(&neighbour_node).unwrap();
    if cost_to_go_from_this_node < *up_node_cost {
        *up_node_cost = cost_to_go_from_this_node;
    }
}

fn update_neighbours(node: &Node, bounds: &Node, table: &mut Table, graph: &Graph) {
    // visit it's neighbour updating the cost_pair
    let x = node.0;
    let y = node.1;
    let cost = table.get(&node).unwrap().clone();

    // up
    if y > 1 {
        let up = (x, y - 1);
        update_neighbour(up, cost, table, &graph);
    }

    // down
    if y < bounds.1 - 1 {
        let down = (x, y + 1);
        update_neighbour(down, cost, table, &graph);
    }

    // left
    if x > 1 {
        let left = (x - 1, y);
        update_neighbour(left, cost, table, &graph);
    }

    // right
    if x < bounds.0 - 1 {
        let right = (x + 1, y);
        update_neighbour(right, cost, table, &graph);
    }
}

fn lowest_risk_path(graph: &Graph, bounds: &Node) {
    let mut visited = Visited::new();

    let mut table: Table = graph.keys().map(|&key| (key, usize::MAX)).collect();

    // set start to 0
    let start_node: Node = (0, 0);
    *table.get_mut(&start_node).unwrap() = 0;

    // while there is some minimum node that is not visited
    while let Some(node) = min_cost_node(&table, &visited) {
        update_neighbours(&node, bounds, &mut table, graph);

        // set this to visited
        visited.insert(node);
    }

    // trace cost to end
    let end_node = (bounds.0 - 1, bounds.1 - 1);
    let cost_to_end_node = table.get(&end_node).unwrap();

    println!("{}", cost_to_end_node);
}

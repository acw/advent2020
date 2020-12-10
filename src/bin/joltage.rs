use advent2020::errors::TopLevelError;
use petgraph::graphmap::GraphMap;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::str::FromStr;

fn visit_all_nodes(
    graph: &GraphMap<u64, u64, petgraph::Directed>,
    start: u64,
    end: u64,
) -> Result<(usize, usize, usize), TopLevelError> {
    let mut ones = 0;
    let mut twos = 0;
    let mut threes = 0;
    let mut current = start;
    while current != end {
        let (_, next, edge_length) = graph
            .edges(current)
            .min()
            .ok_or(TopLevelError::UnknownError)?;
        match edge_length {
            0 => return Err(TopLevelError::UnknownError),
            1 => ones += 1,
            2 => twos += 1,
            3 => threes += 1,
            _ => return Err(TopLevelError::UnknownError),
        }
        current = next;
    }
    Ok((ones, twos, threes))
}

fn path_counts(
    graph: &GraphMap<u64, u64, petgraph::Directed>,
    cheat_codes: &mut HashMap<u64, usize>,
    start: u64,
    end: u64,
) -> usize {
    if start == end {
        return 1;
    }

    let mut sum = 0;

    for (_, to, _) in graph.edges(start) {
        match cheat_codes.get(&to) {
            None => {
                let result = path_counts(graph, cheat_codes, to, end);
                cheat_codes.insert(to, result);
                sum += result;
            }
            Some(result) => {
                sum += result;
            }
        }
    }

    sum
}

fn main() -> Result<(), TopLevelError> {
    let filename = env::args().nth(1).expect("No file argument given.");
    let contents = fs::read_to_string(filename)?;
    let mut graph = GraphMap::new();
    let mut nodes = Vec::new();

    // add the nodes in the graph, which are weighted by their values
    let mut max_jolts = 0;
    for line in contents.lines() {
        let value = u64::from_str(line)?;
        if value > max_jolts {
            max_jolts = value;
        }
        graph.add_node(value);
        nodes.push(value);
    }
    max_jolts += 3;

    graph.add_node(0);
    nodes.push(0); // outlet
    graph.add_node(max_jolts);
    nodes.push(max_jolts); // my device

    for start in nodes.iter() {
        for end in nodes.iter() {
            if (*end >= (start + 1)) && (*end <= (start + 3)) {
                graph.add_edge(*start, *end, end - start);
            }
        }
    }

    let (one_count, two_count, three_count) = visit_all_nodes(&graph, 0, max_jolts)?;
    println!("Found a path that visits all nodes:");
    println!("  jumps of size 1: {}", one_count);
    println!("  jumps of size 2: {}", two_count);
    println!("  jumps of size 3: {}", three_count);
    println!(
        "  product of size 1 and size 3: {}",
        one_count * three_count
    );
    let mut cheat_codes = HashMap::with_capacity(10000);
    println!(
        "Total number of paths: {}",
        path_counts(&graph, &mut cheat_codes, 0, max_jolts)
    );

    Ok(())
}

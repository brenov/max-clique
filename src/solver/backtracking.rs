use crate::graph::Graph;

/// Solves the maximum clique problem by using a backtracking.
pub fn solve(graph: &Graph) -> Graph {
  backtracking(&graph, &graph.nodes(), Graph::default())
}

fn backtracking(graph: &Graph, nodes: &[usize], mut clique: Graph) -> Graph {
  // Clone current solution
  let mut subgraph = clique.clone();
  // Visit all nodes
  for (i, n) in nodes.iter().enumerate() {
    // Add node
    subgraph.insert_node(*n);
    // Add edges
    for c in subgraph.nodes() {
      if graph.adjlst_of(*n).contains(&c) {
        subgraph.insert_edge((c, *n));
      }
    }
    // Create a backtracking branch and get the branch best solution
    let sol = backtracking(graph, &nodes[i + 1..], subgraph.clone());
    // Check if the branch best solution is better than the current one
    if (sol.is_complete() && clique.is_empty()) ||
      (sol.is_complete() && sol.degree() >= clique.degree()) { clique = sol; }
    // Remove added node
    subgraph.remove_node(*n);
  }
  clique
}

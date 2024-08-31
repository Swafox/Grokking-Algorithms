use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    node: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn dijkstra(
    graph: &HashMap<usize, Vec<(usize, usize)>>,
    start: usize,
    end: usize,
) -> Option<usize> {
    if graph.is_empty() {
        return None;
    }

    let mut dist: Vec<_> = (0..graph.len()).map(|_| usize::MAX).collect();
    let mut heap: BinaryHeap<State> = BinaryHeap::new();

    dist[start] = 0;
    heap.push(State {
        cost: 0,
        node: start,
    });

    while let Some(State { cost, node }) = heap.pop() {
        if node == end {
            return Some(cost);
        }

        if cost > dist[node] {
            continue;
        }

        if let Some(neighbors) = graph.get(&node) {
            for &(next, edge_cost) in neighbors {
                let next_cost = cost + edge_cost;
                if next_cost < dist[next] {
                    dist[next] = next_cost;
                    heap.push(State {
                        cost: next_cost,
                        node: next,
                    });
                }
            }
        }
    }

    None
}

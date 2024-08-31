#[cfg(test)]
mod tests {
    use rand::Rng;
    use std::{
        collections::HashMap,
        time::{Duration, Instant},
    };

    use dijkstra::dijkstra;

    #[test]
    fn test_dijkstra() {
        let mut graph: HashMap<usize, Vec<(usize, usize)>> = HashMap::new();
        graph.insert(0, vec![(1, 4), (2, 1)]);
        graph.insert(1, vec![(3, 1)]);
        graph.insert(2, vec![(1, 2), (3, 5)]);
        graph.insert(3, vec![]);

        assert_eq!(dijkstra(&graph, 0, 3), Some(4));
        assert_eq!(dijkstra(&graph, 0, 1), Some(3));
        assert_eq!(dijkstra(&graph, 2, 3), Some(3));
        assert_eq!(dijkstra(&graph, 3, 0), None);
    }

    #[test]
    fn test_dijkstra_empty_graph() {
        let graph: HashMap<usize, Vec<(usize, usize)>> = HashMap::new();
        assert_eq!(dijkstra(&graph, 0, 1), None);
    }

    #[test]
    fn test_dijkstra_single_node() {
        let mut graph: HashMap<usize, Vec<(usize, usize)>> = HashMap::new();
        graph.insert(0, vec![]);
        assert_eq!(dijkstra(&graph, 0, 0), Some(0));
    }

    fn generate_random_graph(
        size: usize,
        edge_density: f64,
    ) -> HashMap<usize, Vec<(usize, usize)>> {
        let mut rng: rand::prelude::ThreadRng = rand::thread_rng();
        let mut graph: HashMap<usize, Vec<(usize, usize)>> = HashMap::new();

        for i in 0..size {
            let mut neighbors: Vec<(usize, usize)> = Vec::new();
            for j in 0..size {
                if i != j && rng.gen::<f64>() < edge_density {
                    neighbors.push((j, rng.gen_range(1..100)));
                }
            }
            graph.insert(i, neighbors);
        }

        graph
    }

    fn benchmark_dijkstra(
        graph: &HashMap<usize, Vec<(usize, usize)>>,
        iterations: u32,
    ) -> Duration {
        let mut rng: rand::prelude::ThreadRng = rand::thread_rng();
        let mut total_duration: Duration = Duration::new(0, 0);

        for _ in 0..iterations {
            let start: usize = rng.gen_range(0..graph.len());
            let end: usize = rng.gen_range(0..graph.len());

            let start_time: Instant = Instant::now();
            let _ = dijkstra(graph, start, end);
            total_duration += start_time.elapsed();
        }

        total_duration / iterations
    }

    #[test]
    fn test_dijkstra_performance() {
        let sizes: Vec<usize> = vec![100, 1000];
        let edge_density: f64 = 0.1;
        let iterations: u32 = 100;

        println!("Dijkstra's Algorithm Performance Benchmark:");
        for size in sizes {
            let graph: HashMap<usize, Vec<(usize, usize)>> =
                generate_random_graph(size, edge_density);
            let avg_time: Duration = benchmark_dijkstra(&graph, iterations);
            println!(
                "Average time for graph with {} nodes over {} iterations: {:?}",
                size, iterations, avg_time
            );
        }
    }
}

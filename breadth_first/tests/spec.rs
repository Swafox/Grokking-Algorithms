#[cfg(test)]
mod tests {
    use breadth_first::breadth_first_search;

    use std::collections::HashMap;
    use std::time::Instant;

    #[test]
    fn test_breadth_first_search() {
        let mut graph: HashMap<String, Vec<String>> = HashMap::new();

        graph.insert(
            "you".to_string(),
            vec!["alice".to_string(), "bob".to_string(), "claire".to_string()],
        );
        graph.insert(
            "bob".to_string(),
            vec!["anuj".to_string(), "peggy".to_string()],
        );
        graph.insert("alice".to_string(), vec!["peggy".to_string()]);
        graph.insert(
            "claire".to_string(),
            vec!["thom".to_string(), "jonny".to_string()],
        );
        graph.insert("anuj".to_string(), Vec::new());
        graph.insert("peggy".to_string(), Vec::new());
        graph.insert("thom".to_string(), Vec::new());
        graph.insert("jonny".to_string(), Vec::new());

        assert!(breadth_first_search(graph, "you"));
    }

    #[test]
    fn test_breadth_first_search_performance() {
        let mut graph: HashMap<String, Vec<String>> = HashMap::new();

        for i in 0..1000 {
            let key: String = i.to_string();
            let values: Vec<String> = (1000..2000).map(|j| j.to_string()).collect();
            graph.insert(key, values);
        }

        let start_time: Instant = Instant::now();
        let result: bool = breadth_first_search(graph, "0");
        let duration: std::time::Duration = start_time.elapsed();

        println!(
            "Breadth-first search took {:.6} seconds",
            duration.as_secs_f64()
        );
        assert!(!result);
    }
}

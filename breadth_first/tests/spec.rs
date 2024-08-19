#[cfg(test)]
mod tests {
    use breadth_first::breadth_first_search;

    use std::collections::HashMap;

    #[test]
    fn test_breadth_first_search() {
        let mut graph: HashMap<&str, Vec<&str>> = HashMap::new();

        graph.insert("you", vec!["alice", "bob", "claire"]);
        graph.insert("bob", vec!["anuj", "peggy"]);
        graph.insert("alice", vec!["peggy"]);
        graph.insert("claire", vec!["thom", "jonny"]);
        graph.insert("anuj", Vec::new());
        graph.insert("peggy", Vec::new());
        graph.insert("thom", Vec::new());
        graph.insert("jonny", Vec::new());

        assert!(breadth_first_search(graph, "you"));
    }
}

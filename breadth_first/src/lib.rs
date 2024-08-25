use std::collections::{HashMap, HashSet, VecDeque};

fn person_is_seller(name: &str) -> bool {
    name.ends_with('m')
}

pub fn breadth_first_search(graph: HashMap<String, Vec<String>>, start: &str) -> bool {
    let mut search_queue: VecDeque<String> = VecDeque::new();
    search_queue.push_back(start.to_string());
    let mut searched: HashSet<String> = HashSet::new();

    while let Some(person) = search_queue.pop_front() {
        if !searched.contains(&person) {
            if person_is_seller(&person) {
                println!("{} is a mango seller!", person);
                return true;
            }
            if let Some(neighbors) = graph.get(&person) {
                search_queue.extend(neighbors.iter().map(|n| n.to_string()));
            }
            searched.insert(person);
        }
    }

    false
}

use std::collections::{HashMap, HashSet, VecDeque};

fn person_is_seller(name: &str) -> bool {
    name.ends_with('m')
}

pub fn breadth_first_search(graph: HashMap<&str, Vec<&str>>, start: &str) -> bool {
    let mut search_queue: VecDeque<&str> = VecDeque::new();
    search_queue.push_back(start);
    let mut searched: HashSet<&str> = HashSet::new();

    while let Some(person) = search_queue.pop_front() {
        if !searched.contains(person) {
            if person_is_seller(person) {
                println!("{} is a mango seller!", person);
                return true;
            }
            if let Some(neighbors) = graph.get(person) {
                search_queue.extend(neighbors);
            }
            searched.insert(person);
        }
    }

    false
}

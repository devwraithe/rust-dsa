// Import the binary search function from the binary search module
use data_structure_algorithms::binary_search::{binary_search, delete, insert};
use data_structure_algorithms::graph::Graph;

fn main() {
    // Define a sorted array of integers
    let arr = [1,2,3,4,5];

    // Define a target value to search for
    let target = 3;

    // Perform the binary search and store the result
    let result = binary_search(&arr, target);

    // Print the result
    match result {
        Some(index) => println!("Found target {} at index {}", target, index),
        None => eprintln!("Target {} not found", target),
    }

    // Searching for value not in the array
    let target_not_found = 6;
    let result_not_found = binary_search(&arr, target_not_found);
    match result_not_found {
        Some(index) => println!("Found target {} at index {}", target, index),
        None => eprintln!("Target {} not found", target),
    }

    // Test insertion
    let mut vec_ins = vec![1,2,3,4,5];
    println!("Original ins vector: {:?}", vec_ins);
    insert(&mut vec_ins, 3);
    println!("After insertion: {:?}", vec_ins);

    // Test deletion
    let mut vec_del = vec![1,2,3,4,5];
    print!("Origin del vector: {:?}", vec_ins);
    let del_result = delete(&mut vec_del, 3);
    println!("After deletion: {:?}, Success: {}", vec_del, del_result);

    let del_result_not_found = delete(&mut vec_del, 6);
    println!("Attempt to delete non-existent element, Success: {}", del_result_not_found);

    // Testing the graph functions
    let mut graph = Graph::new();
    graph.add_node(1);
    graph.add_node(2);
    graph.add_node(3);
    graph.add_edge(1, 2);
    graph.add_edge(2, 3);
    graph.add_edge(1, 3);

    println!("Graph traversal starting from node 1: {:?}", graph.traverse(1));
    println!("Graph traversal starting from node 2: {:?}", graph.traverse(2));

}

use sha2::{Sha256, Digest};
use std::io;

fn hash_data(data: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data.as_bytes());
    let result = hasher.finalize();
    format!("{:x}", result)
}

fn merkle_tree(leaves: Vec<&str>) -> (String, Vec<Vec<String>>) {
    let mut layer: Vec<String> = leaves.iter().map(|&leaf| hash_data(leaf)).collect();
    let mut merkle_paths: Vec<Vec<String>> = Vec::new();
    
    while layer.len() > 1 {
        if layer.len() % 2 != 0 {
            layer.push(layer.last().unwrap().clone());
        }
        
        let mut new_layer: Vec<String> = Vec::new();
        let mut current_layer_paths: Vec<String> = Vec::new();
        
        for i in (0..layer.len()).step_by(2) {
            let combined = format!("{}{}", layer[i], layer[i + 1]);
            let hash = hash_data(&combined);
            new_layer.push(hash);
            
            current_layer_paths.push(layer[i].clone());
            current_layer_paths.push(layer[i + 1].clone());
        }
        
        merkle_paths.push(current_layer_paths);
        layer = new_layer;
    }
    
    (layer[0].clone(), merkle_paths)
}

fn generate_merkle_proof(leaf: &str, leaves: &[&str], merkle_paths: &[Vec<String>]) -> Vec<String> {
    let leaf_hash = hash_data(leaf);
    let leaf_index = leaves.iter().position(|&x| hash_data(x) == leaf_hash)
        .expect("Leaf not found in the original leaves");
    
    let mut proof = Vec::new();
    let mut current_index = leaf_index;
    
    for layer_paths in merkle_paths {
        let is_left_node = current_index % 2 == 0;
        let sibling_index = if is_left_node { current_index + 1 } else { current_index - 1 };
        
        if sibling_index < layer_paths.len() {
            proof.push(layer_paths[sibling_index].clone());
        }
        
        current_index /= 2;
    }
    
    proof
}

fn verify_merkle_proof(leaf: &str, root: &str, proof: &[String], index: usize) -> bool {
    let mut current_hash = hash_data(leaf);
    let mut current_index = index;
    
    for level_proof in proof {
        let is_left_node = current_index % 2 == 0;
        
        current_hash = if is_left_node {
            hash_data(&format!("{}{}", current_hash, level_proof))
        } else {
            hash_data(&format!("{}{}", level_proof, current_hash))
        };
        
        current_index /= 2;
    }
    
    current_hash == root
}

pub fn run() {
    let mut input_leaves: Vec<String> = Vec::new();
    
    println!("Enter leaves for Merkle Tree (type 'done' to finish):");
    
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let input = input.trim();
        
        if input == "done" {
            break;
        }
        
        input_leaves.push(input.to_string());
    }
    
    // Convert input_leaves to slice of references
    let leaves: Vec<&str> = input_leaves.iter().map(|s| s.as_str()).collect();
    
    let (root_hash, merkle_paths) = merkle_tree(leaves.clone());
    println!("Merkle tree root hash: {}", root_hash);
    
    loop {
        println!("\nEnter a leaf to verify (or 'exit' to quit):");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let input = input.trim();
        
        if input == "exit" {
            break;
        }
        
        if !input_leaves.contains(&input.to_string()) {
            println!("Leaf not in the Merkle tree.");
            continue;
        }
        
        let leaf_index = leaves.iter().position(|&x| x == input).unwrap();
        let proof = generate_merkle_proof(input, &leaves, &merkle_paths);
        let is_valid = verify_merkle_proof(input, &root_hash, &proof, leaf_index);
        
        println!("Proof for leaf '{}' is valid: {}", input, is_valid);
    }
}
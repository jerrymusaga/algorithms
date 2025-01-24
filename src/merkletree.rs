use sha2::{Sha256, Digest};

/// Hashes a piece of data using SHA-256
fn hash_data(data: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data.as_bytes());
    let result = hasher.finalize();
    format!("{:x}", result) 
}

fn merkle_tree(leaves: Vec<&str>) -> String {
    
    let mut layer: Vec<String> = leaves.iter().map(|&leaf| hash_data(leaf)).collect();

   
    while layer.len() > 1 {
        
        if layer.len() % 2 != 0 {
            layer.push(layer.last().unwrap().clone());
        }

        let mut new_layer: Vec<String> = Vec::new();
        for i in (0..layer.len()).step_by(2) {
            let combined = format!("{}{}", layer[i], layer[i + 1]);
            new_layer.push(hash_data(&combined));
        }

        // Move to the new layer
        layer = new_layer;
    }

    layer[0].clone()
}

pub fn run() {
    let leaves = vec!["data1", "data2", "data3", "data4", "data5", "data6", "data7", "data8"];
    let root_hash = merkle_tree(leaves);
    println!("Merkle tree root hash: {}", root_hash);
}
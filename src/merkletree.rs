use sha2::{Sha256, Digest};

#[derive(Debug)]
struct MerkleTree {
    nodes: Vec<Vec<u8>>,
    leaf_count: usize,
}

impl MerkleTree {
    pub fn new(data: Vec<Vec<u8>>) -> Self {
        let leaf_count = data.len();
        let mut tree = MerkleTree {
            nodes: Vec::new(),
            leaf_count,
        };
        
        // Handle empty input
        if leaf_count == 0 {
            return tree;
        }
        
        // Calculate the total number of nodes needed
        let mut total_nodes = leaf_count;
        let mut level_size = leaf_count;
        while level_size > 1 {
            level_size = (level_size + 1) / 2;  // Ceiling division to handle odd numbers
            total_nodes += level_size;
        }
        
        // Initialize nodes vector with capacity
        tree.nodes = Vec::with_capacity(total_nodes);
        
        // Add leaf nodes
        for item in data {
            tree.nodes.push(tree.hash_leaf(&item));
        }
        
        // Build the tree
        tree.build_tree();
        tree
    }
    
    fn hash_leaf(&self, data: &[u8]) -> Vec<u8> {
        let mut hasher = Sha256::new();
        hasher.update([0x00]);  // Prefix for leaf nodes
        hasher.update(data);
        hasher.finalize().to_vec()
    }
    
    fn hash_nodes(&self, left: &[u8], right: &[u8]) -> Vec<u8> {
        let mut hasher = Sha256::new();
        hasher.update([0x01]);  // Prefix for internal nodes
        hasher.update(left);
        hasher.update(right);
        hasher.finalize().to_vec()
    }
    
    fn build_tree(&mut self) {
        let mut level_size = self.leaf_count;
        let mut level_offset = 0;
        
        while level_size > 1 {
            let next_level_size = (level_size + 1) / 2;
            
            for i in (0..level_size).step_by(2) {
                let left = &self.nodes[level_offset + i];
                
                // If this is the last node and level_size is odd, duplicate it
                let right = if i + 1 < level_size {
                    &self.nodes[level_offset + i + 1]
                } else {
                    left
                };
                
                let parent = self.hash_nodes(left, right);
                self.nodes.push(parent);
            }
            
            level_offset += level_size;
            level_size = next_level_size;
        }
    }
    
    pub fn get_root(&self) -> Option<&[u8]> {
        self.nodes.last().map(|v| v.as_slice())
    }
    
    pub fn verify(&self, leaf_data: &[u8], proof: &[Vec<u8>], index: usize) -> bool {
        if index >= self.leaf_count {
            return false;
        }
        
        let mut current_hash = self.hash_leaf(leaf_data);
        let mut current_index = index;
        
        for sibling in proof {
            let (left, right) = if current_index % 2 == 0 {
                (&current_hash, sibling)
            } else {
                (sibling, &current_hash)
            };
            
            current_hash = self.hash_nodes(left, right);
            current_index /= 2;
        }
        
        Some(current_hash.as_slice()) == self.get_root()
    }
    
    pub fn generate_proof(&self, index: usize) -> Option<Vec<Vec<u8>>> {
        if index >= self.leaf_count {
            return None;
        }
        
        let mut proof = Vec::new();
        let mut current_index = index;
        let mut level_size = self.leaf_count;
        let mut level_offset = 0;
        
        while level_size > 1 {
            let sibling_index = if current_index % 2 == 0 {
                current_index + 1
            } else {
                current_index - 1
            };
            
            if sibling_index < level_size {
                proof.push(self.nodes[level_offset + sibling_index].clone());
            }
            
            level_offset += level_size;
            level_size = (level_size + 1) / 2;
            current_index /= 2;
        }
        
        Some(proof)
    }
}

// Example usage
pub fn run() {
    let data = vec![
        b"Transaction1".to_vec(),
        b"Transaction2".to_vec(),
        b"Transaction3".to_vec(),  // Odd number of transactions
    ];
    
    let tree = MerkleTree::new(data.clone());
    
    // Generate and verify a proof for the second transaction
    if let Some(proof) = tree.generate_proof(1) {
        let is_valid = tree.verify(&data[1], &proof, 1);
        println!("Proof verification result: {}", is_valid);
    }
}
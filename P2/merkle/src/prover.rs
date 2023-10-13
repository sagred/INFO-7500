use crate::util::{Hash32Bytes, write_merkle_proof,encode_hash,hash_internal, MerkleProof, hash_leaf};

fn gen_leaves_for_merkle_tree(num_leaves: usize) -> Vec<String> {
    let leaves: Vec<String> = (0..num_leaves)
        .map(|i| format!("data item {}", i))
        .collect();

    println!("\nI generated #{} leaves for a Merkle tree.", num_leaves);

    leaves
}
pub fn gen_merkle_proof(leaves: Vec<String>, leaf_pos: usize) -> Vec<Hash32Bytes> {
    let height = (leaves.len() as f64).log2().ceil() as u32;
    let padlen = (2u32.pow(height)) as usize - leaves.len();

    let mut state: Vec<Hash32Bytes> = leaves.into_iter().map(hash_leaf).collect();

    let zeros = [0u8; 32];
    for _ in 0..padlen {
        state.push(zeros);
    }

    let mut hashes: Vec<Hash32Bytes> = vec![];

    let mut level_pos = leaf_pos;
    for _level in 0..height {
        let sibling_pos = if level_pos % 2 == 0 { level_pos + 1 } else { level_pos - 1 };

        hashes.push(state[sibling_pos]);

        level_pos /= 2;

        let mut next_state: Vec<Hash32Bytes> = Vec::new();
        for i in (0..state.len()).step_by(2) {
            next_state.push(hash_internal(state[i], state[i + 1]));
        }
        state = next_state;
    }
    

    hashes
}

pub fn run(leaf_position: usize) {
    const NUM_LEAVES: usize = 1000; // replace with your actual constant

    let leaves = gen_leaves_for_merkle_tree(NUM_LEAVES);
    assert!(leaf_position < leaves.len());
    let leaf_value = leaves[leaf_position].clone();
    let hashes = gen_merkle_proof(leaves, leaf_position);

    let mut proof_hash_values_base64: Vec<String> = Vec::new();

    for hash in hashes {
        proof_hash_values_base64.push(encode_hash(hash))
    }

    let proof = MerkleProof{
        leaf_position,
        leaf_value,
        proof_hash_values_base64,
        proof_hash_values: None,
    };

    write_merkle_proof(&proof, "proof_gen.yaml");
}


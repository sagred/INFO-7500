use std::collections::VecDeque;
use crate::util;
use crate::util::{decode_hash,read_merkle_proof,MerkleProof, hash_internal, encode_hash};


pub fn run(proof_file: &String) {
    let merkle_proof = read_merkle_proof(proof_file);
    verify_merkle_proof(merkle_proof)
}


fn verify_merkle_proof(merkle_proof: Box<MerkleProof>) {
    let root = decode_hash("xMUi3TVE4yvKXoA4Xrsmzw0Otb51PxnjiFXqPwovXyg=");
    let computed_root = compute_merkle_root_from_merkle_proof(merkle_proof);
    let _encoded = encode_hash(computed_root);
    // println!("Encoded computed_root: {}", encoded);
    println!("computed_root: {:?}", computed_root);
    assert_eq!(computed_root, root);
}

fn compute_merkle_root_from_merkle_proof(merkle_proof: Box<MerkleProof>) -> util::Hash32Bytes {
    let mut pos = merkle_proof.leaf_position;
    let mut hashes = VecDeque::from(merkle_proof.proof_hash_values.unwrap());
    let mut cur_root = util::hash_leaf(merkle_proof.leaf_value);
   
    while let Some(hash) = hashes.pop_front() {
        let (left, right);
        if pos % 2 == 0 {
            left = cur_root;
            right = hash;
        } else {
            left = hash;
            right = cur_root;
        }
        cur_root = hash_internal(left, right);
        pos >>= 1;
    }
    cur_root
}

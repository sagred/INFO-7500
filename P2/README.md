# Problem Set 2

## How to Run

### Clone the Repository

```shell
cd merkel
```

### Build and Run

1. **Generate a Merkle Proof**

   Generate a Merkle proof for a leaf at a specific position.

   ```shell
   cargo run -- prove [LEAF_POSITION]
   ```
   Replace `[LEAF_POSITION]` with the position of the leaf for which you wish to generate the Merkle proof.

2. **Verify a Merkle Proof**

   Verify the validity of a provided Merkle proof using a proof file.

   ```shell
   cargo run -- verify [PROOF_FILE]
   ```
   Replace `[PROOF_FILE]` with the path to the file containing the Merkle proof to be verified.

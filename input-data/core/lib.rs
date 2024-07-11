use k256::ecdsa::Signature;
use risc0_merkle::merkle::MerkleTree;


pub struct SigMerkleTree<const N: u32>(MerkleTree<Signature>);

impl<const N: u32> SigMerkleTree<N> {
    pub fn new(signatures: Vec<Signature>) -> Self {
    
        Self(MerkleTree::new(signatures))
    }

    pub fn root(&self) -> Node {
        self.0.root()
    }

    #[cfg(not(target_os = "zkvm"))]
    pub fn vector_oracle_callback<'a>(
        &'a self,
    ) -> impl Fn(risc0_zkvm::Bytes) -> risc0_zkvm::Result<risc0_zkvm::Bytes> + 'a {
        self.0.vector_oracle_callback()
    }
}
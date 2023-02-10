use alloc::sync::Arc;

pub struct OfaChainWrapper<Chain> {
    pub chain: Arc<Chain>,
}

impl<Chain> OfaChainWrapper<Chain> {
    pub fn new(chain: Chain) -> Self {
        Self {
            chain: Arc::new(chain),
        }
    }

    pub fn from_arc(chain: Arc<Chain>) -> Self {
        Self { chain }
    }
}

impl<Chain> Clone for OfaChainWrapper<Chain> {
    fn clone(&self) -> Self {
        Self {
            chain: self.chain.clone(),
        }
    }
}

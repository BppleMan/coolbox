use std::sync::atomic::AtomicBool;
use std::sync::Arc;

#[derive(Default, Debug, Clone)]
pub struct State {
    pub checked: Arc<AtomicBool>,
}

use std::sync::Arc;

use crate::application::{App, AppResult};

pub trait Command {
    fn execute(&self, state: &mut App) -> AppResult<()>;
}

#[derive(Clone)]
pub struct CommandBox(pub Arc<dyn Command + Send + Sync>);

impl CommandBox {
    pub(super) fn new<T: Command + Send + Sync + 'static>(cmd: T) -> Self {
        Self(Arc::new(cmd))
    }
}

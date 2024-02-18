use crate::write_diagnostic;
use exc_diagnostic::Diagnostics;
use parking_lot::Mutex;
use std::sync::Arc;
use tokio::{
    sync::mpsc::{self, UnboundedReceiver, UnboundedSender},
    task::JoinHandle,
};

#[derive(Debug)]
pub struct DiagnosticsReceiver {
    sender: UnboundedSender<Diagnostics>,
    diagnostics: Arc<Mutex<Vec<Diagnostics>>>,
    receiver_handle: JoinHandle<()>,
}

impl DiagnosticsReceiver {
    pub fn new(print_diagnostics: bool) -> Self {
        let (sender, receiver) = mpsc::unbounded_channel();
        let diagnostics = Arc::new(Mutex::new(Vec::new()));
        let receiver_handle =
            create_diagnostics_receiver(receiver, diagnostics.clone(), print_diagnostics);

        Self {
            sender,
            diagnostics,
            receiver_handle,
        }
    }

    pub fn sender(&self) -> UnboundedSender<Diagnostics> {
        self.sender.clone()
    }

    pub async fn into_diagnostics(self) -> Vec<Diagnostics> {
        drop(self.sender);
        self.receiver_handle.await.unwrap();
        Arc::try_unwrap(self.diagnostics).unwrap().into_inner()
    }
}

fn create_diagnostics_receiver(
    mut receiver: UnboundedReceiver<Diagnostics>,
    collection: Arc<Mutex<Vec<Diagnostics>>>,
    print_diagnostics: bool,
) -> JoinHandle<()> {
    tokio::spawn(async move {
        while let Some(diagnostics) = receiver.recv().await {
            if print_diagnostics {
                write_diagnostic(&diagnostics);
            }

            collection.lock().push(diagnostics);
        }
    })
}

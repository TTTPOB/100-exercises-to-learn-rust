use crate::data::{Ticket, TicketDraft};
use crate::store::{TicketId, TicketStore};
use std::sync::mpsc::{Receiver, Sender};
use std::sync::Arc;

pub mod data;
pub mod store;

#[derive(Clone)]
// TODO: flesh out the client implementation.
pub struct TicketStoreClient {
    cmd_sender: Sender<Command>,
    insert_response_sender: Sender<TicketId>,
    insert_response_receiver: Arc<Receiver<TicketId>>,
    get_response_sender: Sender<Option<Ticket>>,
    get_response_receiver: Arc<Receiver<Option<Ticket>>>,
}

impl TicketStoreClient {
    // Feel free to panic on all errors, for simplicity.
    pub fn insert(&self, draft: TicketDraft) -> TicketId {
        self.cmd_sender.send(Command::Insert {
            draft: draft,
            response_channel: self.insert_response_sender.clone(),
        }).unwrap();
        self.insert_response_receiver.recv().unwrap()
    }

    pub fn get(&self, id: TicketId) -> Option<Ticket> {
        self.cmd_sender.send(Command::Get {
            id,
            response_channel: self.get_response_sender.clone(),
        }).unwrap();
        self.get_response_receiver.recv().unwrap()
    }
}

pub fn launch() -> TicketStoreClient {
    let (sender, receiver) = std::sync::mpsc::channel();
    std::thread::spawn(move || server(receiver));
    let (insert_response_sender, insert_response_receiver) = std::sync::mpsc::channel();
    let (get_response_sender, get_response_receiver) = std::sync::mpsc::channel();
    TicketStoreClient {
        cmd_sender: sender,
        insert_response_sender,
        insert_response_receiver: Arc::new(insert_response_receiver),
        get_response_sender: get_response_sender,
        get_response_receiver: Arc::new(get_response_receiver),
    }
}

// No longer public! This becomes an internal detail of the library now.
enum Command {
    Insert {
        draft: TicketDraft,
        response_channel: Sender<TicketId>,
    },
    Get {
        id: TicketId,
        response_channel: Sender<Option<Ticket>>,
    },
}

fn server(receiver: Receiver<Command>) {
    let mut store = TicketStore::new();
    loop {
        match receiver.recv() {
            Ok(Command::Insert {
                draft,
                response_channel,
            }) => {
                let id = store.add_ticket(draft);
                let _ = response_channel.send(id);
            }
            Ok(Command::Get {
                id,
                response_channel,
            }) => {
                let ticket = store.get(id);
                let _ = response_channel.send(ticket.cloned());
            }
            Err(_) => {
                // There are no more senders, so we can safely break
                // and shut down the server.
                break;
            }
        }
    }
}

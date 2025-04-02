use parking_lot::RwLock;
use std::collections::BTreeMap;
use std::sync::Arc;
use thiserror::Error;
use tokio::sync::mpsc::{self, Receiver, Sender};

use crate::ticket;

use super::ticket::{Ticket, TicketId, TicketPatch};

#[derive(Debug, Clone)]
pub struct TicketStore {
    tickets: Arc<RwLock<BTreeMap<TicketId, Ticket>>>,
}

#[derive(Debug, Error)]
pub enum PatchError {
    #[error("Requested Ticket {0} not found")]
    NotFound(TicketId),
    #[error(transparent)]
    Mismatch(#[from] ticket::TicketUpdateError),
}
impl TicketStore {
    pub fn new() -> Self {
        TicketStore {
            tickets: Arc::new(RwLock::new(BTreeMap::new())),
        }
    }
    pub fn get(&self, id: TicketId) -> Option<Ticket> {
        self.tickets.read().get(&id).cloned()
    }
    pub fn insert(&self, ticket: Ticket){
        self.tickets.write().insert(ticket.id, ticket);
    }
    pub fn patch(&self, id: TicketId, p: TicketPatch) -> Result<(), PatchError> {
        let mut tickets = self.tickets.write();
        if let Some(existing_ticket) = tickets.get_mut(&id) {
            existing_ticket.update(p)?;
            Ok(())
        } else {
            Err(PatchError::NotFound(id))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ticket::TicketStatus;

    fn get_ticket() -> Ticket {
        Ticket::new(
            42.into(),
            "this is a title".try_into().unwrap(),
            "this is a description".try_into().unwrap(),
            "todo".try_into().unwrap(),
        )
    }

    #[test]
    fn test_ticket_insert_get() {
        let store = TicketStore::new();
        let ticket = get_ticket();
        store.insert(ticket.clone());

        assert_eq!(store.get(ticket.id), Some(ticket.clone()));
    }
    #[test]
    fn test_ticket_patch() {
        let store = TicketStore::new();
        let ticket = get_ticket();
        store.insert(ticket.clone());

        let patch = TicketPatch::new(ticket.id, None, None, Some(TicketStatus::Done));
        store.patch(ticket.id, patch).unwrap();

        assert_eq!(store.get(ticket.id).unwrap().status, TicketStatus::Done);
    }
}

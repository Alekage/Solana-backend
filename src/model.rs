use crate::{error, LoginError, Result};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};


#[derive(Clone, Debug, Serialize)]
pub struct Ticket {
    pub id: u64,
    pub title: String,
}

#[derive(Deserialize)]
pub struct TicketForCreate {
    pub title: String
}

/// Model Conroler is a component/module responsible for managing
/// and interacting with the data model of an application
#[derive(Clone)]
pub struct ModelController {
    /// As of right now, this is in-memory data structure, 
    /// which will become a database at one point
    tickets_store: Arc<Mutex<Vec<Option<Ticket>>>>
}


impl ModelController {
    // Constructor function
    pub async fn new() -> Result<Self> {
        Ok(Self {
            tickets_store: Arc::default(),
        })
    }

    // CRUD functions:

    // Create
    pub async fn create_ticket(&self, ticket: TicketForCreate) -> Result<Ticket> {
        let mut store = self.tickets_store.lock().unwrap();
        let id = store.len() as u64;

        let ticket = Ticket {
            id,
            title: ticket.title
        };

        store.push(Some(ticket.clone()));

        Ok(ticket)
    }

    // Read
    pub async fn ticket_list(&self) -> Result<Vec<Ticket>> {
        let store = self.tickets_store.lock().unwrap();

        let tickets: Vec<Ticket> = store.iter().filter_map(|t| t.clone()).collect();
        
        Ok(tickets)
    }

    // Delete 
    pub async fn delete_ticket(&self, id: u64) -> Result<Ticket> {
        let mut store = self.tickets_store.lock().unwrap();

        let ticket = store.get_mut(id as usize).and_then(|t| t.take());

        ticket.ok_or(LoginError::DeleteTicketFailIdNotFound { id })
    }
}
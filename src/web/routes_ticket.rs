use crate::model::{ModelController, Ticket, TicketForCreate};
use axum::{extract::{Path, State}, routing::{post, get, delete}, Json, Router};
use crate::Result;

// create ticket route function
async fn create_ticket(State(mc): State<ModelController>, Json(ticket_fc): Json<TicketForCreate>) -> Result<Json<Ticket>> {
    let ticket = mc.create_ticket(ticket_fc).await?;

    Ok(Json(ticket))
}

// get list of tickets 
async fn list_tickets(State(mc): State<ModelController>) -> Result<Json<Vec<Ticket>>> {
    let tickets = mc.ticket_list().await?;

    Ok(Json(tickets))
}

// delete ticket 
async fn delete_ticket(State(mc): State<ModelController>, Path(id): Path<u64>) -> Result<Json<Ticket>> {
    let deleted_ticket = mc.delete_ticket(id).await?;

    Ok(Json(deleted_ticket))
}



/// Routes ///

pub fn routes(mc: ModelController) -> Router {
    Router::new()
    .route("/tickets", post(create_ticket).get(list_tickets))
    .route("tickets/:id", delete(delete_ticket))
    .with_state(mc)
}
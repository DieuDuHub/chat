#[macro_use] extern crate rocket;
use rocket::tokio::sync::broadcast::{channel, Sender, error::RecvError};
use serde::{Serialize, Deserialize};
use rocket::form::Form;
use rocket::{State, Shutdown};
use rocket::response::stream::{EventStream, Event};
use rocket::tokio::select;
use ws::{WebSocket, Stream};
use async_stream::stream; // For creating asynchronous streams
use futures::stream::Stream; // For working with streams
use serde_json::json;
use uuid::Uuid; // Add this for UUID generation

// ARCHIMATE : RUST Events Server based on Rocket


// Temp message for http receiver
#[derive(Debug, Clone, FromForm, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct MessageForm {
    #[field(validate = len(..30))]
    pub room: String,
    #[field(validate = len(..20))]
    pub username: String,
    pub message: String,
}

#[derive(Debug, Clone, FromForm, Serialize, Deserialize)]
#[cfg_attr(test, derive(PartialEq, UriDisplayQuery))]
#[serde(crate = "rocket::serde")]
struct Message {
    pub id: String, // Unique identifier for the message
    #[field(validate = len(..30))]
    pub room: String,
    #[field(validate = len(..20))]
    pub username: String,
    pub message: String,
}

impl Message {
    // Constructor to create a new message with a generated id
    pub fn new(room: String, username: String, message: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(), // Generate a unique UUID
            room,
            username,
            message,
        }
    }
}

// Websocket clarification
#[get("/api/echo")]
fn echo_compose(ws: WebSocket) -> Stream!['static] {
    ws.stream(|io| io)
}


// Websocket clarification
#[get("/api/sse")]
async fn echo_stream(ws: WebSocket,queue: &State<Sender<Message>>, mut end: Shutdown) -> EventStream![] {
    let mut rx = queue.subscribe();
    // Create a stream that emits messages from the queue
    /*
    let stream = async_stream::stream! {
        loop {
            let msg = select! {
                msg = rx.recv() => match msg {
                    Ok(msg) => msg, // Received a message from the queue
                    Err(RecvError::Closed) => break, // Exit the loop if the queue is closed
                    Err(RecvError::Lagged(_)) => continue, // Skip lagged messages
                },
                _ = &mut end => break, // Exit the loop if the shutdown signal is received
            };
            yield msg; // Yield the message as a WebSocket text message
        }
    };
    */
     EventStream! {
        loop {
            let msg = select! {
                msg = rx.recv() => match msg {
                    Ok(msg) => msg,
                    Err(RecvError::Closed) => break,
                    Err(RecvError::Lagged(_)) => continue,
                },
                _ = &mut end => break,
            };
            yield Event::json(&msg);
        }
    }
   // ws.stream(|stream| stream)
}

/*
#[post("/api/message", data = "<form>")]
fn post(form: Form<MessageForm>, queue: &State<Sender<Message>>) {
    // A send 'fails' if there are no active subscribers. That's okay.
    let message = Message::new(
        form.room.clone(),
        form.username.clone(),
        form.message.clone(),
    );
    // A send 'fails' if there are no active subscribers. That's okay.
    let _res = queue.send(message);
}*/

#[post("/api/message", format = "json", data = "<message>")]
fn post(message: rocket::serde::json::Json<MessageForm>, queue: &State<Sender<Message>>) {
    let message = Message::new(
        message.room.clone(),
        message.username.clone(),
        message.message.clone(),
    );
    // A send 'fails' if there are no active subscribers. That's okay.
    let _res = queue.send(message);
}

#[get("/api/events")]
async fn events(queue: &State<Sender<Message>>, mut end: Shutdown) -> EventStream![] {
    let mut rx = queue.subscribe();
    EventStream! {
        loop {
            let msg = select! {
                msg = rx.recv() => match msg {
                    Ok(msg) => msg,
                    Err(RecvError::Closed) => break,
                    Err(RecvError::Lagged(_)) => continue,
                },
                _ = &mut end => break,
            };
            yield Event::json(&msg);
        }
    }
}

#[launch]
fn rocket() -> _ {
    
    rocket::build()
        .manage(channel::<Message>(1024).0)
        .mount("/", routes![post, events,echo_compose,echo_stream])
        //mount("/", FileServer::from(relative!("static")))
}
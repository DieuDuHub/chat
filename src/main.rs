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

#[derive(Debug, Clone, FromForm, Serialize, Deserialize)]
#[cfg_attr(test, derive(PartialEq, UriDisplayQuery))]
#[serde(crate = "rocket::serde")]
struct Message {
    #[field(validate = len(..30))]
    pub room: String,
    #[field(validate = len(..20))]
    pub username: String,
    pub message: String,
}

// Websocket clarification
#[get("/echo")]
fn echo_compose(ws: WebSocket) -> Stream!['static] {
    ws.stream(|io| io)
}


// Websocket clarification
#[get("/echostream")]
async fn echo_stream(ws: WebSocket,queue: &State<Sender<Message>>, mut end: Shutdown) -> Stream!['static]  {
    let mut rx = queue.subscribe();
    // Create a stream that emits messages from the queue
    let stream = async_stream::stream! {
        loop {
            let msg = tokio::select! {
                msg = rx.recv() => match msg {
                    Ok(msg) => msg, // Received a message from the queue
                    Err(RecvError::Closed) => break, // Exit the loop if the queue is closed
                    Err(RecvError::Lagged(_)) => continue, // Skip lagged messages
                },
                _ = &mut end => break, // Exit the loop if the shutdown signal is received
            };
            yield Message::Text(msg); // Yield the message as a WebSocket text message
        }
    };

    ws.stream(stream)
}


#[post("/message", data = "<form>")]
fn post(form: Form<Message>, queue: &State<Sender<Message>>) {
    // A send 'fails' if there are no active subscribers. That's okay.
    let _res = queue.send(form.into_inner());
}

#[get("/events")]
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
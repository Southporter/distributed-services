use crate::internal::log::Log;
use crate::internal::server::Message;
// use actix_web::{get, middleware::Logger, post, web, App, HttpResponse, HttpServer, Responder};
// use tokio::sync::{mpsc, oneshot, Mutex};

mod internal;

fn main() {
    println!("Main not callable");
}

// type ServerData = web::Data<Mutex<mpsc::Sender<Message>>>;

// #[get("/{offset}")]
// async fn read(web::Path(offset): web::Path<usize>, data: ServerData) -> impl Responder {
//     println!("Reading {}", offset);
//     let (sender, receiver) = oneshot::channel();
//     let mut messenger = data.lock().await;
//     let res = messenger.send(Message::Read(offset, sender)).await;
//     match res {
//         Ok(_) => match receiver.await {
//             Ok(value) => HttpResponse::Ok().body(value),
//             Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
//         },
//         Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
//     }
// }

// #[post("/")]
// async fn append(req_body: web::Bytes, data: ServerData) -> impl Responder {
//     println!("Appending");
//     let (sender, receiver) = oneshot::channel();
//     let mut messenger = data.lock().await;
//     let res = messenger.send(Message::Append(req_body, sender)).await;
//     match res {
//         Ok(_) => match receiver.await {
//             Ok(offset) => HttpResponse::Ok().body(format!("{}", offset)),
//             Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
//         },
//         Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
//     }
// }

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     let (sender, mut receiver) = mpsc::channel(10);
//     tokio::spawn(async move {
//         let mut log = Log::new();
//         while let Some(message) = receiver.recv().await {
//             match message {
//                 Message::Append(record, sender) => {
//                     let offset = log.append(record).await;
//                     sender.send(offset).expect("Chould not send response");
//                 }
//                 Message::Read(offset, sender) => match log.read(offset).await {
//                     Some(record) => {
//                         sender
//                             .send(record.value.clone())
//                             .expect("Could not send response");
//                     }
//                     None => {
//                         sender
//                             .send(format!("No record found for offset {}", offset).into())
//                             .expect("Could not send response");
//                     }
//                 },
//             }
//         }
//     });

//     pretty_env_logger::init();

//     let data = web::Data::new(Mutex::new(sender));

//     HttpServer::new(move || {
//         App::new()
//             .app_data(data.clone())
//             .service(read)
//             .service(append)
//             .wrap(Logger::new("%a %{User-Agent}i"))
//     })
//     .bind("127.0.0.1:3030")?
//     .run()
//     .await
// }

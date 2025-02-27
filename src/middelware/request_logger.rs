use rocket::{fairing::{Fairing, Info, Kind}, Request, Data};
use tracing::{info, error};

pub struct RequestLogger;

#[rocket::async_trait]
impl Fairing for RequestLogger {
    fn info(&self) -> Info {
        Info {
            name: "Request Logger",
            kind: Kind::Request | Kind::Response
        }
    }

    async fn on_request(&self, request: &mut Request<'_>, _: &mut Data<'_>) {
        info!("Incoming request: {} {}", request.method(), request.uri());
    }

    async fn on_response<'r>(&self, req: &'r Request<'_>, res : &mut rocket::Response<'r>) {
       let status = res.status();
       if status.code >= 400 {
              error!("Failed request: {} {} - {}", req.method(), req.uri(), status.code);
         } else {
              info!("Outgoing response: {} {} - {}", req.method(), req.uri(), status.code);
       }
    }
}

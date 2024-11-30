use actix_web::{App, web, HttpServer, HttpResponse, Result};
use actix_files::{NamedFile};


async fn handler() -> Result<NamedFile>{
        Ok(NamedFile::open("index.html")?)
}


#[actix_web::main]
async fn main() -> std::io::Result<()>{
    println!("Starting server on port 8080..");        

 HttpServer::new(|| {
   App::new()       //This creates a new app instance that serve routes and services
       .route("/", web::get().to(handler))  //web::get() is the Http get method
    }).bind("127.0.0.1:8080")?.run().await
}

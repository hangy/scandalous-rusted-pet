use rweb::*;
use serde::{Serialize, Deserialize};

#[get("/output")]
fn output() -> String {
    String::from("this returns 200 with text/plain mime type")
}

#[derive(Debug, Serialize, Deserialize, Schema)]
struct Product {
    id: String,
    title: String,
}

#[get("/products")]
fn products() -> &'static str {
    "Hello, World!"
}

#[tokio::main]
async fn main() {
    let routes = rweb::get().and(
        products()
    );

    rweb::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
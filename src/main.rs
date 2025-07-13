pub mod routes;
use std::sync::{Arc, Mutex};

use axum::{
    Router,
    extract::State,
    response::{Html, IntoResponse},
    routing::get,
};
use routes::{METHODS, RouteHandler, Routes};

#[derive(Clone, Debug)]
pub struct AppState {
    html: Arc<Mutex<&'static str>>,
}

async fn main_page_html(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let html_file = tokio::fs::read_to_string("src/webpage/main.html")
        .await
        .unwrap();
    // works for prod but we are in dev ts
    // let html_file = include_str!("./webpage/main.html");
    Html(html_file)
}

fn initialize_routes() -> Option<Vec<Routes>> {
    let frontend_route = Routes {
        route: "/",
        method: METHODS::Get,
        func: get(main_page_html),
    };

    let route_vec = vec![frontend_route];
    return Some(route_vec);
}

#[tokio::main]
async fn main() {
    let state = Arc::new(AppState {
        html: Arc::new(Mutex::new("")),
    });

    let routes: Option<Vec<Routes>> = initialize_routes();
    let app = Router::new().with_state(Arc::clone(&state));

    let handler = RouteHandler {
        routes: routes,
        router: app,
        state: Arc::clone(&state),
    };

    let val = handler.define_routes().unwrap().init_listener().await;

    print!("{:?}", val);
}

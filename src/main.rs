pub mod checks;
pub mod routes;
use std::sync::{Arc, Mutex};

use axum::{
    Router,
    extract::State,
    response::{Html, IntoResponse},
    routing::get,
};
use routes::{METHODS, RouteHandler, Routes};

use crate::checks::{expand_tilde, first_start, vars::CONFIG_PATH};

#[derive(Clone, Debug)]
pub struct AppState {
    html: Arc<Mutex<&'static str>>,
}

async fn main_page_html(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    //TODO: make it so that the main.html is grabbed from the vars in  a semantic way, I did it
    //like this to avoid needing to do FILES[0] as it would be more annoying to understand, will
    //rework
    let base_path = expand_tilde(CONFIG_PATH);
    let html_path = base_path.join("UI").join("main.html");

    let html_file = tokio::fs::read_to_string(html_path).await.unwrap();

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
    first_start();

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

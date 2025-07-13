use std::sync::Arc;

use axum::{
    Router, ServiceExt,
    routing::{MethodRouter, get, post},
};

use crate::AppState;

#[derive(Debug)]
pub enum METHODS {
    Get,
    Post,
}

#[derive(Debug)]
pub struct Routes {
    pub route: &'static str,
    pub method: METHODS,
    pub func: MethodRouter<Arc<AppState>>,
}

#[derive(Debug)]
pub struct RouteHandler {
    pub router: Router<Arc<AppState>>,
    pub routes: Option<Vec<Routes>>,
    pub state: Arc<AppState>,
}

impl RouteHandler {
    pub fn new(&mut self, app_state: Arc<AppState>) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            router: self.router.clone(),
            routes: None,
            state: app_state,
        })
    }

    pub fn define_routes(mut self) -> Result<Self, Box<dyn std::error::Error>> {
        let Some(routes) = &self.routes else {
            return Err("No Routes defined".into());
        };

        let mut router = self.router.clone();

        for route in routes {
            router = router.route(route.route, route.func.clone());
        }

        self.router = router;
        Ok(self)
    }

    pub async fn init_listener(self) -> () {
        let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
        axum::serve(
            listener,
            self.router.with_state(self.state).into_make_service(),
        )
        .await
        .unwrap();
    }
}

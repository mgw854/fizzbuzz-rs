mod game;

use crate::game::fizzbuzz::FizzBuzz;
use warp::{http, Reply};

pub async fn run() {
    warp::serve(routes::get_routes(1))
        .run(([0, 0, 0, 0], 9001))
        .await
}

async fn always_ok() -> Result<impl warp::Reply, warp::Rejection> {
    Ok(warp::reply::with_status("OK", http::StatusCode::OK).into_response())
}

async fn get_fizzbuzz(current_state: u64) -> Result<impl warp::Reply, warp::Rejection> {
    let val = FizzBuzz::from(current_state);

    Ok(warp::reply::with_status(val.to_string(), http::StatusCode::OK).into_response())
}
pub mod routes {
    use warp::Filter;

    pub fn get_routes(
        current_state: u64,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        let state_filter = warp::any().map(move || current_state);

        let ready = warp::get()
            .and(warp::path("ready"))
            .and_then(super::always_ok);

        let current = warp::get()
            .and(warp::path("fizzbuzz"))
            .and(state_filter)
            .and_then(super::get_fizzbuzz);

        ready.or(current)
    }
}
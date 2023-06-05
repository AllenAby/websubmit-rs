use rocket::http::Status;
use rocket::outcome::IntoOutcome;
use rocket::outcome::Outcome::{Failure, Forward, Success};
use rocket::request::{self, FromRequest, Request};
use rocket::Route;

// TODO(artem): make sure the context is non-manufacturable (clone user on write?)

#[derive(Debug)]
pub struct Context<'a, U> {
    user: Option<U>,
    route: &'a Route,
}

impl<'a, U> Context<'a, U> {
    pub fn get_user(&self) -> Option<&U> {
        match &self.user {
            None => { None }
            Some(user) => { Some(user) }
        }
    }

    pub fn get_route(&self) -> &Route {
        self.route
    }
}

#[derive(Debug)]
pub enum ContextError {
    Unconstructible,
}

#[rocket::async_trait]
impl<'r, U: FromRequest<'r>> FromRequest<'r> for Context<'r, U> {
    type Error = ContextError;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let apikey: Option<U> = match request.guard::<U>().await {
            Success(apikey) => { Some(apikey) }
            Failure(_) => { None }
            Forward(_) => { None }
        };

        request.route()
            .and_then(|route| Some(Context { user: apikey, route }))
            .into_outcome((Status::InternalServerError, ContextError::Unconstructible))
    }
}

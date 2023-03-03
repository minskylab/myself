use std::{borrow::BorrowMut, convert::Infallible};

use async_graphql::{
    http::GraphiQLSource, Context, EmptyMutation, EmptySubscription, Object, Schema,
};
use async_graphql_warp::{GraphQLBadRequest, GraphQLResponse};
use http::StatusCode;
use myself::agent::Agent;
use warp::{http::Response as HttpResponse, Filter, Rejection};

struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn interact_by_default<'a>(&self, ctx: &Context<'a>, message: String) -> String {
        let mut agent = ctx.data::<Agent>().unwrap().to_owned();
        // let agent = agent.borrow_mut();

        // agent.my_name = "Alice".to_string();

        // agent.interact_with_default(message).await
        "".into()
    }
}

#[tokio::main]
async fn main() {
    let agent = Agent::new_with_defaults("Bob".to_string()).await;

    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
        .data(agent)
        .finish();

    println!("GraphiQL IDE: http://localhost:8000");

    let graphql_post = async_graphql_warp::graphql(schema).and_then(
        |(schema, request): (
            Schema<QueryRoot, EmptyMutation, EmptySubscription>,
            async_graphql::Request,
        )| async move {
            Ok::<_, Infallible>(GraphQLResponse::from(schema.execute(request).await))
        },
    );

    let graphiql = warp::path::end().and(warp::get()).map(|| {
        HttpResponse::builder()
            .header("content-type", "text/html")
            .body(GraphiQLSource::build().endpoint("/").finish())
    });

    let routes = graphiql
        .or(graphql_post)
        .recover(|err: Rejection| async move {
            if let Some(GraphQLBadRequest(err)) = err.find() {
                return Ok::<_, Infallible>(warp::reply::with_status(
                    err.to_string(),
                    StatusCode::BAD_REQUEST,
                ));
            }

            Ok(warp::reply::with_status(
                "INTERNAL_SERVER_ERROR".to_string(),
                StatusCode::INTERNAL_SERVER_ERROR,
            ))
        });

    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}

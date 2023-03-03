use std::convert::Infallible;

use async_graphql::{
    http::GraphiQLSource, Context, EmptyMutation, EmptySubscription, Object, Schema, SimpleObject,
};
use async_graphql_warp::{GraphQLBadRequest, GraphQLResponse};
use http::StatusCode;
use myself::agent::Agent;
use rbdc::uuid::Uuid;
use warp::{http::Response as HttpResponse, Filter, Rejection};

struct QueryRoot;

#[derive(SimpleObject)]
struct Interaction {
    id: String,
    user_name: String,
    constitution: String,
    memory_buffer: String,
}

#[Object]
impl QueryRoot {
    async fn interact_with_default<'a>(&self, ctx: &Context<'a>, message: String) -> String {
        let mut agent = ctx.data::<Agent>().unwrap().to_owned();
        agent.interact_with_default(message).await
    }

    async fn interact_with<'a>(&self, ctx: &Context<'a>, id: String, message: String) -> String {
        let mut agent = ctx.data::<Agent>().unwrap().to_owned();
        agent.interact_with(Uuid(id), message).await
    }

    async fn interactions<'a>(&self, ctx: &Context<'a>) -> Vec<Interaction> {
        let mut agent = ctx.data::<Agent>().unwrap().to_owned();
        agent
            .get_all_interactions()
            .await
            .iter()
            .map(|i| Interaction {
                id: i.id.to_string(),
                user_name: i.user_name.to_owned(),
                constitution: i.template_memory.to_owned(),
                memory_buffer: i.dynamic_memory.to_owned().unwrap_or("".into()),
            })
            .collect()
    }

    async fn interaction<'a>(&self, ctx: &Context<'a>, id: String) -> Option<Interaction> {
        let mut agent = ctx.data::<Agent>().unwrap().to_owned();
        match agent.get_interaction(Uuid(id)).await {
            Some(i) => Some(Interaction {
                id: i.id.to_string(),
                user_name: i.user_name.to_owned(),
                constitution: i.template_memory.to_owned(),
                memory_buffer: i.dynamic_memory.to_owned().unwrap_or("".into()),
            }),
            None => None,
        }
    }
}

#[tokio::main]
async fn main() {
    let agent = Agent::new_with_defaults("AI".to_string()).await;

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

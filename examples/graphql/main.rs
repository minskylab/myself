use std::convert::Infallible;

use async_graphql::{
    http::GraphiQLSource, Context, EmptySubscription, Object, Schema, SimpleObject,
};
use async_graphql_warp::{GraphQLBadRequest, GraphQLResponse};
use dotenvy::dotenv;
use http::StatusCode;
use myself::database::Interaction as DBInteraction;
use myself::{agent::Agent, agent_builder::AgentBuilder};
use rbdc::uuid::Uuid;
use warp::{http::Response as HttpResponse, Filter, Rejection};

struct QueryRoot;
struct MutationRoot;

#[derive(SimpleObject)]
struct Interaction {
    id: String,
    user_name: String,
    constitution: String,
    memory_buffer: String,
}

#[derive(SimpleObject)]
struct InteractionResponse {
    response: String,
    interaction: Interaction,
}

impl Interaction {
    fn parse(db_interaction: &DBInteraction) -> Self {
        Self {
            id: db_interaction.id.0.to_owned(),
            user_name: db_interaction.user_name.to_owned(),
            constitution: db_interaction.long_term_memory.to_owned(),
            memory_buffer: db_interaction
                .short_term_memory
                .to_owned()
                .unwrap_or("".into()),
        }
    }
}

#[Object]
impl QueryRoot {
    async fn interactions<'a>(&self, ctx: &Context<'a>) -> Vec<Interaction> {
        let mut agent = ctx.data::<Agent>().unwrap().to_owned();
        agent
            .get_all_interactions()
            .await
            .iter()
            .map(|i| Interaction::parse(i))
            .collect()
    }

    async fn interaction<'a>(&self, ctx: &Context<'a>, id: String) -> Option<Interaction> {
        let mut agent = ctx.data::<Agent>().unwrap().to_owned();
        match agent.get_interaction(Uuid(id)).await {
            Some(i) => Some(Interaction::parse(&i)),
            None => None,
        }
    }
}

#[Object]
impl MutationRoot {
    async fn new_interaction<'a>(
        &self,
        ctx: &Context<'a>,
        user_name: String,
        constitution: String,
        memory_size: usize,
    ) -> Interaction {
        let mut agent = ctx.data::<Agent>().unwrap().to_owned();
        let interaction = agent
            .init_interaction(user_name, constitution, memory_size)
            .await;

        Interaction::parse(&interaction)
    }

    async fn interact_with_default<'a>(
        &self,
        ctx: &Context<'a>,
        message: String,
    ) -> InteractionResponse {
        let mut agent = ctx.data::<Agent>().unwrap().to_owned();

        InteractionResponse {
            response: agent.interact_with_default(&message).await.unwrap(),
            interaction: Interaction::parse(&agent.get_default_interaction().await),
        }
    }

    async fn interact_with<'a>(
        &self,
        ctx: &Context<'a>,
        id: String,
        message: String,
    ) -> InteractionResponse {
        let mut agent = ctx.data::<Agent>().unwrap().to_owned();
        let uuid = Uuid(id);

        InteractionResponse {
            // TODO: Improve memory management
            response: agent.interact_with(uuid.clone(), &message).await.unwrap(),
            interaction: Interaction::parse(&agent.get_interaction(uuid).await.unwrap()),
        }
    }

    async fn update_constitution<'a>(
        &self,
        ctx: &Context<'a>,
        id: String,
        constitution: String,
    ) -> Interaction {
        let mut agent = ctx.data::<Agent>().unwrap().to_owned();
        let interaction = agent.update_constitution(Uuid(id), constitution).await;

        Interaction::parse(&interaction)
    }

    async fn forget_memory<'a>(&self, ctx: &Context<'a>, id: String) -> Interaction {
        let mut agent = ctx.data::<Agent>().unwrap().to_owned();
        let interaction = agent.forget(Uuid(id)).await;

        Interaction::parse(&interaction.unwrap())
    }
}

#[tokio::main]
async fn main() {
    // Don't forget to create a .env file with the following content:
    // OPENAI_API_KEY=your_api_key

    dotenv().ok();

    let agent = AgentBuilder::new()
        .with_name("AI".to_string())
        .build()
        .await;

    let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(agent)
        .finish();

    println!("GraphiQL IDE: http://localhost:8000");

    let graphql_post = async_graphql_warp::graphql(schema).and_then(
        |(schema, request): (
            Schema<QueryRoot, MutationRoot, EmptySubscription>,
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

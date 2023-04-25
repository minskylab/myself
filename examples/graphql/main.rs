use std::convert::Infallible;
use std::marker::PhantomData;
use std::str::FromStr;

use async_graphql::{
    http::GraphiQLSource, Context, EmptySubscription, Object, Schema, SimpleObject,
};
use async_graphql_warp::{GraphQLBadRequest, GraphQLResponse};
use dotenvy::dotenv;
use http::StatusCode;
use myself::backend::{AgentBackend, OpenAIBackend};
use myself::database::memory::MemoryEngine;
use myself::sdk::interactions::{Interaction as DBInteraction, InteractionState};
use myself::{agent::Agent, agent_builder::AgentBuilder};
use std::env::var;
use uuid::Uuid;
use warp::{http::Response as HttpResponse, Filter, Rejection};
struct QueryRoot<Backend>
where
    Backend: AgentBackend + Sized + Default + Clone,
{
    _backend: PhantomData<Backend>,
}
struct MutationRoot<Backend>
where
    Backend: AgentBackend + Sized + Default + Clone,
{
    _backend: PhantomData<Backend>,
}

#[derive(SimpleObject)]
struct Interaction {
    id: String,
    user_name: String,
    // long_term_memory: String,
    short_term_memory: String,
}

#[derive(SimpleObject)]
struct InteractionResponse {
    response: String,
    interaction: Interaction,
}

impl Interaction {
    fn parse<Backend, State>(db_interaction: &DBInteraction<Backend, State>) -> Self
    where
        Backend: AgentBackend + Sized + Default + Clone,
        State: InteractionState,
    {
        Self {
            id: db_interaction.id.to_string(),
            user_name: db_interaction.user_name.to_owned(),
            // long_term_memory: db_interaction.long_term_memory.to_owned(),
            short_term_memory: db_interaction.short_term_memory.to_owned(),
        }
    }
}

#[Object]
impl<Backend> QueryRoot<Backend>
where
    Backend: AgentBackend + Sized + Default + Clone + Send + Sync + 'static,
{
    async fn interactions<'a>(&self, ctx: &Context<'a>) -> Vec<Interaction>
where {
        let mut agent = ctx.data::<Agent<Backend>>().unwrap().to_owned();
        agent
            .get_all_interactions()
            .await
            .iter()
            .map(Interaction::parse)
            .collect()
    }

    async fn interaction<'a>(&self, ctx: &Context<'a>, id: String) -> Option<Interaction> {
        let mut agent = ctx.data::<Agent<Backend>>().unwrap().to_owned();
        agent
            .get_interaction(Uuid::from_str(id.as_str()).unwrap())
            .await
            .map(|i| Interaction::parse(&i))
    }
}

#[Object]
impl<Backend> MutationRoot<Backend>
where
    Backend: AgentBackend + Sized + Default + Clone + Send + Sync + 'static,
{
    async fn new_interaction<'a>(
        &self,
        ctx: &Context<'a>,
        user_name: String,
        constitution: String,
        memory_size: usize,
    ) -> Interaction {
        let mut agent = ctx.data::<Agent<Backend>>().unwrap().to_owned();
        let interaction = agent
            .init_interaction(user_name, constitution, memory_size)
            .await;

        Interaction::parse(&interaction)
    }

    async fn interact_default<'a>(
        &self,
        ctx: &Context<'a>,
        message: String,
    ) -> InteractionResponse {
        let mut agent = ctx.data::<Agent<Backend>>().unwrap().to_owned();

        InteractionResponse {
            response: agent.interact_default(&message).await.unwrap(),
            interaction: Interaction::parse(&agent.get_default_interaction().await),
        }
    }

    async fn interact<'a>(
        &self,
        ctx: &Context<'a>,
        id: String,
        message: String,
    ) -> InteractionResponse {
        let mut agent = ctx.data::<Agent<Backend>>().unwrap().to_owned();
        let uuid = Uuid::from_str(id.as_str()).unwrap();

        InteractionResponse {
            // TODO: Improve memory management
            response: agent.interact(uuid, &message).await.unwrap(),
            interaction: Interaction::parse(&agent.get_interaction(uuid).await.unwrap()),
        }
    }

    async fn update_constitution<'a>(
        &self,
        ctx: &Context<'a>,
        id: String,
        constitution: String,
    ) -> Interaction {
        let mut agent = ctx.data::<Agent<Backend>>().unwrap().to_owned();
        let interaction = agent
            .update_long_term_memory(Uuid::from_str(id.as_str()).unwrap(), constitution)
            .await;

        Interaction::parse(&interaction)
    }

    async fn forget_memory<'a>(&self, ctx: &Context<'a>, id: String) -> Interaction {
        let mut agent = ctx.data::<Agent<Backend>>().unwrap().to_owned();
        let interaction = agent
            .forgot_short_term_memory(Uuid::from_str(id.as_str()).unwrap())
            .await;

        Interaction::parse(&interaction.unwrap())
    }
}

#[tokio::main]
async fn main() {
    // Don't forget to create a .env file with the following content:
    // OPENAI_API_KEY=your_api_key

    dotenv().ok();

    let llm_engine = OpenAIBackend::new(var("OPENAI_API_KEY").unwrap());
    let memory_engine = MemoryEngine::new(var("DATABASE_URL").unwrap()).await;

    let agent = AgentBuilder::new()
        .name("AI".to_string())
        .build(llm_engine, memory_engine)
        .await;

    let schema = Schema::build(
        QueryRoot {
            _backend: PhantomData,
        },
        MutationRoot {
            _backend: PhantomData,
        },
        EmptySubscription,
    )
    .data(agent)
    .finish();

    let graphql_post = async_graphql_warp::graphql(schema).and_then(
        |(schema, request): (
            Schema<QueryRoot<OpenAIBackend>, MutationRoot<OpenAIBackend>, EmptySubscription>,
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

    println!("GraphiQL IDE: http://localhost:8000");

    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}

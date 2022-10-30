use async_graphql::{Context, FieldResult, Object, Schema, EmptySubscription};
use crate::config::surrealdb::SurrealDB;
use crate::schemas::project_schemas::{Owner, FetchOwner, FetchProject, Project, CreateOwner, CreateProject};
pub struct Query;

#[Object(extends)]
impl Query {
    async fn owner(&self, ctx: &Context<'_>, input: FetchOwner) -> FieldResult<Owner> {
        let db = &ctx.data_unchecked::<SurrealDB>();
        
        let owner = db.get("owner", &input._id).await?;
        
        let owner = Owner::from(owner);
        
        Ok(owner)
    }

    async fn get_owners(&self, ctx: &Context<'_>) -> FieldResult<Vec<Owner>> {
        let db = &ctx.data_unchecked::<SurrealDB>();
        
        let owners = db.get_all("owner").await?;

        let owners = owners.into_iter().map(|v| Owner::from(v)).collect();

        Ok(owners)
    }

    async fn project(&self, ctx: &Context<'_>, input: FetchProject) -> FieldResult<Project> {
        let db = &ctx.data_unchecked::<SurrealDB>();
        
        let project = db.get("project", &input._id).await?;
        
        let project = Project::from(project);
        
        Ok(project)
    }

    async fn get_projects(&self, ctx: &Context<'_>) -> FieldResult<Vec<Project>> {
        let db = &ctx.data_unchecked::<SurrealDB>();
        
        let projects = db.get_all("project").await?;

        let projects = projects.into_iter().map(|v| Project::from(v)).collect();

        Ok(projects)
    }
}

pub struct Mutation;

#[Object]
impl Mutation {
    async fn create_owner(&self, ctx: &Context<'_>, input: CreateOwner) -> FieldResult<Owner> {
        let db = &ctx.data_unchecked::<SurrealDB>();

        let new_owner = CreateOwner {
            email: input.email,
            name: input.name,
            phone: input.phone,
        };

        let owner = db.create("owner", new_owner).await.unwrap();

        Ok(Owner::from(owner))
    }
    async fn create_project(&self, ctx: &Context<'_>, input: CreateProject) -> FieldResult<Project> {
        let db = &ctx.data_unchecked::<SurrealDB>();

        let new_project = CreateProject {
            owner_id: input.owner_id,
            name: input.name,
            description: input.description,
            status: input.status,
        };

        let project = db.create("project", new_project).await?;

        Ok(Project::from(project))
    }
}

pub type ProjectSchema = Schema<Query, Mutation, EmptySubscription>;
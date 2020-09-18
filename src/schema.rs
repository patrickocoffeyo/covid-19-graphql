use juniper::FieldResult;
use juniper::GraphQLObject;
use juniper::RootNode;

#[derive(GraphQLObject)]
#[graphql(description = "Object representing a country")]
struct Country {
    country: String,
    slug: String,
    iso2: String,
}

pub struct QueryRoot;

#[juniper::object]
impl QueryRoot {
    fn country(id: String) -> FieldResult<Country> {
        Ok(Country {
            country: "Afghanistan".to_string(),
            slug: "afghanistan".to_string(),
            iso2: "AF".to_string(),
        })
    }
}

pub type Schema = RootNode<'static, QueryRoot, QueryRoot>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, QueryRoot {})
}

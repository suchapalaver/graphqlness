use async_graphql::*;

struct Query;

#[Object]
impl Query {
    /// Returns the sum of a and b
    async fn add(&self, a: i32, b: i32) -> i32 {
        a + b
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
pub mod test {
    use crate::Query;
    use async_graphql::*;

    #[tokio::test]
    async fn test_query() {
        let schema = Schema::new(Query, EmptyMutation, EmptySubscription);
        let res = schema.execute("{ add(a: 444, b: 222) }").await;
        let json = serde_json::to_string(&res).unwrap();
        insta::assert_snapshot!(json, @r###"{"data":{"add":666}}"###);
    }
}

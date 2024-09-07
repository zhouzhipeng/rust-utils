pub mod data;

#[cfg(test)]
mod tests {
    use std::thread;
    use serde::{Deserialize, Serialize};
    use crate::data::IData;
    use super::*;

    #[derive(Serialize, Deserialize, Debug, Clone)]
    struct Demo{
        id : i32,
        name: String,
    }

    struct DemoAPI;

    impl IData for DemoAPI{
        type Model = Demo;

        fn get_host(&self) -> &'static str {
            "http://127.0.0.1:9000"
        }

        fn get_category(&self) -> &'static str {
            "demo"
        }
    }

    #[tokio::test]
    async fn test_insert()->anyhow::Result<()> {
        let api = DemoAPI;
        let r = api.insert(&Demo{ id: 456, name: "demo name222".to_string() } ).await?;
        println!("{:?}", r);

        Ok(())
    }
    #[tokio::test]
    async fn test_delete()->anyhow::Result<()> {
        let api = DemoAPI;
        let r = api.delete(502).await?;
        println!("{:?}", r);

        Ok(())
    }
    #[tokio::test]
    async fn test_update_full()->anyhow::Result<()> {
        let api = DemoAPI;
        let r = api.update_full(501, &Demo{ id: 1, name: "501 name".to_string() }).await?;
        println!("{:?}", r);

        Ok(())
    }
    #[tokio::test]
    async fn test_update_field()->anyhow::Result<()> {
        let api = DemoAPI;
        let r = api.update_field(501, ("name2", "502 name")).await?;
        println!("{:?}", r);

        Ok(())
    }
    #[tokio::test]
    async fn test_query_by_id()->anyhow::Result<()> {
        let api = DemoAPI;
        let r = api.get(500).await?;
        println!("{:?}", r);

        Ok(())
    }
    #[tokio::test]
    async fn test_query_by_category()->anyhow::Result<()> {
        let api = DemoAPI;
        let r = api.list(100).await?;
        println!("{:?}", r);

        Ok(())
    }
}

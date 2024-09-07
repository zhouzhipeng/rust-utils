use anyhow::{anyhow, bail, ensure, Context};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use serde::de::DeserializeOwned;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct RawData{
    id : i64,
    cat: String,
    data: String,
    created: i64,
    updated: i64,
}



pub trait IData {
    type Model: Serialize+for<'de>  Deserialize<'de>;

    fn get_host(&self) -> &'static str;


    /// under which category your data should put in. like a table name.
    fn get_category(&self) -> &'static str;

    fn get_client(&self) -> anyhow::Result<Client> {
        Ok(Client::builder().timeout(Duration::from_secs(3)).build().context("failed to build client")?)
    }

    async fn insert(&self, data: &Self::Model) -> anyhow::Result<RawData> {
        ensure!(!self.get_category().is_empty());
        let client = self.get_client()?;

        let response = client.post(format!("{}/data/cat/{}", self.get_host(), self.get_category())).json(data).send().await?;
        if response.status().is_success(){
            let r: Vec<RawData> = response.json().await?;
            Ok(r[0].clone())
        }else{
            bail!(response.text().await?)
        }

    }
    async fn delete(&self, id: i64) -> anyhow::Result<RawData> {
        ensure!(!self.get_category().is_empty());
        let client = self.get_client()?;

        let response = client.delete(format!("{}/data/id/{}", self.get_host(), id)).send().await?;
        if response.status().is_success(){
            let r: Vec<RawData> = response.json().await?;
            Ok(r[0].clone())
        }else{
            bail!(response.text().await?)
        }

    }
    async fn update_full(&self, id: i64, data: &Self::Model) -> anyhow::Result<RawData> {
        ensure!(!self.get_category().is_empty());
        let client = self.get_client()?;

        let response = client.put(format!("{}/data/id/{}", self.get_host(), id))
            .json(data)
            .send().await?;
        if response.status().is_success(){
            let r: Vec<RawData> = response.json().await?;
            Ok(r[0].clone())
        }else{
            bail!(response.text().await?)
        }
    }
    async fn update_field(&self, id: i64, field_param: (&str,&str)) -> anyhow::Result<RawData> {
        ensure!(!self.get_category().is_empty());
        let client = self.get_client()?;

        let response = client.patch(format!("{}/data/id/{}", self.get_host(), id))
            .query(&[field_param])
            .send().await?;
        if response.status().is_success(){
            let r: Vec<RawData> = response.json().await?;
            Ok(r[0].clone())
        }else{
            bail!(response.text().await?)
        }
    }
    async fn query_by_id(&self, id: i64) -> anyhow::Result<Vec<Self::Model>> {
        ensure!(!self.get_category().is_empty());
        let client = self.get_client()?;

        let response = client.get(format!("{}/data/id/{}", self.get_host(), id))
            .send().await?;
        if response.status().is_success(){
            let r: Vec<RawData> = response.json().await?;

            let data_list = r.iter().map(|m| serde_json::from_str::<Self::Model>(&m.data).unwrap()).collect::<Vec<Self::Model>>();

            Ok(data_list)

        }else{
            bail!(response.text().await?)
        }
    }
    async fn query_by_category(&self, category: &str, limit: u32) -> anyhow::Result<Vec<Self::Model>> {
        ensure!(!self.get_category().is_empty());
        let client = self.get_client()?;

        let response = client.get(format!("{}/data/cat/{}", self.get_host(), category))
            .query(&[("_limit", limit)])
            .send().await?;
        if response.status().is_success(){
            let r: Vec<RawData> = response.json().await?;

            let data_list = r.iter().map(|m| serde_json::from_str::<Self::Model>(&m.data).unwrap()).collect::<Vec<Self::Model>>();

            Ok(data_list)

        }else{
            bail!(response.text().await?)
        }
    }
}

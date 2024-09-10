use anyhow::{anyhow, bail, ensure, Context};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct RawData{
    pub id : i64,
    pub cat: String,
    pub data: String,
    pub created: i64,
    pub updated: i64,
}


impl RawData {
    pub fn to<T:for<'de>  Deserialize<'de> >(&self)-> anyhow::Result<T>{
        serde_json::from_str::<T>(&self.data).context(anyhow!("failed to parse data"))
    }
}


pub trait IData {
    type Model: Serialize+for<'de>  Deserialize<'de>+Clone;

    fn get_host() -> &'static str;


    /// under which category your data should put in. like a table name.
    fn get_category() -> &'static str;


    /// used to authenticate
    fn get_auth_key() -> &'static str{""}

    fn get_client() -> anyhow::Result<Client> {
        let mut builder = Client::builder();
        #[cfg(not(target_arch = "wasm32"))]
        {
            builder = builder.timeout(Duration::from_secs(3));
        }

        Ok(builder
            .build().context("failed to build client")?)
    }

    async fn insert(data: &Self::Model) -> anyhow::Result<RawData> {
        ensure!(!Self::get_category().is_empty());
        let client = Self::get_client()?;

        let response = client.post(format!("{}/data/cat/{}", Self::get_host(), Self::get_category()))
            .header("X-Browser-Fingerprint", Self::get_auth_key())
            .json(data).send().await?;
        if response.status().is_success(){
            let r: Vec<RawData> = response.json().await?;
            Ok(r[0].clone())
        }else{
            bail!(response.text().await?)
        }

    }
    async fn delete( id: i64) -> anyhow::Result<RawData> {
        ensure!(!Self::get_category().is_empty());
        let client = Self::get_client()?;

        let response = client.delete(format!("{}/data/id/{}", Self::get_host(), id))
            .header("X-Browser-Fingerprint", Self::get_auth_key()).send().await?;
        if response.status().is_success(){
            let r: Vec<RawData> = response.json().await?;
            Ok(r[0].clone())
        }else{
            bail!(response.text().await?)
        }

    }
    async fn update_full( id: i64, data: &Self::Model) -> anyhow::Result<RawData> {
        ensure!(!Self::get_category().is_empty());
        let client = Self::get_client()?;

        let response = client.put(format!("{}/data/id/{}", Self::get_host(), id))
            .header("X-Browser-Fingerprint", Self::get_auth_key())
            .json(data)
            .send().await?;
        if response.status().is_success(){
            let r: Vec<RawData> = response.json().await?;
            Ok(r[0].clone())
        }else{
            bail!(response.text().await?)
        }
    }
    async fn update_field( id: i64, field_param: (&str,&str)) -> anyhow::Result<RawData> {
        ensure!(!Self::get_category().is_empty());
        let client = Self::get_client()?;

        let response = client.patch(format!("{}/data/id/{}", Self::get_host(), id))
            .query(&[field_param])
            .header("X-Browser-Fingerprint", Self::get_auth_key())
            .send().await?;
        if response.status().is_success(){
            let r: Vec<RawData> = response.json().await?;
            Ok(r[0].clone())
        }else{
            bail!(response.text().await?)
        }
    }
    async fn get( id: i64) -> anyhow::Result<Self::Model> {
        ensure!(!Self::get_category().is_empty());
        let client = Self::get_client()?;

        let response = client.get(format!("{}/data/cat/{}/id/{}", Self::get_host(), Self::get_category(), id))
            .header("X-Browser-Fingerprint", Self::get_auth_key())
            .send().await?;
        if response.status().is_success(){
            let r: Vec<RawData> = response.json().await?;

            let data_list = r.iter().map(|m| serde_json::from_str::<Self::Model>(&m.data).unwrap()).collect::<Vec<Self::Model>>();
            if data_list.is_empty(){
                bail!("id : {} not found!" , id)
            }
            Ok(data_list[0].clone())

        }else{
            bail!(response.text().await?)
        }
    }
    async fn list( limit: u32) -> anyhow::Result<Vec<Self::Model>> {
        let raw_data = Self::list_raw(limit).await?;
        Ok(raw_data.iter().map(|m|m.to::<Self::Model>().unwrap()).collect())
    }
    async fn list_raw(limit: u32) -> anyhow::Result<Vec<RawData>> {
        ensure!(!Self::get_category().is_empty());
        let client = Self::get_client()?;

        let response = client.get(format!("{}/data/cat/{}", Self::get_host(), Self::get_category()))
            .query(&[("_limit", limit)])
            .header("X-Browser-Fingerprint", Self::get_auth_key())
            .send().await?;
        if response.status().is_success(){
            let r: Vec<RawData> = response.json().await?;
            Ok(r)

        }else{
            bail!(response.text().await?)
        }
    }
}

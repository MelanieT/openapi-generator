/*
 * OpenAPI Petstore
 *
 * This is a sample server Petstore server. For this sample, you can use the api key `special-key` to test the authorization filters.
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

#[allow(unused_imports)]
use std::rc::Rc;
use std::borrow::Borrow;
use std::option::Option;

use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

pub struct StoreApiClient {
    configuration: Rc<configuration::Configuration>,
}

impl StoreApiClient {
    pub fn new(configuration: Rc<configuration::Configuration>) -> StoreApiClient {
        StoreApiClient {
            configuration,
        }
    }
}


/// struct for typed successes of method `delete_order`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteOrderSuccess {
    UnknownList(Vec<serde_json::Value>),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method `get_inventory`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetInventorySuccess {
    Status200(::std::collections::HashMap<String, i32>),
    UnknownList(Vec<serde_json::Value>),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method `get_order_by_id`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetOrderByIdSuccess {
    Status200(crate::models::Order),
    UnknownList(Vec<serde_json::Value>),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method `place_order`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PlaceOrderSuccess {
    Status200(crate::models::Order),
    UnknownList(Vec<serde_json::Value>),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `delete_order`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteOrderError {
    Status400(),
    Status404(),
    UnknownList(Vec<serde_json::Value>),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `get_inventory`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetInventoryError {
    DefaultResponse(::std::collections::HashMap<String, i32>),
    UnknownList(Vec<serde_json::Value>),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `get_order_by_id`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetOrderByIdError {
    DefaultResponse(crate::models::Order),
    Status400(),
    Status404(),
    UnknownList(Vec<serde_json::Value>),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `place_order`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PlaceOrderError {
    DefaultResponse(crate::models::Order),
    Status400(),
    UnknownList(Vec<serde_json::Value>),
    UnknownValue(serde_json::Value),
}


pub trait StoreApi {
    fn delete_order(&self, order_id: &str) -> Result<ResponseContent<DeleteOrderSuccess>, Error<DeleteOrderError>>;
    fn get_inventory(&self, ) -> Result<ResponseContent<GetInventorySuccess>, Error<GetInventoryError>>;
    fn get_order_by_id(&self, order_id: i64) -> Result<ResponseContent<GetOrderByIdSuccess>, Error<GetOrderByIdError>>;
    fn place_order(&self, body: crate::models::Order) -> Result<ResponseContent<PlaceOrderSuccess>, Error<PlaceOrderError>>;
}

impl StoreApi for StoreApiClient {
    fn delete_order(&self, order_id: &str) -> Result<ResponseContent<DeleteOrderSuccess>, Error<DeleteOrderError>> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/store/order/{orderId}", configuration.base_path, orderId=crate::apis::urlencode(order_id));
        let mut req_builder = client.delete(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

        let req = req_builder.build()?;
        let mut resp = client.execute(req)?;

        let status = resp.status();
        let content = resp.text()?;

        if status.is_success() {
            let entity: Option<DeleteOrderSuccess> = serde_json::from_str(&content).ok();
            let result = ResponseContent { status, content, entity };
            Ok(result)
        } else {
            let entity: Option<DeleteOrderError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    fn get_inventory(&self, ) -> Result<ResponseContent<GetInventorySuccess>, Error<GetInventoryError>> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/store/inventory", configuration.base_path);
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        if let Some(ref apikey) = configuration.api_key {
            let key = apikey.key.clone();
            let val = match apikey.prefix {
                Some(ref prefix) => format!("{} {}", prefix, key),
                None => key,
            };
            req_builder = req_builder.header("api_key", val);
        };

        let req = req_builder.build()?;
        let mut resp = client.execute(req)?;

        let status = resp.status();
        let content = resp.text()?;

        if status.is_success() {
            let entity: Option<GetInventorySuccess> = serde_json::from_str(&content).ok();
            let result = ResponseContent { status, content, entity };
            Ok(result)
        } else {
            let entity: Option<GetInventoryError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    fn get_order_by_id(&self, order_id: i64) -> Result<ResponseContent<GetOrderByIdSuccess>, Error<GetOrderByIdError>> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/store/order/{orderId}", configuration.base_path, orderId=order_id);
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

        let req = req_builder.build()?;
        let mut resp = client.execute(req)?;

        let status = resp.status();
        let content = resp.text()?;

        if status.is_success() {
            let entity: Option<GetOrderByIdSuccess> = serde_json::from_str(&content).ok();
            let result = ResponseContent { status, content, entity };
            Ok(result)
        } else {
            let entity: Option<GetOrderByIdError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    fn place_order(&self, body: crate::models::Order) -> Result<ResponseContent<PlaceOrderSuccess>, Error<PlaceOrderError>> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/store/order", configuration.base_path);
        let mut req_builder = client.post(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        req_builder = req_builder.json(&body);

        let req = req_builder.build()?;
        let mut resp = client.execute(req)?;

        let status = resp.status();
        let content = resp.text()?;

        if status.is_success() {
            let entity: Option<PlaceOrderSuccess> = serde_json::from_str(&content).ok();
            let result = ResponseContent { status, content, entity };
            Ok(result)
        } else {
            let entity: Option<PlaceOrderError> = serde_json::from_str(&content).ok();
            let error = ResponseContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

}

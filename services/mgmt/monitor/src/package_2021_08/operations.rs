#![doc = "generated by AutoRust"]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unused_imports)]
use super::models;
#[derive(Clone)]
pub struct Client {
    endpoint: String,
    credential: std::sync::Arc<dyn azure_core::auth::TokenCredential>,
    scopes: Vec<String>,
    pipeline: azure_core::Pipeline,
}
#[derive(Clone)]
pub struct ClientBuilder {
    credential: std::sync::Arc<dyn azure_core::auth::TokenCredential>,
    endpoint: Option<String>,
    scopes: Option<Vec<String>>,
}
pub const DEFAULT_ENDPOINT: &str = azure_core::resource_manager_endpoint::AZURE_PUBLIC_CLOUD;
impl ClientBuilder {
    pub fn new(credential: std::sync::Arc<dyn azure_core::auth::TokenCredential>) -> Self {
        Self {
            credential,
            endpoint: None,
            scopes: None,
        }
    }
    pub fn endpoint(mut self, endpoint: impl Into<String>) -> Self {
        self.endpoint = Some(endpoint.into());
        self
    }
    pub fn scopes(mut self, scopes: &[&str]) -> Self {
        self.scopes = Some(scopes.iter().map(|scope| (*scope).to_owned()).collect());
        self
    }
    pub fn build(self) -> Client {
        let endpoint = self.endpoint.unwrap_or_else(|| DEFAULT_ENDPOINT.to_owned());
        let scopes = self.scopes.unwrap_or_else(|| vec![format!("{}/", endpoint)]);
        Client::new(endpoint, self.credential, scopes)
    }
}
impl Client {
    pub(crate) fn endpoint(&self) -> &str {
        self.endpoint.as_str()
    }
    pub(crate) fn token_credential(&self) -> &dyn azure_core::auth::TokenCredential {
        self.credential.as_ref()
    }
    pub(crate) fn scopes(&self) -> Vec<&str> {
        self.scopes.iter().map(String::as_str).collect()
    }
    pub(crate) async fn send(&self, request: impl Into<azure_core::Request>) -> Result<azure_core::Response, azure_core::Error> {
        let mut context = azure_core::Context::default();
        let mut request = request.into();
        self.pipeline.send(&mut context, &mut request).await
    }
    pub fn new(
        endpoint: impl Into<String>,
        credential: std::sync::Arc<dyn azure_core::auth::TokenCredential>,
        scopes: Vec<String>,
    ) -> Self {
        let endpoint = endpoint.into();
        let pipeline = azure_core::Pipeline::new(
            option_env!("CARGO_PKG_NAME"),
            option_env!("CARGO_PKG_VERSION"),
            azure_core::ClientOptions::default(),
            Vec::new(),
            Vec::new(),
        );
        Self {
            endpoint,
            credential,
            scopes,
            pipeline,
        }
    }
    pub fn scheduled_query_rules(&self) -> scheduled_query_rules::Client {
        scheduled_query_rules::Client(self.clone())
    }
}
#[non_exhaustive]
#[derive(Debug, thiserror :: Error)]
#[allow(non_camel_case_types)]
pub enum Error {
    #[error(transparent)]
    ScheduledQueryRules_ListBySubscription(#[from] scheduled_query_rules::list_by_subscription::Error),
    #[error(transparent)]
    ScheduledQueryRules_ListByResourceGroup(#[from] scheduled_query_rules::list_by_resource_group::Error),
    #[error(transparent)]
    ScheduledQueryRules_Get(#[from] scheduled_query_rules::get::Error),
    #[error(transparent)]
    ScheduledQueryRules_CreateOrUpdate(#[from] scheduled_query_rules::create_or_update::Error),
    #[error(transparent)]
    ScheduledQueryRules_Update(#[from] scheduled_query_rules::update::Error),
    #[error(transparent)]
    ScheduledQueryRules_Delete(#[from] scheduled_query_rules::delete::Error),
}
pub mod scheduled_query_rules {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        pub fn list_by_subscription(&self, subscription_id: impl Into<String>) -> list_by_subscription::Builder {
            list_by_subscription::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
            }
        }
        pub fn list_by_resource_group(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
        ) -> list_by_resource_group::Builder {
            list_by_resource_group::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
            }
        }
        pub fn get(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            rule_name: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                rule_name: rule_name.into(),
            }
        }
        pub fn create_or_update(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            rule_name: impl Into<String>,
            parameters: impl Into<models::ScheduledQueryRuleResource>,
        ) -> create_or_update::Builder {
            create_or_update::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                rule_name: rule_name.into(),
                parameters: parameters.into(),
            }
        }
        pub fn update(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            rule_name: impl Into<String>,
            parameters: impl Into<models::ScheduledQueryRuleResourcePatch>,
        ) -> update::Builder {
            update::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                rule_name: rule_name.into(),
                parameters: parameters.into(),
            }
        }
        pub fn delete(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            rule_name: impl Into<String>,
        ) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                rule_name: rule_name.into(),
            }
        }
    }
    pub mod list_by_subscription {
        use super::models;
        #[derive(Debug, thiserror :: Error)]
        pub enum Error {
            #[error("HTTP status code {}", status_code)]
            DefaultResponse {
                status_code: http::StatusCode,
                value: models::ErrorContract,
            },
            #[error("Failed to parse request URL: {0}")]
            ParseUrl(url::ParseError),
            #[error("Failed to build request: {0}")]
            BuildRequest(http::Error),
            #[error("Failed to serialize request body: {0}")]
            Serialize(serde_json::Error),
            #[error("Failed to get access token: {0}")]
            GetToken(azure_core::Error),
            #[error("Failed to execute request: {0}")]
            SendRequest(azure_core::Error),
            #[error("Failed to get response bytes: {0}")]
            ResponseBytes(azure_core::StreamError),
            #[error("Failed to deserialize response: {0}, body: {1:?}")]
            Deserialize(serde_json::Error, bytes::Bytes),
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, std::result::Result<models::ScheduledQueryRuleResourceCollection, Error>> {
                Box::pin(async move {
                    let url_str = &format!(
                        "{}/subscriptions/{}/providers/Microsoft.Insights/scheduledQueryRules",
                        self.client.endpoint(),
                        &self.subscription_id
                    );
                    let mut url = url::Url::parse(url_str).map_err(Error::ParseUrl)?;
                    let mut req_builder = http::request::Builder::new();
                    req_builder = req_builder.method(http::Method::GET);
                    let credential = self.client.token_credential();
                    let token_response = credential
                        .get_token(&self.client.scopes().join(" "))
                        .await
                        .map_err(Error::GetToken)?;
                    req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
                    url.query_pairs_mut().append_pair("api-version", "2021-08-01");
                    let req_body = azure_core::EMPTY_BODY;
                    req_builder = req_builder.uri(url.as_str());
                    let req = req_builder.body(req_body).map_err(Error::BuildRequest)?;
                    let rsp = self.client.send(req).await.map_err(Error::SendRequest)?;
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        http::StatusCode::OK => {
                            let rsp_body = azure_core::collect_pinned_stream(rsp_stream).await.map_err(Error::ResponseBytes)?;
                            let rsp_value: models::ScheduledQueryRuleResourceCollection =
                                serde_json::from_slice(&rsp_body).map_err(|source| Error::Deserialize(source, rsp_body.clone()))?;
                            Ok(rsp_value)
                        }
                        status_code => {
                            let rsp_body = azure_core::collect_pinned_stream(rsp_stream).await.map_err(Error::ResponseBytes)?;
                            let rsp_value: models::ErrorContract =
                                serde_json::from_slice(&rsp_body).map_err(|source| Error::Deserialize(source, rsp_body.clone()))?;
                            Err(Error::DefaultResponse {
                                status_code,
                                value: rsp_value,
                            })
                        }
                    }
                })
            }
        }
    }
    pub mod list_by_resource_group {
        use super::models;
        #[derive(Debug, thiserror :: Error)]
        pub enum Error {
            #[error("HTTP status code {}", status_code)]
            DefaultResponse {
                status_code: http::StatusCode,
                value: models::ErrorContract,
            },
            #[error("Failed to parse request URL: {0}")]
            ParseUrl(url::ParseError),
            #[error("Failed to build request: {0}")]
            BuildRequest(http::Error),
            #[error("Failed to serialize request body: {0}")]
            Serialize(serde_json::Error),
            #[error("Failed to get access token: {0}")]
            GetToken(azure_core::Error),
            #[error("Failed to execute request: {0}")]
            SendRequest(azure_core::Error),
            #[error("Failed to get response bytes: {0}")]
            ResponseBytes(azure_core::StreamError),
            #[error("Failed to deserialize response: {0}, body: {1:?}")]
            Deserialize(serde_json::Error, bytes::Bytes),
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, std::result::Result<models::ScheduledQueryRuleResourceCollection, Error>> {
                Box::pin(async move {
                    let url_str = &format!(
                        "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Insights/scheduledQueryRules",
                        self.client.endpoint(),
                        &self.subscription_id,
                        &self.resource_group_name
                    );
                    let mut url = url::Url::parse(url_str).map_err(Error::ParseUrl)?;
                    let mut req_builder = http::request::Builder::new();
                    req_builder = req_builder.method(http::Method::GET);
                    let credential = self.client.token_credential();
                    let token_response = credential
                        .get_token(&self.client.scopes().join(" "))
                        .await
                        .map_err(Error::GetToken)?;
                    req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
                    url.query_pairs_mut().append_pair("api-version", "2021-08-01");
                    let req_body = azure_core::EMPTY_BODY;
                    req_builder = req_builder.uri(url.as_str());
                    let req = req_builder.body(req_body).map_err(Error::BuildRequest)?;
                    let rsp = self.client.send(req).await.map_err(Error::SendRequest)?;
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        http::StatusCode::OK => {
                            let rsp_body = azure_core::collect_pinned_stream(rsp_stream).await.map_err(Error::ResponseBytes)?;
                            let rsp_value: models::ScheduledQueryRuleResourceCollection =
                                serde_json::from_slice(&rsp_body).map_err(|source| Error::Deserialize(source, rsp_body.clone()))?;
                            Ok(rsp_value)
                        }
                        status_code => {
                            let rsp_body = azure_core::collect_pinned_stream(rsp_stream).await.map_err(Error::ResponseBytes)?;
                            let rsp_value: models::ErrorContract =
                                serde_json::from_slice(&rsp_body).map_err(|source| Error::Deserialize(source, rsp_body.clone()))?;
                            Err(Error::DefaultResponse {
                                status_code,
                                value: rsp_value,
                            })
                        }
                    }
                })
            }
        }
    }
    pub mod get {
        use super::models;
        #[derive(Debug, thiserror :: Error)]
        pub enum Error {
            #[error("HTTP status code {}", status_code)]
            DefaultResponse {
                status_code: http::StatusCode,
                value: models::ErrorContract,
            },
            #[error("Failed to parse request URL: {0}")]
            ParseUrl(url::ParseError),
            #[error("Failed to build request: {0}")]
            BuildRequest(http::Error),
            #[error("Failed to serialize request body: {0}")]
            Serialize(serde_json::Error),
            #[error("Failed to get access token: {0}")]
            GetToken(azure_core::Error),
            #[error("Failed to execute request: {0}")]
            SendRequest(azure_core::Error),
            #[error("Failed to get response bytes: {0}")]
            ResponseBytes(azure_core::StreamError),
            #[error("Failed to deserialize response: {0}, body: {1:?}")]
            Deserialize(serde_json::Error, bytes::Bytes),
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) rule_name: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, std::result::Result<models::ScheduledQueryRuleResource, Error>> {
                Box::pin(async move {
                    let url_str = &format!(
                        "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Insights/scheduledQueryRules/{}",
                        self.client.endpoint(),
                        &self.subscription_id,
                        &self.resource_group_name,
                        &self.rule_name
                    );
                    let mut url = url::Url::parse(url_str).map_err(Error::ParseUrl)?;
                    let mut req_builder = http::request::Builder::new();
                    req_builder = req_builder.method(http::Method::GET);
                    let credential = self.client.token_credential();
                    let token_response = credential
                        .get_token(&self.client.scopes().join(" "))
                        .await
                        .map_err(Error::GetToken)?;
                    req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
                    url.query_pairs_mut().append_pair("api-version", "2021-08-01");
                    let req_body = azure_core::EMPTY_BODY;
                    req_builder = req_builder.uri(url.as_str());
                    let req = req_builder.body(req_body).map_err(Error::BuildRequest)?;
                    let rsp = self.client.send(req).await.map_err(Error::SendRequest)?;
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        http::StatusCode::OK => {
                            let rsp_body = azure_core::collect_pinned_stream(rsp_stream).await.map_err(Error::ResponseBytes)?;
                            let rsp_value: models::ScheduledQueryRuleResource =
                                serde_json::from_slice(&rsp_body).map_err(|source| Error::Deserialize(source, rsp_body.clone()))?;
                            Ok(rsp_value)
                        }
                        status_code => {
                            let rsp_body = azure_core::collect_pinned_stream(rsp_stream).await.map_err(Error::ResponseBytes)?;
                            let rsp_value: models::ErrorContract =
                                serde_json::from_slice(&rsp_body).map_err(|source| Error::Deserialize(source, rsp_body.clone()))?;
                            Err(Error::DefaultResponse {
                                status_code,
                                value: rsp_value,
                            })
                        }
                    }
                })
            }
        }
    }
    pub mod create_or_update {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Ok200(models::ScheduledQueryRuleResource),
            Created201(models::ScheduledQueryRuleResource),
        }
        #[derive(Debug, thiserror :: Error)]
        pub enum Error {
            #[error("HTTP status code {}", status_code)]
            DefaultResponse {
                status_code: http::StatusCode,
                value: models::ErrorContract,
            },
            #[error("Failed to parse request URL: {0}")]
            ParseUrl(url::ParseError),
            #[error("Failed to build request: {0}")]
            BuildRequest(http::Error),
            #[error("Failed to serialize request body: {0}")]
            Serialize(serde_json::Error),
            #[error("Failed to get access token: {0}")]
            GetToken(azure_core::Error),
            #[error("Failed to execute request: {0}")]
            SendRequest(azure_core::Error),
            #[error("Failed to get response bytes: {0}")]
            ResponseBytes(azure_core::StreamError),
            #[error("Failed to deserialize response: {0}, body: {1:?}")]
            Deserialize(serde_json::Error, bytes::Bytes),
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) rule_name: String,
            pub(crate) parameters: models::ScheduledQueryRuleResource,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, std::result::Result<Response, Error>> {
                Box::pin(async move {
                    let url_str = &format!(
                        "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Insights/scheduledQueryRules/{}",
                        self.client.endpoint(),
                        &self.subscription_id,
                        &self.resource_group_name,
                        &self.rule_name
                    );
                    let mut url = url::Url::parse(url_str).map_err(Error::ParseUrl)?;
                    let mut req_builder = http::request::Builder::new();
                    req_builder = req_builder.method(http::Method::PUT);
                    let credential = self.client.token_credential();
                    let token_response = credential
                        .get_token(&self.client.scopes().join(" "))
                        .await
                        .map_err(Error::GetToken)?;
                    req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
                    url.query_pairs_mut().append_pair("api-version", "2021-08-01");
                    req_builder = req_builder.header("content-type", "application/json");
                    let req_body = azure_core::to_json(&self.parameters).map_err(Error::Serialize)?;
                    req_builder = req_builder.uri(url.as_str());
                    let req = req_builder.body(req_body).map_err(Error::BuildRequest)?;
                    let rsp = self.client.send(req).await.map_err(Error::SendRequest)?;
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        http::StatusCode::OK => {
                            let rsp_body = azure_core::collect_pinned_stream(rsp_stream).await.map_err(Error::ResponseBytes)?;
                            let rsp_value: models::ScheduledQueryRuleResource =
                                serde_json::from_slice(&rsp_body).map_err(|source| Error::Deserialize(source, rsp_body.clone()))?;
                            Ok(Response::Ok200(rsp_value))
                        }
                        http::StatusCode::CREATED => {
                            let rsp_body = azure_core::collect_pinned_stream(rsp_stream).await.map_err(Error::ResponseBytes)?;
                            let rsp_value: models::ScheduledQueryRuleResource =
                                serde_json::from_slice(&rsp_body).map_err(|source| Error::Deserialize(source, rsp_body.clone()))?;
                            Ok(Response::Created201(rsp_value))
                        }
                        status_code => {
                            let rsp_body = azure_core::collect_pinned_stream(rsp_stream).await.map_err(Error::ResponseBytes)?;
                            let rsp_value: models::ErrorContract =
                                serde_json::from_slice(&rsp_body).map_err(|source| Error::Deserialize(source, rsp_body.clone()))?;
                            Err(Error::DefaultResponse {
                                status_code,
                                value: rsp_value,
                            })
                        }
                    }
                })
            }
        }
    }
    pub mod update {
        use super::models;
        #[derive(Debug, thiserror :: Error)]
        pub enum Error {
            #[error("HTTP status code {}", status_code)]
            DefaultResponse {
                status_code: http::StatusCode,
                value: models::ErrorContract,
            },
            #[error("Failed to parse request URL: {0}")]
            ParseUrl(url::ParseError),
            #[error("Failed to build request: {0}")]
            BuildRequest(http::Error),
            #[error("Failed to serialize request body: {0}")]
            Serialize(serde_json::Error),
            #[error("Failed to get access token: {0}")]
            GetToken(azure_core::Error),
            #[error("Failed to execute request: {0}")]
            SendRequest(azure_core::Error),
            #[error("Failed to get response bytes: {0}")]
            ResponseBytes(azure_core::StreamError),
            #[error("Failed to deserialize response: {0}, body: {1:?}")]
            Deserialize(serde_json::Error, bytes::Bytes),
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) rule_name: String,
            pub(crate) parameters: models::ScheduledQueryRuleResourcePatch,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, std::result::Result<models::ScheduledQueryRuleResource, Error>> {
                Box::pin(async move {
                    let url_str = &format!(
                        "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Insights/scheduledQueryRules/{}",
                        self.client.endpoint(),
                        &self.subscription_id,
                        &self.resource_group_name,
                        &self.rule_name
                    );
                    let mut url = url::Url::parse(url_str).map_err(Error::ParseUrl)?;
                    let mut req_builder = http::request::Builder::new();
                    req_builder = req_builder.method(http::Method::PATCH);
                    let credential = self.client.token_credential();
                    let token_response = credential
                        .get_token(&self.client.scopes().join(" "))
                        .await
                        .map_err(Error::GetToken)?;
                    req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
                    url.query_pairs_mut().append_pair("api-version", "2021-08-01");
                    req_builder = req_builder.header("content-type", "application/json");
                    let req_body = azure_core::to_json(&self.parameters).map_err(Error::Serialize)?;
                    req_builder = req_builder.uri(url.as_str());
                    let req = req_builder.body(req_body).map_err(Error::BuildRequest)?;
                    let rsp = self.client.send(req).await.map_err(Error::SendRequest)?;
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        http::StatusCode::OK => {
                            let rsp_body = azure_core::collect_pinned_stream(rsp_stream).await.map_err(Error::ResponseBytes)?;
                            let rsp_value: models::ScheduledQueryRuleResource =
                                serde_json::from_slice(&rsp_body).map_err(|source| Error::Deserialize(source, rsp_body.clone()))?;
                            Ok(rsp_value)
                        }
                        status_code => {
                            let rsp_body = azure_core::collect_pinned_stream(rsp_stream).await.map_err(Error::ResponseBytes)?;
                            let rsp_value: models::ErrorContract =
                                serde_json::from_slice(&rsp_body).map_err(|source| Error::Deserialize(source, rsp_body.clone()))?;
                            Err(Error::DefaultResponse {
                                status_code,
                                value: rsp_value,
                            })
                        }
                    }
                })
            }
        }
    }
    pub mod delete {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Ok200,
            NoContent204,
        }
        #[derive(Debug, thiserror :: Error)]
        pub enum Error {
            #[error("HTTP status code {}", status_code)]
            DefaultResponse {
                status_code: http::StatusCode,
                value: models::ErrorContract,
            },
            #[error("Failed to parse request URL: {0}")]
            ParseUrl(url::ParseError),
            #[error("Failed to build request: {0}")]
            BuildRequest(http::Error),
            #[error("Failed to serialize request body: {0}")]
            Serialize(serde_json::Error),
            #[error("Failed to get access token: {0}")]
            GetToken(azure_core::Error),
            #[error("Failed to execute request: {0}")]
            SendRequest(azure_core::Error),
            #[error("Failed to get response bytes: {0}")]
            ResponseBytes(azure_core::StreamError),
            #[error("Failed to deserialize response: {0}, body: {1:?}")]
            Deserialize(serde_json::Error, bytes::Bytes),
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) rule_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, std::result::Result<Response, Error>> {
                Box::pin(async move {
                    let url_str = &format!(
                        "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Insights/scheduledQueryRules/{}",
                        self.client.endpoint(),
                        &self.subscription_id,
                        &self.resource_group_name,
                        &self.rule_name
                    );
                    let mut url = url::Url::parse(url_str).map_err(Error::ParseUrl)?;
                    let mut req_builder = http::request::Builder::new();
                    req_builder = req_builder.method(http::Method::DELETE);
                    let credential = self.client.token_credential();
                    let token_response = credential
                        .get_token(&self.client.scopes().join(" "))
                        .await
                        .map_err(Error::GetToken)?;
                    req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
                    url.query_pairs_mut().append_pair("api-version", "2021-08-01");
                    let req_body = azure_core::EMPTY_BODY;
                    req_builder = req_builder.uri(url.as_str());
                    let req = req_builder.body(req_body).map_err(Error::BuildRequest)?;
                    let rsp = self.client.send(req).await.map_err(Error::SendRequest)?;
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        http::StatusCode::OK => Ok(Response::Ok200),
                        http::StatusCode::NO_CONTENT => Ok(Response::NoContent204),
                        status_code => {
                            let rsp_body = azure_core::collect_pinned_stream(rsp_stream).await.map_err(Error::ResponseBytes)?;
                            let rsp_value: models::ErrorContract =
                                serde_json::from_slice(&rsp_body).map_err(|source| Error::Deserialize(source, rsp_body.clone()))?;
                            Err(Error::DefaultResponse {
                                status_code,
                                value: rsp_value,
                            })
                        }
                    }
                })
            }
        }
    }
}

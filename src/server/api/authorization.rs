use crate::configuration;

use anyhow::anyhow;
use hyper::Body;
use std::{
    sync::Arc,
    task::{Context, Poll},
};

use tonic::body::BoxBody;
use tower::{
    layer::util::{Identity, Stack},
    Layer, Service, ServiceBuilder,
};

/// `Authorization` middleware.
#[derive(Clone)]
pub(super) struct Authorization<S> {
    config: Arc<String>,
    inner: S,
}

/// Tower `Service` implementation for AuthorizationLayer.
impl<S> Service<hyper::Request<Body>> for Authorization<S>
where
    S: Service<hyper::Request<Body>, Response = hyper::Response<BoxBody>> + Clone + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = anyhow::Error;
    type Future = futures::future::BoxFuture<'static, Result<Self::Response, Self::Error>>;

    // Poll the inner context.
    fn poll_ready(&mut self, context: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner
            .poll_ready(context)
            .map_err(|_| anyhow!("Error polling future."))
    }

    // Execute the middleware.
    fn call(&mut self, request: hyper::Request<Body>) -> Self::Future {
        // Put the configuration path in scope.
        let config = self.config.clone();
        let config = std::mem::replace(&mut self.config, config);

        // Put the inner `Body` in scope.
        let inner = self.inner.clone();
        let mut inner = std::mem::replace(&mut self.inner, inner);

        Box::pin(async move {
            // Retrieve the server token.
            let server_token = configuration::get_token(&config).await.unwrap_or_default();

            // Retrieve the client token.
            let client_token = request
                .headers()
                .get("token")
                .ok_or_else(|| anyhow!("Missing client token."))?
                .to_str()?
                .to_string();

            // Create the service response.
            let response = inner
                .call(request)
                .await
                .map_err(|_| anyhow!("Error preparing service response."))?;

            // Proceed to service call if tokens match.
            match server_token == client_token {
                true => Ok(response),
                false => Err(anyhow!("Invalid client token.")),
            }
        })
    }
}

/// `Authorization` middleware layer.
#[derive(Clone)]
pub(super) struct AuthorizationLayer {
    config: Arc<String>,
}

impl AuthorizationLayer {
    /// Build a new `Authorization`.
    fn new(path: &str) -> Self {
        let raw = path.to_string();
        Self {
            config: Arc::new(raw),
        }
    }
}

/// Tower `Layer` implementation for AuthorizationLayer.
impl<S> Layer<S> for AuthorizationLayer {
    type Service = Authorization<S>;

    /// Build the `Authorization` middleware layer.
    fn layer(&self, service: S) -> Self::Service {
        Authorization {
            config: Arc::clone(&self.config),
            inner: service,
        }
    }
}

/// Initialize the `Authorization` middleware layer.
pub(super) fn init(path: &str) -> Stack<AuthorizationLayer, Identity> {
    let layer = AuthorizationLayer::new(path);
    let service = ServiceBuilder::new().layer(layer).into_inner();
    info!("Initialized.");
    service
}

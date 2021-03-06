// Use code generated by Tonic in `build.rs` from `proto/upate.proto`.
mod update {
    tonic::include_proto!("update");
}

use crate::configuration;

use anyhow::Result;
use std::process::ExitStatus;
use tokio::task::JoinHandle;
use tonic::{Request, Response, Status};
use update::updater_server::{Updater, UpdaterServer};
use update::{UpdateRequest, UpdateResponse};

type UpdateRx = Request<UpdateRequest>;
type UpdateTx = Response<UpdateResponse>;

/// `Handle` for other parts of code to interact with `Updater` services.
pub(super) struct Handle {
    config: String,
}

impl Handle {
    /// Create a new `Updater` services `Handle`.
    fn new(path: &str) -> Self {
        let config = path.to_string();
        Self { config }
    }
}

#[tonic::async_trait]
impl Updater for Handle {
    // Handle an `UpdateRequest` from the client.
    async fn update(&self, _: UpdateRx) -> Result<UpdateTx, Status> {
        info!("Updating server.");

        // Retrieve the path to the script.
        let script = get_script(&self.config).await?;

        // Retrieve the child process future handle.
        let child = get_child_process(&script)?;

        // Run child process.
        let task = spawn_child_task(child);

        // Await the completion of the script process task.
        match task.await.map_err(|e| Status::aborted(e.to_string()))? {
            Ok(_) => Ok(Response::new(UpdateResponse {})),
            Err(e) => Err(Status::aborted(e.to_string())),
        }
    }
}

/// Initialise `Updater` service.
pub(super) async fn init(path: &str) -> UpdaterServer<Handle> {
    let handle = Handle::new(path);
    let service = UpdaterServer::new(handle);
    info!("Initialized.");
    service
}

/// Retrieve the script path from the configuration file.
async fn get_script(path: &str) -> Result<String, Status> {
    let script = configuration::get_script(path);
    script.await.map_err(|e| Status::not_found(e.to_string()))
}

/// Retrieve the child process handle.
fn get_child_process(path: &str) -> Result<tokio::process::Child, Status> {
    let mut child = tokio::process::Command::new(path);
    child.spawn().map_err(|e| Status::aborted(e.to_string()))
}

/// Spawn the child process.
fn spawn_child_task(mut child: tokio::process::Child) -> JoinHandle<Result<ExitStatus, Status>> {
    tokio::task::spawn(async move {
        let future = child.wait();
        future.await.map_err(|e| Status::aborted(e.to_string()))
    })
}

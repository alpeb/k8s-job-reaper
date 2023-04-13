use anyhow::Result;
use clap::Parser;
use futures::{pin_mut, TryStreamExt};
use k8s_openapi::api::batch::v1::Job;
use kube::{
    api::{Api, DeleteParams, ResourceExt},
    Client,
    runtime::{watcher, WatchStreamExt},
};
use tracing_subscriber::{prelude::*, EnvFilter};

// Delete Jobs with finished Pods
#[derive(Parser)]
#[clap(version, about)]
struct Args {
    // Target namespace
    #[clap(long)]
    namespace: String,

    #[clap(long, env = "LOG_LEVEL", default_value = "info")]
    log_level: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let Args {
        namespace,
        log_level,
    } = Args::parse();

    let log_filter = log_level
        .parse::<EnvFilter>()?;
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(log_filter)
        .init();

    let client = Client::try_default().await?;

    let api: Api<Job> = Api::namespaced(client, namespace.as_str());

    let ew = watcher(api.clone(), watcher::Config::default()).applied_objects();
    pin_mut!(ew);

    while let Some(job) = ew.try_next().await? {
        if let Some(Some(successes)) = job.status.map(|s| s.succeeded) {
            if successes > 0 {
                let name = job.metadata.name.unwrap();
                tracing::info!("Killing Job {}", name);
                let _ = api.delete(name.as_str(), &DeleteParams::foreground()).await.map(|res| {
                    res.map_left(|o| {
                        tracing::info!(
                            "Deleting {}: ({:?})",
                            o.name_any(),
                            o.status.unwrap()
                        );
                    })
                    .map_right(|s| {
                        tracing::info!("Deleted Job: ({:?})", s);
                    })
                });
            }
        }
    }

    Ok(())
}

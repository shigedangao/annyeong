use clap::ArgMatches;
use k8s_openapi::api::core::v1::Pod;
use kube::Client;
use kube::api::{Api, ListParams};
use tokio::runtime::Runtime;
use super::err::AnnyeongError;

const NS: &str = "namespace";
const DEFAULT_NS: &str = "default";

pub fn trigger_list_pods(args: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let namespace = args.value_of(NS);

    let rt = Runtime::new()?;
    let res = rt.block_on(get_pods(namespace.unwrap_or(DEFAULT_NS)));

    if let Ok(pods) = res {
        for p in pods {
            println!("{} so amazing !", p);
        }
        
        return Ok(())
    }

    Err(Box::new(AnnyeongError::from(res.unwrap_err())))
}

pub async fn get_pods(ns: &str) -> Result<Vec<String>, kube::Error> {
    let client = Client::try_default().await?;
    let pods: Api<Pod> = Api::namespaced(client, ns);

    let mut pod_list: Vec<String> = Vec::new();
    for p in pods.list(&ListParams::default()).await? {
        if let Some(pod_name) = p.metadata.name {
            pod_list.push(pod_name);
        }
    }

    Ok(pod_list)
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn expect_to_get_pods() {
        let pods = super::get_pods("default").await;
        assert!(pods.is_ok());

        let pods_list = pods.unwrap();
        assert!(!pods_list.is_empty());
    }
}
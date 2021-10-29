use k8s_openapi::api::core::v1::Pod;
use kube::Client;
use kube::api::{Api, ListParams};
use tokio::runtime::Runtime;
use super::err::AnnyeongError;

pub fn trigger_list_pods() -> Result<(), Box<dyn std::error::Error>> {
    let rt = Runtime::new()?;

    let res = rt.block_on(get_pods("default"));

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
#![feature(result_option_inspect)]

use kube::{Api, Client, Config};
use kube::api::ListParams;

use mirrord_operator::crd::TargetCrd;

#[tokio::main]
async fn main() {
    let config = Config::infer().await.unwrap();
    let client = Client::try_from(config).unwrap();
    let target_api = Api::<TargetCrd>::namespaced(client, "default");
    let object_list = target_api.list(&ListParams::default()).await.unwrap();
    for target in object_list {
        println!("{target:?}");
    }
}

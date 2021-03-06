use futures::prelude::*;
use hubcaps::{Credentials, Github, InstallationTokenGenerator, JWTCredentials};
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::Read;

const USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    pretty_env_logger::init();
    let key_file = env::var("GH_APP_KEY")?;
    let app_id = env::var("GH_APP_ID")?;
    let installation_id = env::var("GH_INSTALL_ID")?;

    let mut key = Vec::new();
    File::open(&key_file)?.read_to_end(&mut key)?;
    let cred = JWTCredentials::new(app_id.parse().expect("Bad GH_APP_ID"), key)?;

    let mut github = Github::new(USER_AGENT, Credentials::JWT(cred.clone()))?;
    github.set_credentials(Credentials::InstallationToken(
        InstallationTokenGenerator::new(installation_id.parse().unwrap(), cred),
    ));

    github
        .org("NixOS")
        .membership()
        .invitations()
        .try_for_each(|invite| async move {
            println!("{:#?}", invite);
            Ok(())
        })
        .await?;

    Ok(())
}

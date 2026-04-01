use std::process::ExitCode;

use anyhow::Context;
use camino::Utf8PathBuf;
use clap::Parser;
use dropshot_api_manager::{
    Environment, ManagedApi, ManagedApiConfig, ManagedApis,
};
use dropshot_api_manager_types::{ManagedApiMetadata, Versions};
use lumen_auth_api::*;
use lumen_storage_api::*;
use serde::{Deserialize, Serialize};

fn environment() -> anyhow::Result<Environment> {
    let workspace_root = Utf8PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf();
    let env =
        Environment::new("cargo xtask openapi", workspace_root, "openapi")?;
    Ok(env)
}

fn all_apis() -> anyhow::Result<ManagedApis> {
    let apis = vec![
        ManagedApi::from(ManagedApiConfig {
            title: "Auth API",
            versions: Versions::new_versioned(
                lumen_auth_api::supported_versions(),
            ),
            metadata: ManagedApiMetadata {
                description: Some(
                    "Authentication API for consumption by other internal services.",
                ),
                contact_url: Some("https://chronicbutthriving.co.uk"),
                contact_email: Some("engineering@chronicbutthriving.co.uk"),
                extra: to_value(ApiBoundary::External),
            },
            api_description: auth_api_mod::stub_api_description,
            ident: "auth",
        }),
        ManagedApi::from(ManagedApiConfig {
            title: "Storage API",
            versions: Versions::new_versioned(
                lumen_storage_api::supported_versions(),
            ),
            metadata: ManagedApiMetadata {
                description: Some(
                    "Storage API for consumption by other internal services.",
                ),
                contact_url: Some("https://chronicbutthriving.co.uk"),
                contact_email: Some("engineering@chronicbutthriving.co.uk"),
                extra: to_value(ApiBoundary::Internal),
            },
            api_description: storage_api_mod::stub_api_description,
            ident: "storage",
        }),
    ];

    let apis = ManagedApis::new(apis)
        .context("error creating ManagedApis")?
        .with_git_stub_storage();

    Ok(apis)
}

/// A bit of extra metadata that can be supplied to each API.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct ApiExtra {
    boundary: ApiBoundary,
}

fn to_value(boundary: ApiBoundary) -> serde_json::Value {
    serde_json::to_value(ApiExtra { boundary }).unwrap()
}

/// This is some example data that is used in the `validate` function below.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum ApiBoundary {
    Internal,
    External,
}

fn main() -> anyhow::Result<ExitCode> {
    let app = dropshot_api_manager::App::parse();
    let env = environment()?;
    let apis = all_apis()?;

    Ok(app.exec(&env, &apis))
}

#[cfg(test)]
mod tests {
    use dropshot_api_manager::test_util::check_apis_up_to_date;

    use super::*;

    #[test]
    fn test_apis_up_to_date() -> anyhow::Result<ExitCode> {
        let env = environment()?;
        let apis = all_apis()?;

        let result = check_apis_up_to_date(&env, &apis)?;
        Ok(result.to_exit_code())
    }
}

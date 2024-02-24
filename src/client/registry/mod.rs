// Relative Modules
pub mod methods;

// Standard Uses

// Crate Uses

// External Uses
use eyre::{bail, Result};
use colored::Colorize;



pub fn login(method: &str, url: &str, password: Option<String>) -> Result<()> {
    let methods = [
        ("ssh", "SSH Authentication with a Private Key"),
        ("github_oidc", "Github OpenID Connect Authentication with a Token"),
        ("github", "Github Authentication with a Personal Access Token (PAT)")
    ];

    match method {
        "github" => {
            if password.is_none() {
                bail!("Github Authentication requires a PAT token, please provide one")
            }

            login_github(url, &*password.unwrap())?;
        }
        _ => {
            let mut methods_msg = String::new();
            for (name, desc) in methods {
                methods_msg += &*format!(" - {}: {desc}\n", name.green().underline());
            }

            bail!(
                "Authentication method '{}' is not valid.\n\n\
                Only existing methods are:\n{}",
                method.yellow().underline(), methods_msg
            )
        }
    }

    Ok(())
}


/// Logs-in to Github by using a user or organization name and a PAT (Personal Access Token)
fn login_github(user: &str, pat_token: &str) -> Result<()> {
    todo!()
}

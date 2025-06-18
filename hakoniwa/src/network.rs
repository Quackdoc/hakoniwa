mod pasta;

pub use pasta::Pasta;

use nix::unistd::Pid;
use std::process::Command;

use crate::{error::*, Container};

/// Network mode.
#[derive(Clone, Debug)]
pub enum Network {
    Pasta(Pasta),
}

impl From<Pasta> for Network {
    fn from(val: Pasta) -> Self {
        Self::Pasta(val)
    }
}

pub(crate) fn configure(container: &Container, child: Pid) -> Result<()> {
//    match &container.network {
//        Some(network) => match network {
//            Network::Pasta(pasta) => configure_pasta(pasta, child)?,
//        },
//        None => unreachable!(),
//    };

    let o = Command::new("newuidmap").args([child.as_raw().to_string(),
        "0".to_string(), "1000".to_string(), "1".to_string(),
        "1".to_string(), "100000".to_string(), "65536".to_string()]).output();

    let o = Command::new("newgidmap").args([child.as_raw().to_string(),
        "0".to_string(), "1000".to_string(), "1".to_string(),
        "1".to_string(), "100000".to_string(), "65536".to_string()]).output();

    log::debug!("================================");
    Ok(())
}

fn configure_pasta(pasta: &Pasta, child: Pid) -> Result<()> {
    let cmdline = pasta.to_cmdline(child);
    log::debug!("Configuring Network: Execve: \n{:?}", cmdline);

    let output = Command::new(cmdline[0].clone())
        .args(&cmdline[1..])
        .output();
    let output = match output {
        Ok(output) => {
            let stderr = String::from_utf8_lossy(&output.stderr);
            log::debug!("Configuring Network: Output: \n{}", &stderr);
            output
        }
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
            let errmsg = format!("Command {:?} not found", pasta.prog);
            log::debug!("Configuring Network: Output: \n{}", errmsg);
            Err(ProcessErrorKind::StdIoError(err))?
        }
        Err(err) => {
            log::debug!("Configuring Network: Output: \n{}", err);
            Err(ProcessErrorKind::StdIoError(err))?
        }
    };

    if output.status.success() {
        Ok(())
    } else {
        Err(ProcessErrorKind::SetupNetworkFailed)?
    }
}

//***************************************************************************
//
// In the end this file should be auto-generated with the varlink-generator
// from the corresponding varlink file.
//
//***************************************************************************

use std::result::Result;
use std::convert::From;

use varlink;

pub trait Interface: varlink::server::Interface {
    fn info(&self, i64) -> Result<InfoReply, Error>;
    fn list(&self) -> Result<ListReply, Error>;
}

#[macro_export]
macro_rules! IoSystemdNetwork {
	(
		()
		$(pub)* struct $name:ident $($_tail:tt)*
	) => {

impl varlink::server::Interface for $name {
    fn get_description(&self) -> &'static str {
        r#"
# Provides information about network state
interface io.systemd.network

type NetdevInfo (
  ifindex: int,
  ifname: string
)

type Netdev (
  ifindex: int,
  ifname: string
)

# Returns information about a network device
method Info(ifindex: int) -> (info: NetdevInfo)

# Lists all network devices
method List() -> (netdevs: Netdev[])

error UnknownNetworkDevice ()
error UnknownError (text: string)
	"#
    }

    fn get_name(&self) -> &'static str {
        "io.systemd.network"
    }

    fn call(&self, req: varlink::server::Request) -> Result<serde_json::Value, varlink::server::Error> {
        match req.method.as_ref() {
            "io.systemd.network.Info" => {
                if let Some(args) = req.parameters {
                    let infoargs: Result<InfoArgs, serde_json::Error> =
                        serde_json::from_value(args);
                    match infoargs {
                        Ok(args) => Ok(serde_json::to_value(self.info(args.ifindex)?)?),
                        Err(e) => Err(varlink::server::VarlinkError::InvalidParameter(Some(e.to_string().into())).into())
                    }
                } else {
                    Err(varlink::server::VarlinkError::InvalidParameter(None).into())
                }
            }
            "io.systemd.network.List" => Ok(serde_json::to_value(self.list()?)?),
            m => {
                let method: String = m.clone().into();
                Err(varlink::server::VarlinkError::MethodNotFound(Some(method.into())).into())
            }
        }
    }
}
};
}


#[derive(Debug)]
pub enum Error {
    UnknownNetworkDevice,
}

impl From<Error> for varlink::server::Error {
    fn from(e: Error) -> Self {
        varlink::server::Error {
            error: match e {
                Error::UnknownNetworkDevice => "io.systemd.network.UnknownNetworkDevice".into(),
            },
            parameters: match e {
                _ => None,
            },
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NetdevInfo {
    pub ifindex: i64,
    pub ifname: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Netdev {
    pub ifindex: i64,
    pub ifname: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InfoArgs {
    pub ifindex: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InfoReply {
    pub info: NetdevInfo,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ListReply {
    pub netdevs: Vec<Netdev>,
}

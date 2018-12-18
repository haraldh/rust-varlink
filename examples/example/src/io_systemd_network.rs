#![doc = "This file was automatically generated by the varlink rust generator"]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
use chainerror::*;
use serde_derive::{Deserialize, Serialize};
use serde_json;
use std::io::BufRead;
use std::sync::{Arc, RwLock};
use varlink::{self, CallTrait};
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct r#Netdev {
    pub r#ifindex: i64,
    pub r#ifname: String,
}
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct r#NetdevInfo {
    pub r#ifindex: i64,
    pub r#ifname: String,
}
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct UnknownError_Args {
    pub r#text: String,
}
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct UnknownNetworkIfIndex_Args {
    pub r#ifindex: i64,
}
pub trait VarlinkCallError: varlink::CallTrait {
    fn reply_unknown_error(&mut self, r#text: String) -> varlink::Result<()> {
        self.reply_struct(varlink::Reply::error(
            "io.systemd.network.UnknownError",
            Some(serde_json::to_value(UnknownError_Args { r#text }).map_err(minto_cherr!())?),
        ))
    }
    fn reply_unknown_network_if_index(&mut self, r#ifindex: i64) -> varlink::Result<()> {
        self.reply_struct(varlink::Reply::error(
            "io.systemd.network.UnknownNetworkIfIndex",
            Some(
                serde_json::to_value(UnknownNetworkIfIndex_Args { r#ifindex })
                    .map_err(minto_cherr!())?,
            ),
        ))
    }
}
impl<'a> VarlinkCallError for varlink::Call<'a> {}
#[allow(dead_code)]
#[derive(Clone, PartialEq, Debug)]
pub enum ErrorKind {
    Io_Error(::std::io::ErrorKind),
    SerdeJson_Error(serde_json::error::Category),
    Varlink_Error,
    VarlinkReply_Error(varlink::Reply),
    Generic,
    UnknownError(Option<UnknownError_Args>),
    UnknownNetworkIfIndex(Option<UnknownNetworkIfIndex_Args>),
}
impl ::std::fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match self {
            ErrorKind::Io_Error(_) => write!(f, "IO error"),
            ErrorKind::SerdeJson_Error(_) => write!(f, "(De)Serialization Error"),
            ErrorKind::Varlink_Error => write!(f, "Varlink Error"),
            ErrorKind::VarlinkReply_Error(v) => write!(f, "Unknown error reply: '{:#?}'", v),
            ErrorKind::Generic => Ok(()),
            ErrorKind::UnknownError(v) => write!(f, "io.systemd.network.UnknownError: {:#?}", v),
            ErrorKind::UnknownNetworkIfIndex(v) => {
                write!(f, "io.systemd.network.UnknownNetworkIfIndex: {:#?}", v)
            }
        }
    }
}
impl ::std::error::Error for ErrorKind {}
impl ChainErrorFrom<std::io::Error> for ErrorKind {
    fn chain_error_from(
        e: std::io::Error,
        line_filename: Option<(u32, &'static str)>,
    ) -> ChainError<Self> {
        ChainError::<_>::new(
            ErrorKind::Io_Error(e.kind()),
            Some(Box::from(e)),
            line_filename,
        )
    }
}
impl ChainErrorFrom<serde_json::error::Error> for ErrorKind {
    fn chain_error_from(
        e: serde_json::error::Error,
        line_filename: Option<(u32, &'static str)>,
    ) -> ChainError<Self> {
        ChainError::<_>::new(
            ErrorKind::SerdeJson_Error(e.classify()),
            Some(Box::from(e)),
            line_filename,
        )
    }
}
impl ChainErrorFrom<varlink::ErrorKind> for ErrorKind {
    fn chain_error_from(
        e: varlink::ErrorKind,
        line_filename: Option<(u32, &'static str)>,
    ) -> ChainError<Self> {
        ChainError::<_>::new(
            ErrorKind::Varlink_Error,
            Some(Box::from(ChainError::<_>::new(e, None, line_filename))),
            line_filename,
        )
    }
}
#[allow(dead_code)]
pub type Result<T> = ChainResult<T, ErrorKind>;
#[allow(dead_code)]
pub type Error = ErrorKind;
impl ChainErrorFrom<varlink::Reply> for ErrorKind {
    #[allow(unused_variables)]
    fn chain_error_from(
        e: varlink::Reply,
        line_filename: Option<(u32, &'static str)>,
    ) -> ChainError<Self> {
        if varlink::ErrorKind::is_error(&e) {
            let e: varlink::ErrorKind = e.into();
            return into_cherr!(e);
        }
        match e {
            varlink::Reply {
                error: Some(ref t), ..
            } if t == "io.systemd.network.UnknownError" => match e {
                varlink::Reply {
                    parameters: Some(p),
                    ..
                } => match serde_json::from_value(p) {
                    Ok(v) => into_cherr!(ErrorKind::UnknownError(v)),
                    Err(_) => into_cherr!(ErrorKind::UnknownError(None)),
                },
                _ => into_cherr!(ErrorKind::UnknownError(None)),
            },
            varlink::Reply {
                error: Some(ref t), ..
            } if t == "io.systemd.network.UnknownNetworkIfIndex" => match e {
                varlink::Reply {
                    parameters: Some(p),
                    ..
                } => match serde_json::from_value(p) {
                    Ok(v) => into_cherr!(ErrorKind::UnknownNetworkIfIndex(v)),
                    Err(_) => into_cherr!(ErrorKind::UnknownNetworkIfIndex(None)),
                },
                _ => into_cherr!(ErrorKind::UnknownNetworkIfIndex(None)),
            },
            _ => into_cherr!(ErrorKind::VarlinkReply_Error(e)),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Info_Reply {
    pub r#info: NetdevInfo,
}
impl varlink::VarlinkReply for Info_Reply {}
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Info_Args {
    pub r#ifindex: i64,
}
pub trait Call_Info: VarlinkCallError {
    fn reply(&mut self, r#info: NetdevInfo) -> varlink::Result<()> {
        self.reply_struct(Info_Reply { r#info }.into())
    }
}
impl<'a> Call_Info for varlink::Call<'a> {}
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct List_Reply {
    pub r#netdevs: Vec<Netdev>,
}
impl varlink::VarlinkReply for List_Reply {}
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct List_Args {}
pub trait Call_List: VarlinkCallError {
    fn reply(&mut self, r#netdevs: Vec<Netdev>) -> varlink::Result<()> {
        self.reply_struct(List_Reply { r#netdevs }.into())
    }
}
impl<'a> Call_List for varlink::Call<'a> {}
pub trait VarlinkInterface {
    fn info(&self, call: &mut Call_Info, r#ifindex: i64) -> varlink::Result<()>;
    fn list(&self, call: &mut Call_List) -> varlink::Result<()>;
    fn call_upgraded(
        &self,
        _call: &mut varlink::Call,
        _bufreader: &mut BufRead,
    ) -> varlink::Result<Vec<u8>> {
        Ok(Vec::new())
    }
}
pub trait VarlinkClientInterface {
    fn info(&mut self, r#ifindex: i64) -> varlink::MethodCall<Info_Args, Info_Reply, Error>;
    fn list(&mut self) -> varlink::MethodCall<List_Args, List_Reply, Error>;
}
#[allow(dead_code)]
pub struct VarlinkClient {
    connection: Arc<RwLock<varlink::Connection>>,
}
impl VarlinkClient {
    #[allow(dead_code)]
    pub fn new(connection: Arc<RwLock<varlink::Connection>>) -> Self {
        VarlinkClient { connection }
    }
}
impl VarlinkClientInterface for VarlinkClient {
    fn info(&mut self, r#ifindex: i64) -> varlink::MethodCall<Info_Args, Info_Reply, Error> {
        varlink::MethodCall::<Info_Args, Info_Reply, Error>::new(
            self.connection.clone(),
            "io.systemd.network.Info",
            Info_Args { r#ifindex },
        )
    }
    fn list(&mut self) -> varlink::MethodCall<List_Args, List_Reply, Error> {
        varlink::MethodCall::<List_Args, List_Reply, Error>::new(
            self.connection.clone(),
            "io.systemd.network.List",
            List_Args {},
        )
    }
}
#[allow(dead_code)]
pub struct VarlinkInterfaceProxy {
    inner: Box<VarlinkInterface + Send + Sync>,
}
#[allow(dead_code)]
pub fn new(inner: Box<VarlinkInterface + Send + Sync>) -> VarlinkInterfaceProxy {
    VarlinkInterfaceProxy { inner }
}
impl varlink::Interface for VarlinkInterfaceProxy {
    fn get_description(&self) -> &'static str {
        "# Provides information about network state\n#\ninterface io.systemd.network\n\ntype NetdevInfo (\n  ifindex: int,\n  ifname: string\n)\n\ntype Netdev (\n  ifindex: int,\n  ifname: string\n)\n\n# Returns information about a network device\nmethod Info(ifindex: int) -> (info: NetdevInfo)\n\n# Lists all network devices\nmethod List() -> (netdevs: []Netdev)\n\nerror UnknownNetworkIfIndex (ifindex: int)\nerror UnknownError (text: string)\n"
    }
    fn get_name(&self) -> &'static str {
        "io.systemd.network"
    }
    fn call_upgraded(
        &self,
        call: &mut varlink::Call,
        bufreader: &mut BufRead,
    ) -> varlink::Result<Vec<u8>> {
        self.inner.call_upgraded(call, bufreader)
    }
    fn call(&self, call: &mut varlink::Call) -> varlink::Result<()> {
        let req = call.request.unwrap();
        match req.method.as_ref() {
            "io.systemd.network.Info" => {
                if let Some(args) = req.parameters.clone() {
                    let args: Info_Args = match serde_json::from_value(args) {
                        Ok(v) => v,
                        Err(e) => {
                            let es = format!("{}", e);
                            let _ = call.reply_invalid_parameter(es.clone());
                            return Err(into_cherr!(varlink::ErrorKind::SerdeJsonDe(es)));
                        }
                    };
                    self.inner.info(call as &mut Call_Info, args.r#ifindex)
                } else {
                    call.reply_invalid_parameter("parameters".into())
                }
            }
            "io.systemd.network.List" => self.inner.list(call as &mut Call_List),
            m => call.reply_method_not_found(String::from(m)),
        }
    }
}

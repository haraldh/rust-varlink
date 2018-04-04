//! DO NOT EDIT
//! This file is automatically generated by the varlink rust generator

#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use serde_json;
use std::io;
use std::sync::{Arc, RwLock};
use varlink;
use varlink::CallTrait;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct State {
    #[serde(skip_serializing_if = "Option::is_none")] pub start: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")] pub progress: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")] pub end: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct _PingReply {
    #[serde(skip_serializing_if = "Option::is_none")] pub pong: Option<String>,
}

impl varlink::VarlinkReply for _PingReply {}

#[derive(Serialize, Deserialize, Debug)]
pub struct _PingArgs {
    #[serde(skip_serializing_if = "Option::is_none")] pub ping: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct _StopServingReply {}

impl varlink::VarlinkReply for _StopServingReply {}

#[derive(Serialize, Deserialize, Debug)]
pub struct _StopServingArgs {}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct _TestMoreReply {
    #[serde(skip_serializing_if = "Option::is_none")] pub state: Option<State>,
}

impl varlink::VarlinkReply for _TestMoreReply {}

#[derive(Serialize, Deserialize, Debug)]
pub struct _TestMoreArgs {
    #[serde(skip_serializing_if = "Option::is_none")] pub n: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct _TestMoreErrorArgs {
    #[serde(skip_serializing_if = "Option::is_none")] pub reason: Option<String>,
}

pub trait _CallErr: varlink::CallTrait {
    fn reply_test_more_error(&mut self, reason: Option<String>) -> io::Result<()> {
        self.reply_struct(varlink::Reply::error(
            "org.example.more.TestMoreError".into(),
            Some(serde_json::to_value(_TestMoreErrorArgs { reason }).unwrap()),
        ))
    }
}

impl<'a> _CallErr for varlink::Call<'a> {}

pub enum _Error {
    TestMoreError(_TestMoreErrorArgs),
    VarlinkError_(varlink::Error),
    UnknownError_(varlink::Reply),
    IOError_(io::Error),
    JSONError_(serde_json::Error),
}

impl From<varlink::Reply> for _Error {
    fn from(e: varlink::Reply) -> Self {
        if varlink::Error::is_error(&e) {
            return _Error::VarlinkError_(e.into());
        }

        match e {
            varlink::Reply {
                error: Some(ref t), ..
            } if t == "org.example.more.TestMoreError" =>
            {
                match e {
                    varlink::Reply {
                        parameters: Some(p),
                        ..
                    } => match serde_json::from_value(p) {
                        Ok(v) => _Error::TestMoreError(v),
                        Err(_) => _Error::TestMoreError(_TestMoreErrorArgs {
                            ..Default::default()
                        }),
                    },
                    _ => _Error::TestMoreError(_TestMoreErrorArgs {
                        ..Default::default()
                    }),
                }
            }
            _ => return _Error::UnknownError_(e),
        }
    }
}

impl From<io::Error> for _Error {
    fn from(e: io::Error) -> Self {
        _Error::IOError_(e)
    }
}

impl From<serde_json::Error> for _Error {
    fn from(e: serde_json::Error) -> Self {
        use serde_json::error::Category;
        match e.classify() {
            Category::Io => _Error::IOError_(e.into()),
            _ => _Error::JSONError_(e),
        }
    }
}

impl From<_Error> for io::Error {
    fn from(e: _Error) -> Self {
        match e {
            _Error::TestMoreError(e) => io::Error::new(
                io::ErrorKind::Other,
                format!(
                    "org.example.more.TestMoreError: '{}'",
                    serde_json::to_string_pretty(&e).unwrap()
                ),
            ),
            _Error::VarlinkError_(e) => e.into(),
            _Error::IOError_(e) => e,
            _Error::JSONError_(e) => e.into(),
            _Error::UnknownError_(e) => io::Error::new(
                io::ErrorKind::Other,
                format!(
                    "unknown varlink error: {}",
                    serde_json::to_string_pretty(&e).unwrap()
                ),
            ),
        }
    }
}
pub trait _CallPing: _CallErr {
    fn reply(&mut self, pong: Option<String>) -> io::Result<()> {
        self.reply_struct(_PingReply { pong }.into())
    }
}

impl<'a> _CallPing for varlink::Call<'a> {}

pub trait _CallStopServing: _CallErr {
    fn reply(&mut self) -> io::Result<()> {
        self.reply_struct(varlink::Reply::parameters(None))
    }
}

impl<'a> _CallStopServing for varlink::Call<'a> {}

pub trait _CallTestMore: _CallErr {
    fn reply(&mut self, state: Option<State>) -> io::Result<()> {
        self.reply_struct(_TestMoreReply { state }.into())
    }
}

impl<'a> _CallTestMore for varlink::Call<'a> {}

pub trait VarlinkInterface {
    fn ping(&self, call: &mut _CallPing, ping: Option<String>) -> io::Result<()>;
    fn stop_serving(&self, call: &mut _CallStopServing) -> io::Result<()>;
    fn test_more(&self, call: &mut _CallTestMore, n: Option<i64>) -> io::Result<()>;
    fn call_upgraded(&self, _call: &mut varlink::Call) -> io::Result<()> {
        Ok(())
    }
}

pub trait VarlinkClientInterface {
    fn ping(
        &mut self,
        ping: Option<String>,
    ) -> io::Result<varlink::MethodCall<_PingArgs, _PingReply, _Error>>;
    fn stop_serving(
        &mut self,
    ) -> io::Result<varlink::MethodCall<_StopServingArgs, _StopServingReply, _Error>>;
    fn test_more(
        &mut self,
        n: Option<i64>,
    ) -> io::Result<varlink::MethodCall<_TestMoreArgs, _TestMoreReply, _Error>>;
}

pub struct VarlinkClient {
    connection: Arc<RwLock<varlink::Connection>>,
    more: bool,
}

impl VarlinkClient {
    pub fn new(connection: Arc<RwLock<varlink::Connection>>) -> Self {
        VarlinkClient {
            connection,
            more: false,
        }
    }
    pub fn more(&self) -> Self {
        VarlinkClient {
            connection: self.connection.clone(),
            more: true,
        }
    }
}

impl VarlinkClientInterface for VarlinkClient {
    fn ping(
        &mut self,
        ping: Option<String>,
    ) -> io::Result<varlink::MethodCall<_PingArgs, _PingReply, _Error>> {
        varlink::MethodCall::<_PingArgs, _PingReply, _Error>::call(
            self.connection.clone(),
            "org.example.more.Ping".into(),
            _PingArgs { ping },
            self.more,
        )
    }
    fn stop_serving(
        &mut self,
    ) -> io::Result<varlink::MethodCall<_StopServingArgs, _StopServingReply, _Error>> {
        varlink::MethodCall::<_StopServingArgs, _StopServingReply, _Error>::call(
            self.connection.clone(),
            "org.example.more.StopServing".into(),
            _StopServingArgs {},
            self.more,
        )
    }
    fn test_more(
        &mut self,
        n: Option<i64>,
    ) -> io::Result<varlink::MethodCall<_TestMoreArgs, _TestMoreReply, _Error>> {
        varlink::MethodCall::<_TestMoreArgs, _TestMoreReply, _Error>::call(
            self.connection.clone(),
            "org.example.more.TestMore".into(),
            _TestMoreArgs { n },
            self.more,
        )
    }
}

pub struct _InterfaceProxy {
    inner: Box<VarlinkInterface + Send + Sync>,
}

pub fn new(inner: Box<VarlinkInterface + Send + Sync>) -> _InterfaceProxy {
    _InterfaceProxy { inner }
}

impl varlink::Interface for _InterfaceProxy {
    fn get_description(&self) -> &'static str {
        r#"# Example service
interface org.example.more

# Enum, returning either start, progress or end
# progress: [0-100]
type State (
  start: bool,
  progress: int,
  end: bool
)

# Returns the same string
method Ping(ping: string) -> (pong: string)

# Dummy progress method
# n: number of progress steps
method TestMore(n: int) -> (state: State)

# Stop serving
method StopServing() -> ()

# Something failed in TestMore
error TestMoreError (reason: string)
"#
    }

    fn get_name(&self) -> &'static str {
        "org.example.more"
    }

    fn call_upgraded(&self, call: &mut varlink::Call) -> io::Result<()> {
        self.inner.call_upgraded(call)
    }

    fn call(&self, call: &mut varlink::Call) -> io::Result<()> {
        let req = call.request.unwrap();
        match req.method.as_ref() {
            "org.example.more.Ping" => {
                if let Some(args) = req.parameters.clone() {
                    let args: _PingArgs = serde_json::from_value(args)?;
                    return self.inner.ping(call as &mut _CallPing, args.ping);
                } else {
                    return call.reply_invalid_parameter(None);
                }
            }
            "org.example.more.StopServing" => {
                return self.inner.stop_serving(call as &mut _CallStopServing);
            }
            "org.example.more.TestMore" => {
                if let Some(args) = req.parameters.clone() {
                    let args: _TestMoreArgs = serde_json::from_value(args)?;
                    return self.inner.test_more(call as &mut _CallTestMore, args.n);
                } else {
                    return call.reply_invalid_parameter(None);
                }
            }

            m => {
                return call.reply_method_not_found(Some(String::from(m)));
            }
        }
    }
}

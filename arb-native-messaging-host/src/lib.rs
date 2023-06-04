mod errors;

pub use crate::errors::Error;

use {
    serde::Serialize,
    serde_json::{
        json,
        Value,
    },
    std::{
        borrow::Cow,
        fmt::Display,
        io::{
            self,
            Read,
            Write,
        },
        panic,
        process::Command,
    },
};

/// Writes the given JSON data to stdout, thereby 'sending' a message
/// back to the browser.
#[macro_export]
macro_rules! send {
    ($($json:tt)+) => {{
        let v = json!($($json),+);
        $crate::send_message(::std::io::stdout(), &v)
    }}
}

/// Reads input from a stream, decoded according to
/// the Chrome documentation on native messaging.
/// (https://developer.chrome.com/extensions/nativeMessaging)
/// 1. A 32bit unsigned integer specifies how long the message is.
/// 2. The message is encoded in JSON
pub fn read_input<R: Read>(mut input: R) -> Result<Value, Error> {
    let mut buf = [0; 4];
    match input.read_exact(&mut buf).map(|()| u32::from_ne_bytes(buf)) {
        Ok(length) => {
            //println!("Found length: {}", length);
            let mut buffer = vec![0; length as usize];
            input.read_exact(&mut buffer)?;
            let value = serde_json::from_slice(&buffer)?;
            Ok(value)
        },
        Err(e) => match e.kind() {
            io::ErrorKind::UnexpectedEof => Err(Error::NoMoreInput),
            _ => Err(e.into()),
        },
    }
}

/// Writes an output to a stream, encoded according to
/// the Chrome documentation on native messaging.
/// (https://developer.chrome.com/extensions/nativeMessaging)
/// Takes a custom value which implements serde::Serialize.
pub fn send_message<W: Write, T: Serialize>(
    mut output: W,
    value: &T,
) -> Result<(), Error> {
    let msg = serde_json::to_string(value)?;
    let len = msg.len();
    // Chrome won't accept a message larger than 1MB
    if len > 1024 * 1024 {
        return Err(Error::MessageTooLarge { size: len });
    }
    let len = len as u32; // Cast is safe due to size check above
    let len_bytes = len.to_ne_bytes();
    output.write_all(&len_bytes)?;
    output.write_all(msg.as_bytes())?;
    output.flush()?;
    Ok(())
}

/// Handles a panic in the application code, by sending
/// a message back to the browser before exiting.
fn handle_panic(info: &std::panic::PanicInfo) {
    let msg = match info.payload().downcast_ref::<&'static str>() {
        Some(s) => *s,
        None => match info.payload().downcast_ref::<String>() {
            Some(s) => &s[..],
            None => "Box<Any>",
        },
    };
    // Ignore error if send fails, we don't want to panic inside the panic handler.
    let _ = send!({
        "status": "panic",
        "payload": msg,
        "file": info.location().map(|l| l.file()),
        "line": info.location().map(|l| l.line())
    });
}

#[derive(Serialize)]
struct BasicMessage {
    payload: String,
}

/// Starts an 'event loop' which listens and writes to
/// stdin and stdout respectively.
///
/// Despite its name implying an asynchronous nature,
/// this function blocks waiting for input.
pub fn event_loop<T, E, F>(callback: F)
where
    F: Fn(serde_json::Value) -> Result<T, E>,
    T: Serialize,
    E: Display,
{
    panic::set_hook(Box::new(handle_panic));

    loop {
        // Wait for input.
        match read_input(io::stdin()) {
            Ok(v) => match callback(v) {
                Ok(response) => send_message(io::stdout(), &response).unwrap(),
                Err(e) => send!({ "error": format!("{}", e) }).unwrap(),
            },
            Err(e) => {
                // If the input stream has finished, then we exit the event loop.
                if let Error::NoMoreInput = e {
                    break;
                }
                send!({ "error": format!("{}", e) }).unwrap();
            },
        }
    }
}

pub fn main() {
    fn run_cmd(
        cmd: String,
        subcmd: String,
        options: &[&str],
    ) -> Cow<'static, str> {
        let mut arb = Command::new("./arb");

        let arb_with_cmd = arb.arg(cmd);

        let output = if subcmd != *"" {
            if options.len() > 0 {
                let mut temp = arb_with_cmd.arg(subcmd);
                for option in options {
                    if option.contains(":") {
                        let (key, value) = option.split_once(":").unwrap();
                        temp = temp.arg(key).arg(value);
                    } else {
                        temp = temp.arg(option);
                    }
                }
                temp.output().expect("failed to execute process")
            } else {
                arb_with_cmd
                    .arg(subcmd)
                    .output()
                    .expect("failed to execute process")
            }
        } else {
            if options.len() > 0 {
                let mut temp = arb_with_cmd;
                for option in options {
                    if option.contains(":") {
                        let (key, value) = option.split_once(":").unwrap();
                        temp = temp.arg(key).arg(value);
                    } else {
                        temp = temp.arg(option);
                    }
                }
                temp.output().expect("failed to execute process")
            } else {
                arb_with_cmd.output().expect("failed to execute process")
            }
        };

        let stdout_str = String::from_utf8_lossy(&output.stdout).to_string();

        if stdout_str != *"" {
            Cow::from(stdout_str.to_owned())
        } else {
            Cow::from(String::from_utf8_lossy(&output.stderr).to_string())
        }
    }

    fn match_it(value: Value) -> Result<BasicMessage, std::io::Error> {
        let input = &*value
            .as_str()
            .expect("REASON")
            .split("/")
            .collect::<Vec<_>>();
        let options = &input[1..];
        match input[0] {
            "arb>epochs" => Ok::<BasicMessage, io::Error>(BasicMessage {
                payload: run_cmd("epochs".to_owned(), "".to_owned(), options).to_string(),
            }),
            "arb>find" => Ok::<BasicMessage, io::Error>(BasicMessage {
                payload: run_cmd("find".to_owned(), "".to_owned(), options).to_string(),
            }),
            "arb>index" => Ok::<BasicMessage, io::Error>(BasicMessage {
                payload: run_cmd("index".to_owned(), "".to_owned(), options).to_string(),
            }),
            "arb>info" => Ok::<BasicMessage, io::Error>(BasicMessage {
                payload: run_cmd("info".to_owned(), "".to_owned(), options).to_string(),
            }),
            "arb>list" => Ok::<BasicMessage, io::Error>(BasicMessage {
                payload: run_cmd("list".to_owned(), "".to_owned(), options).to_string(),
            }),
            "arb>parse" => Ok::<BasicMessage, io::Error>(BasicMessage {
                payload: run_cmd("parse".to_owned(), "".to_owned(), options).to_string(),
            }),
            "arb>subsidy" => Ok::<BasicMessage, io::Error>(BasicMessage {
                payload: run_cmd("subsidy".to_owned(), "".to_owned(), options).to_string(),
            }),
            "arb>supply" => Ok::<BasicMessage, io::Error>(BasicMessage {
                payload: run_cmd("supply".to_owned(), "".to_owned(), options).to_string(),
            }),
            "arb>traits" => Ok::<BasicMessage, io::Error>(BasicMessage {
                payload: run_cmd("traits".to_owned(), "".to_owned(), options).to_string(),
            }),
            "arb>wallet>balance" => Ok::<BasicMessage, io::Error>(BasicMessage {
                payload: run_cmd("wallet".to_owned(), "balance".to_owned(), options).to_string(),
            }),
            "arb>wallet>cardinals" => Ok::<BasicMessage, io::Error>(BasicMessage {
                payload: run_cmd("wallet".to_owned(), "cardinals".to_owned(), options).to_string(),
            }),
            "arb>wallet>create" => Ok::<BasicMessage, io::Error>(BasicMessage {
                payload: run_cmd("wallet".to_owned(), "create".to_owned(), options).to_string(),
            }),
            "arb>wallet>inscribe" => Ok::<BasicMessage, io::Error>(BasicMessage {
                payload: run_cmd("wallet".to_owned(), "inscribe".to_owned(), options).to_string(),
            }),
            "arb>wallet>inscriptions" => Ok::<BasicMessage, io::Error>(BasicMessage {
                payload: run_cmd("wallet".to_owned(), "inscriptions".to_owned(), options)
                    .to_string(),
            }),
            "arb>wallet>outputs" => Ok::<BasicMessage, io::Error>(BasicMessage {
                payload: run_cmd("wallet".to_owned(), "outputs".to_owned(), options).to_string(),
            }),
            "arb>wallet>receive" => Ok::<BasicMessage, io::Error>(BasicMessage {
                payload: run_cmd("wallet".to_owned(), "receive".to_owned(), options).to_string(),
            }),
            "arb>wallet>restore" => Ok::<BasicMessage, io::Error>(BasicMessage {
                payload: run_cmd("wallet".to_owned(), "restore".to_owned(), options).to_string(),
            }),
            "arb>wallet>sats" => Ok::<BasicMessage, io::Error>(BasicMessage {
                payload: run_cmd("wallet".to_owned(), "sats".to_owned(), options).to_string(),
            }),
            "arb>wallet>send" => Ok::<BasicMessage, io::Error>(BasicMessage {
                payload: run_cmd("wallet".to_owned(), "send".to_owned(), options).to_string(),
            }),
            "arb>wallet>transactions" => Ok::<BasicMessage, io::Error>(BasicMessage {
                payload: run_cmd("wallet".to_owned(), "transactions".to_owned(), options)
                    .to_string(),
            }),
            _ => Ok::<BasicMessage, io::Error>(BasicMessage {
                payload: "Error: Unknown Command".to_string(),
            }),
        }
    }

    event_loop(match_it);
}

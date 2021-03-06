use std::{env::Args, io::Write, path::PathBuf, process::{Child, ChildStderr, ChildStdin, ChildStdout}};
use std::{
    io::Read,
    process::{Command, Stdio},
    sync::{Arc, Mutex},
    thread,
};

pub struct SubProcess {
    child: Child,
    stdout: Arc<Mutex<Vec<u8>>>,
    stderr: Arc<Mutex<Vec<u8>>>,
}

fn handle_byte<S: Read + Send + 'static>(stream: &mut S, vec: &Arc<Mutex<Vec<u8>>>) -> bool {
    let mut buf = [0];
    match stream.read(&mut buf) {
        Err(e) => {
            eprintln!("{}: Stream error: {}", line!(), e);
        }
        Ok(size) => {
            if size == 1 {
                vec.lock().expect("Mutex lock poisoned").push(buf[0]);
                return false;
            } else if size != 0 {
                eprintln!("{}: Bad number of bytes: {}", line!(), size);
            }
        }
    }   
    return true;   
}

// Credit to
// https://www.javaer101.com/es/article/20362830.html
fn child_non_blocking_stream<S: Read + Send + 'static>(mut stream: S) -> Arc<Mutex<Vec<u8>>> {
    let res = Arc::new(Mutex::new(Vec::new()));
    let vec = res.clone();
    thread::spawn(move || loop {
        if handle_byte(&mut stream, &vec) {
            break;
        }
    });
    res
}

#[cfg(target_os="windows")]
fn subcommand(cmd: &str) -> Option<Child> { 
    use std::os::windows::process::CommandExt;
    const DONT_CREATE_WINDOW: u32 = 0x08000000;
    Command::new("real_cmd.exe")
        .creation_flags(DONT_CREATE_WINDOW)
        .args(&["/C", cmd])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .ok()
}

#[cfg(not(target_os="windows"))]
fn subcommand(cmd: &str) -> Option<Child> {
    Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .ok()
}

fn subcommand_from_args(cmd: &str, args: &[String]) -> Option<Child> {
    if cfg!(target_os = "windows") {
        use std::os::windows::process::CommandExt;
        const DONT_CREATE_WINDOW: u32 = 0x08000000;
        Command::new(cmd)
            .creation_flags(DONT_CREATE_WINDOW)
            .args(args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .ok()
    }
    else {
        return None;
    }
}

impl SubProcess {
    pub fn from_args(mut args: Args) -> Option<Self> {
        args.next();
        let mut cmd = args.next().unwrap_or("".to_string());
        if cmd == "cmd" { 
            cmd = "real_cmd.exe".to_string();
        }
        else if cmd.to_ascii_lowercase().contains("c:\\windows\\system32\\cmd") {
            cmd = "real_cmd.exe".to_string()
        }
        let args = args.collect::<Vec<_>>();
        dbg!(&cmd, &args);
        let mut child = subcommand_from_args(&cmd, args.as_slice())?;
        Some(Self {
            stderr: child_non_blocking_stream(child.stderr.take()?),
            stdout: child_non_blocking_stream(child.stdout.take()?),
            child: child,
        })
    }

    pub fn from_cmd(cmd: &str) -> Option<Self> {
        let r = subcommand(cmd);
        let mut child = r?;
        Some(Self {
            stderr: child_non_blocking_stream(child.stderr.take()?),
            stdout: child_non_blocking_stream(child.stdout.take()?),
            child: child,
        })
    }

    pub fn kill(&mut self) {
        self.child.kill();
    }

    pub fn is_dead(&mut self) -> bool {
        self.child.try_wait().map(|e| e.is_some()).unwrap_or(true)
    }

    pub fn write_stdin(&mut self, byte: &[u8]) {
        let s = self.child.stdin.take();
        if let Some(mut stdin) = s {
            stdin.write(&byte);
            self.child.stdin = Some(stdin);
        }
    }

    pub fn get_bytes_stdout(&mut self) -> Vec<u8> {
        let mut delta = vec![];
        std::mem::swap(&mut *self.stdout.lock().unwrap(), &mut delta);
        delta
    }
 
    pub fn get_bytes_stderr(&mut self) -> Vec<u8> {
        let mut delta = vec![];
        std::mem::swap(&mut *self.stderr.lock().unwrap(), &mut delta);
        delta
    }
}

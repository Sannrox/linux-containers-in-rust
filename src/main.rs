use libc;
use libc::c_char;
use nix::sched::{self};
use nix::unistd;
use std::env;
use std::ffi::CString;
use std::io::{self, Write};
use std::os::unix::process::CommandExt;
use std::ptr;
use std::thread;
use std::time::Duration;
// use structopt::StructOpt;
use nix::sys::utsname;
use std::fs;
use std::path::Path;
use std::process;
use std::process::Command;
use subprocess::Exec;

// #[derive(StructOpt)]
// enum Command {
//     Info,
//     #[structopt(
//         about = "Plays specified room",
//         help = "USAGE: play MyRoomName"
//     )]
//     Play {
//     },
//     #[structopt(
//         about = "Pauses a specified room",
//         help = "USAGE: volume MyRoomName"
//     )]
//     Pause {
//     },
// }

struct ChildConfig {
    uid: u32,
    fd: u32,
    hostname: String,
    // args: Command,
    mount_dir: String,
}

fn main() {
    let config: ChildConfig = ChildConfig {
        uid: 0,
        fd: 0,
        hostname: String::from(""),
        // args: Command::from_args(),
        mount_dir: String::from(""),
    };

    let arg: Vec<String> = env::args().collect();

    check_linux_version();

    if arg.len() <= 1 {
        eprintln!("You need one argument");
        process::exit(1);
    }
    match arg[1].as_str() {
        "run" => run_clone(),

        "child" => child(),

        _ => eprintln!("NO OPTION"),
    }

    // let mut err: u32 = 0;
    // let mut option: u32 = 0;
    // let mut sockets = {};
    // let mut child_pid: u32 = 0;
    // let mut last_optind: u32 = 0;

    // match config.args {
    //     Command::Info => println!("C"),
    //     Command::Pause {} => println!("M"),
    //     Command::Play {} => println!("U"),
    // }
}

fn run_clone() {
    const STACK_SIZE: usize = 1024 * 1024;
    let ref mut stack: [u8; STACK_SIZE] = [0; STACK_SIZE];
    let test2: i8;
    let mut test: u8;
    let run_fn = Box::new(|| run());
    let mut flags: sched::CloneFlags = sched::CloneFlags::empty();
    flags.set(sched::CloneFlags::CLONE_NEWUTS, true);
    flags.set(sched::CloneFlags::CLONE_NEWPID, true);
    flags.set(sched::CloneFlags::CLONE_NEWNS, true);

    println!("{:?}", flags);

    sched::clone(run_fn, stack, flags, None).expect("clone error");
    // EPERM (No permission to set the scheduling policy and parameters specified in attr)
}

fn task() -> isize {
    thread::sleep(Duration::from_secs(2));
    0
}

fn check_linux_version() {
    println!("=> validating Linux version...");

    let host: utsname::UtsName = utsname::uname();

    if host.sysname() != "Linux" {
        eprintln!("Your are not using linux!");
        eprintln!(
            "You are using {} {} on {}",
            host.sysname(),
            host.release(),
            host.machine()
        );

        process::exit(1)
    }

    println!(
        "{} {} on {}",
        host.sysname(),
        host.release(),
        host.machine()
    )
}

fn run() -> isize {
    let mut cmd = Command::new("/proc/self/exe");
    cmd.args(&["child", "echo helloworld"]);
    let output = cmd.output().expect("failed to execute process");

    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();
    // println!("{:?}", output);
    0
}

fn child() {
    let proc = CString::new("proc").unwrap();
    let proc_str: *const c_char = proc.as_ptr();

    println!("{}", env::args().skip(1).next().unwrap());
    let mut cmd = Command::new(env::args().skip(1).next().unwrap());
    cmd.arg(env::args().skip(2).next().unwrap());
    cg();

    unistd::sethostname("container".to_string()).unwrap();
    unistd::chroot("/home/me/unbuntufs").expect("no such file or directory");
    unistd::chdir("/").unwrap();

    unsafe {
        libc::mount(proc_str, proc_str, proc_str, 0, ptr::null());
    }

    cmd.output().unwrap();

    unsafe {
        libc::umount(proc_str);
    }
}

fn cg() {
    let cgroups = Path::new("/sys/fs/cgroup/");
    let pids = Path::new("/pids");
    let pids_path = env::join_paths([cgroups, pids].iter());
    let cgroup_path = env::join_paths([pids,Path::new("me")].iter()).unwrap();
    if !Path::new(&cgroup_path).exists()
     {
        fs::create_dir(cgroup_path);
    };

    fs::write(
        env::join_paths([pids, Path::new("me/pids.max")].iter()).unwrap(),
        b"20",
    )
    .unwrap();
    fs::write(
        env::join_paths([pids, Path::new("me/notify_on_release")].iter()).unwrap(),
        b"1",
    )
    .unwrap();

    fs::write(
        env::join_paths([pids, Path::new("me/cgroups.procs")].iter()).unwrap(),
        unistd::getpid().as_raw().to_ne_bytes(),
    )
    .unwrap();
}

use std::env;
use std::io;
use nix::unistd;
use libc;
use libc::c_char;
use std::ptr;
use std::ffi::CString;
// use structopt::StructOpt;
use nix::sys::utsname;
use std::process;
use std::process::Command;
use std::fs;
use subprocess::Exec;
use std::path::Path;

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


    match arg[1].as_str() {

        "child" => child(),

        _=> eprintln!("NO OPTION")
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

fn check_linux_version(){
    println!("=> validating Linux version...");

    let host: utsname::UtsName = utsname::uname();

    if host.sysname() != "Linux"  {
        eprintln!("Your are not using linux!");
        eprintln!("You are using {} {} on {}", host.sysname(), host.release(), host.machine());

        process::exit(1)
    }

    println!("{} {} on {}", host.sysname(), host.release(), host.machine())

}

fn child(){
    let proc = CString::new("proc").unwrap();
    let proc_str: *const c_char = proc.as_ptr();

    let mut cmd = Command::new(env::args().skip(1).next().unwrap());
    cmd.arg(env::args().skip(2).next().unwrap());
    cg();

    unistd::sethostname( "container".to_string()).unwrap();
    unistd::chroot("/home/me/unbuntufs").unwrap();
    unistd::chdir("/").unwrap();



    unsafe{

    libc::mount(proc_str, proc_str, proc_str, 0, ptr::null());
    }

    cmd.output().unwrap();

    unsafe {

    libc::umount(proc_str);

    }

}

fn cg(){
    let cgroups = Path::new("/sys/fs/cgroup/");
    let pids = Path::new("/pids");
    let pids_path = env::join_paths([cgroups, pids].iter());
    fs::create_dir(env::join_paths([pids, Path::new("me")].iter()).unwrap()).unwrap();

    fs::write(env::join_paths([pids, Path::new("me/pids.max")].iter()).unwrap(),b"20").unwrap();
    fs::write(env::join_paths([pids, Path::new("me/notify_on_release")].iter()).unwrap(), b"1").unwrap();

    fs::write(env::join_paths([pids, Path::new("me/cgroups.procs")].iter()).unwrap(), unistd::getpid().as_raw().to_ne_bytes()).unwrap();

    }

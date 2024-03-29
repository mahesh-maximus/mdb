
use live_reload::LiveReload;
use mdbm::MdbExitCode;
use std::panic;
use std::env;

mod http_server_adapter;
mod ws_server_adapter;

fn main_executable() -> MdbExitCode {
    // Start firecracker by setting up a panic hook, which will be called before
    // terminating as we're building with panic = "abort".
    // It's worth noting that the abort is caused by sending a SIG_ABORT signal to the process.
    panic::set_hook(Box::new(move |info| {
        // We're currently using the closure parameter, which is a &PanicInfo, for printing the
        // origin of the panic, including the payload passed to panic! and the source code location
        // from which the panic originated.
        println!("MDB {}", info);
    }));

    //start_to_watch("/mdb/frontend".to_string());
    let mut live_reload = LiveReload::new("/mdb/frontend".to_string());
    live_reload.start_to_watch();

    ws_server_adapter::run_ws_server();
    http_server_adapter::run_web_server("main.py".to_string());


    MdbExitCode::Ok
}

fn main() {
    println!("Starting MDB ...");
    println!("Current Dir: {}", env::current_dir().unwrap().display());

    unsafe {
        // Harmless print to standard output.
        libc::syscall(
            libc::SYS_write,
            libc::STDOUT_FILENO,
            "Hello, world from Sys Call!\n",
            14,
        );
    }

    let exit_code = main_executable();

    std::process::exit(exit_code as i32);
}

#[cfg(test)]
mod main_tests {
    #[test]
    fn test_main() {
        assert!(true);
    }
}

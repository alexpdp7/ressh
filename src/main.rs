fn main() {
    let args: Vec<_> = std::env::args().collect();
    assert_eq!(args.len(), 2, "only argument should be host");
    let host = &args[1];
    loop {
        assert_eq!(
            std::process::Command::new("date")
                .status()
                .expect("cannot run date")
                .code()
                .expect("cannot run date"),
            0,
            "date failed"
        );
        let status = std::process::Command::new("ssh")
            .args(["-o", "ConnectTimeOut=5", host, "uptime"])
            .status()
            .expect("could not execute ssh");
        if status.success() {
            std::process::exit(0);
        }
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}

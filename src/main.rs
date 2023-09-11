use libc::rlimit;

fn test(i: i32) {
    let a = i * 2;
    if a > 10 {
        panic!("crash and burn")
    }
}

/// Enable core dumps to file by ensuring that the respective rlimit is
/// set correctly.
/// Note that we do not touch the name under which a core file is
/// created. At least on Linux that is a global property and we do not
/// want to change it for that reason.
fn enable_core_dumps() {
    let mut limit = rlimit {
        rlim_cur: 0,
        rlim_max: 0,
    };

    unsafe { libc::getrlimit(libc::RLIMIT_CORE, &mut limit) };

    // As an application we are only allowed to touch the soft limit
    // (`rlim_cur`), while the hard limit acts as a ceiling. We bump it
    // as high as we can.
    limit.rlim_cur = limit.rlim_max;

    // TODO: There is also setrlimit64. Find out what its deal is and
    //       whether we want/need it.
    unsafe { libc::setrlimit(libc::RLIMIT_CORE, &limit) };
}

fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");

    enable_core_dumps();

    let default_panic = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        default_panic(panic_info);
        // Don't forget to enable core dumps on your shell with eg `ulimit -c unlimited`
        let pid = std::process::id();
        eprintln!("dumping core for pid {}", std::process::id());
        use libc::kill;
        use libc::SIGQUIT;
        unsafe { kill(pid.try_into().unwrap(), SIGQUIT) };
    }));

    test(rand::random())
}

mod shell;

fn main() {
    // foo01();
    // foo02(0o666); // Octal starts with 0o
    // let sh = shell::Shell::new();
    // sh.run();
    foo04();
}

fn foo01() {
    unsafe {
        let pid = libc::fork();
        if pid < 0 {
            panic!("invalid pid: {}", pid);
        }
        if pid == 0 {
            println!(
                "child process. pid: {}, ppid: {}",
                libc::getpid(),
                libc::getppid()
            );
            libc::exit(2)
        }
        if pid > 0 {
            let mut status: libc::c_int = 0;
            libc::waitpid(pid, &mut status, 0);
            let s = libc::WEXITSTATUS(status);
            println!(
                "parent process. pid: {}, child: {}. child exit with code: {}",
                libc::getpid(),
                pid,
                s
            );
        }
    }
}

fn foo02(permission: i32) {
    unsafe {
        let new_mask = 0;
        // should set umake to 0 before create the file
        let old_mask = libc::umask(new_mask);
        let fd = libc::open(
            format!("./foo{:03o}.txt", permission).as_ptr() as *const i8,
            libc::O_CREAT | libc::O_RDWR | libc::O_TRUNC,
            permission,
        );
        let new_mask = old_mask;
        let old_mask = libc::umask(new_mask);
        if nix::errno::errno() != 0 {
            log::error!("{}", std::io::Error::last_os_error());
        }
        let count = libc::write(
            fd,
            "before fork\n".as_ptr() as *const core::ffi::c_void,
            "before fork\n".len(),
        );
        if count as usize != "before fork".len() || nix::errno::errno() != 0 {
            log::error!(
                "errno: {}, count: {}",
                std::io::Error::last_os_error(),
                count
            );
        }

        let pid = libc::fork();
        if pid < 0 {
            panic!("invalid pid: {}", pid);
        }
        if pid == 0 {
            let data = format!(
                "I'm child. My pid is {}, and my parent's pid is {}\n",
                libc::getpid(),
                libc::getppid()
            );
            let count = libc::write(fd, data.as_ptr() as *const core::ffi::c_void, data.len());
            if count as usize != data.len() || nix::errno::errno() != 0 {
                log::error!(
                    "errno: {}, count: {}",
                    std::io::Error::last_os_error(),
                    count
                );
            }
        }
        if pid > 0 {
            let mut status: libc::c_int = 0;
            libc::waitpid(pid, &mut status, 0);
            let data = format!(
                "I'm parent. My pid is {}, and my child's pid is {}\n",
                libc::getpid(),
                pid
            );
            // println!("{}", data);
            let count = libc::write(fd, data.as_ptr() as *const core::ffi::c_void, data.len());
            if count as usize != data.len() || nix::errno::errno() != 0 {
                log::error!(
                    "errno: {}, count: {}",
                    std::io::Error::last_os_error(),
                    count
                );
            }
        }
        libc::close(fd);
        if nix::errno::errno() != 0 {
            log::error!("close fd with error: {}", nix::errno::errno());
        }
        println!("exit: {}", libc::getpid());
    }
}

fn foo04() {
    unsafe {
        let pid = dbg!(libc::fork());
        if pid < 0 {
            panic!(
                "invalid pid: {}, error: {:?}",
                pid,
                std::io::Error::last_os_error()
            );
        }
        if pid == 0 {
            println!("before close stdout");
            let result = dbg!(libc::close(libc::STDOUT_FILENO));
            if dbg!(result != 0) {
                println!("failed to close stdout. error: {}", nix::errno::errno(),);
                return;
            }
            println!("closed stdout successfully");
            let new_mask = 0;
            let old_mask = dbg!(libc::umask(new_mask));
            let fd = dbg!(libc::open(
                format!("./stdout.txt").as_ptr() as *const i8,
                libc::O_CREAT | libc::O_RDWR | libc::O_TRUNC,
                0o666,
            ));
            if dbg!(fd < 0) {
                dbg!(std::io::Error::last_os_error());
                return;
            }
            if dbg!(fd != libc::STDOUT_FILENO) {
                panic!(
                    "invalid fd: {}, error: {:?}",
                    fd,
                    std::io::Error::last_os_error()
                );
            }
            dbg!(libc::umask(old_mask));
            let result = dbg!(libc::execl(
                format!("/bin/ls\0").as_ptr() as *const i8,
                format!("ls\0").as_ptr() as *const i8,
                format!("-G\0").as_ptr() as *const i8,
                std::ptr::null::<i8>(),
            ));
            if result != 0 {
                dbg!(std::io::Error::last_os_error());
            }
            dbg!(libc::close(libc::STDOUT_FILENO));
        }
        if pid > 0 {
            let mut status: libc::c_int = 0;
            libc::waitpid(pid, &mut status, 0);
        }
    }
}

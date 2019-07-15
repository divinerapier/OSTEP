fn main() {
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
        }
        if pid > 0 {
            let mut status: libc::c_int = 0;
            libc::waitpid(pid, &mut status as *mut libc::c_int, 0);
            println!("parent process. pid: {}, child: {}", libc::getpid(), pid);
        }
    }
}

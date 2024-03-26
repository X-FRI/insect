use nix::errno::Errno;

pub fn error(desc: &str, errno: Errno) {
    let msg = format!("{}: {}", desc, errno.desc());
    error!("{}", msg);
}

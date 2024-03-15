use nix::errno::Errno;

pub fn exit(desc: &str, errno: Errno) {
    let msg = format!("{}: {}", desc, errno.desc());
    error!("{}", msg);
    panic!("{}", msg)
}

use std::{env, process::Command};

type Callback = fn();

pub fn sleeper(ms: u32) -> std::process::Child {
    let mut me = env::current_exe().unwrap();

    me.pop();

    if me.ends_with("deps") {
        me.pop();
    }

    me.push("sleep");

    let cmd = Command::new(me).arg(ms.to_string()).spawn();

    match cmd {
        Ok(e) => e,
        Err(e) => panic!("{} failed with {}", stringify!(cmd), e),
    }
}

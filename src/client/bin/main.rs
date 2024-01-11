// https://rust-cli.github.io/book/tutorial/cli-args.html

use std::{
    env, fs,
    io::Write,
    os::unix::fs::{MetadataExt, PermissionsExt},
    time::{SystemTime, UNIX_EPOCH},
};

fn main() {
    let exec_file = match std::env::args().nth(1) {
        None => {
            println!("not enough args");
            return;
        }
        Some(a) => a,
    };

    let exec_file_stat = match fs::metadata(&exec_file) {
        Err(err) => match get_from_path(&exec_file) {
            Some(m) => m,
            None => {
                println!("get exec_file metadata error: {:?}", err);
                return;
            }
        },
        Ok(m) => m,
    };

    if exec_file_stat.permissions().mode() & 0o111 == 0 {
        println!(
            "exec_file is not executable, mode {:#o}, {}",
            exec_file_stat.mode(),
            exec_file_stat.mode() & 0o111
        );
        return;
    }

    let args_parsed: Vec<String> = std::env::args().skip(2).collect();

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    let command_file = format!("./requests/{}-{}.yaml", &exec_file, &now);
    let out_file = format!("./output/{}-{}.log", &exec_file, &now);

    let req = types::Request {
        file_name: "".to_string(),
        command: exec_file,
        args: args_parsed,
        log_file: out_file,
    };

    let data_yaml = serde_yaml::to_string(&req).unwrap();

    let mut f = fs::File::create(command_file).unwrap();
    f.write_all(data_yaml.as_bytes()).unwrap();

    println!("{:?}", req);
}

// is_program_in_path: https://stackoverflow.com/questions/35045996/check-if-a-command-is-in-path-executable-as-process
fn get_from_path(program: &str) -> Option<std::fs::Metadata> {
    if let Ok(path) = env::var("PATH") {
        for p in path.split(':') {
            let p_str = format!("{}/{}", p, program);
            if let Ok(m) = fs::metadata(p_str) {
                return Some(m);
            }
        }
    }
    None
}

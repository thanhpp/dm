use std::path;

use tokio::{
    fs::{self, OpenOptions},
    io::AsyncWriteExt,
    process::Command,
};

pub struct Executor {
    command: String,
    args: Vec<String>,
    stdout_path: String,
    stderr_path: String,
}

impl Executor {
    pub fn new(
        command: String,
        args: Vec<String>,
        stdout_path: String,
        stderr_path: String,
    ) -> Executor {
        Executor {
            command,
            args,
            stdout_path,
            stderr_path,
        }
    }

    pub async fn run(&self) -> anyhow::Result<()> {
        let output = Command::new(&self.command)
            .args(&self.args)
            .output()
            .await
            .map_err(|err| anyhow::format_err!("execute command error: {:?}", err))?;

        self.write_log(&self.stdout_path, &output.stdout).await?;
        self.write_log(&self.stderr_path, &output.stderr).await?;

        Ok(())
    }

    async fn write_log(&self, log_path: &str, data: &[u8]) -> anyhow::Result<()> {
        let p = path::Path::new(log_path);
        let mut file_exist: bool = false;

        match fs::metadata(p).await {
            Ok(_) => file_exist = true,
            Err(_) => {
                println!("file {} not exist", log_path);
            }
        };
        if !file_exist {
            if let Some(parent) = p.parent() {
                fs::create_dir_all(parent)
                    .await
                    .map_err(|err| anyhow::format_err!("create dir all error: {:?}", err))?;
            }
        }

        let mut f = OpenOptions::new()
            .append(true)
            .create(true)
            .open(log_path)
            .await
            .map_err(|err| anyhow::format_err!("open options error {:?}", err))?;
        f.write_all(data)
            .await
            .map_err(|err| anyhow::format_err!("write all error: {:?}", err))?;
        f.sync_all()
            .await
            .map_err(|err| anyhow::format_err!("sync all error: {:?}", err))?;

        Ok(())
    }
}

mod test {
    #[tokio::test]
    async fn test_executor() {
        let e = crate::executor::Executor::new(
            "go".to_string(),
            vec!["env".to_string()],
            "/home/thanhpp/dm/logs/go_env_stdout.log".to_string(),
            "/home/thanhpp/dm/logs/go_env_stderr.log".to_string(),
        );

        if let Err(err) = e.run().await {
            println!("execute error: {:#?}", err);
        }
    }

    #[tokio::test]
    async fn test_executor_1() {
        let e = crate::executor::Executor::new(
            "go".to_string(),
            vec!["env".to_string()],
            "/home/thanhpp/dm/logs/go_env_stderr.log".to_string(),
            "/home/thanhpp/dm/logs/go_env_stderr.log".to_string(),
        );

        if let Err(err) = e.run().await {
            println!("execute error: {:#?}", err);
        }
    }
}

use std::{fs, time::Duration};

use types::Request;

mod executor;

#[tokio::main]
async fn main() {
    let m = Manager::new("requests");
    m.run().await;
}

#[derive(Clone)]
struct Manager {
    request_folder: String,
}

impl Manager {
    fn new(folder: &str) -> Self {
        Manager {
            request_folder: folder.to_string(),
        }
    }

    async fn run(&self) {
        let m = self.clone();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(1));

            loop {
                interval.tick().await;

                let reqs = match m.read_request() {
                    Err(err) => {
                        println!("read request error: {:?}", err);
                        continue;
                    }
                    Ok(r) => r,
                };

                let mut futures = Vec::new();
                for r in reqs {
                    let a_request = r;
                    futures.push(tokio::spawn(async move {
                        Self::execute(&a_request).await;
                    }))
                }

                for f in futures {
                    f.await.unwrap();
                }
            }
        })
        .await
        .unwrap();
    }

    fn read_request(&self) -> anyhow::Result<Vec<Request>> {
        let dir = fs::read_dir(&self.request_folder)?;

        let mut requests: Vec<Request> = Vec::new();
        for e in dir.filter_map(|e| match e {
            Err(_) => None,
            Ok(e) => Some(e),
        }) {
            match e.file_type() {
                Ok(t) => {
                    if !t.is_file() {
                        continue;
                    }
                    match fs::read_to_string(e.path()) {
                        Err(err) => {
                            println!("read file error: {:?}, file: {:?}", err, e.path());
                            continue;
                        }
                        Ok(s) => match serde_yaml::from_str::<Request>(&s) {
                            Err(err) => {
                                println!("parse yaml error: {:?}, file: {:?}", err, e.path());
                                continue;
                            }
                            Ok(mut r) => {
                                r.file_name = e.path().into_os_string().into_string().unwrap();
                                requests.push(r);
                            }
                        },
                    }
                }
                Err(_) => continue,
            }
        }

        for r in requests.iter() {
            println!("{:?}", r);
        }

        Ok(requests)
    }

    async fn execute(req: &Request) {
        let request_path = std::path::Path::new(&req.file_name);
        let output_path = std::path::Path::new(&req.log_file);
        let output_file = match output_path.file_name() {
            None => req.log_file.clone(),
            Some(f_name) => match f_name.to_str() {
                None => req.log_file.clone(),
                Some(f_name) => {
                    format!(
                        "{}/{}-{}",
                        req.log_file.clone().trim_end_matches(f_name),
                        request_path
                            .file_name()
                            .unwrap_or_default()
                            .to_str()
                            .unwrap_or_default()
                            .trim_end_matches(".yaml"),
                        f_name,
                    )
                }
            },
        };

        let e = executor::Executor::new(
            req.command.clone(),
            req.args.clone(),
            output_file.clone(),
            output_file,
        );

        if let Err(err) = e.run().await {
            println!("execute error {:?}", err)
        };
        if let Err(err) = fs::remove_file(&req.file_name) {
            println!("remove file error {:?}", err)
        };
    }
}

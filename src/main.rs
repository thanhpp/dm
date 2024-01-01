use std::fs;

use serde::Deserialize;

mod executor;

#[tokio::main]
async fn main() {
    let m = Manager::new("requests");
    let req = m.read_request().unwrap();
    for r in req.iter() {
        m.execute(r).await;
    }
}

struct Manager {
    request_folder: String,
}

impl Manager {
    fn new(folder: &str) -> Self {
        Manager {
            request_folder: folder.to_string(),
        }
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

    async fn execute(&self, req: &Request) {
        let e = executor::Executor::new(
            req.command.clone(),
            req.args.clone(),
            req.log_file.clone(),
            req.log_file.clone(),
        );

        e.run().await.unwrap();
    }
}

#[derive(Deserialize, Debug)]
struct Request {
    #[serde(skip)]
    file_name: String,
    command: String,
    args: Vec<String>,
    log_file: String,
}

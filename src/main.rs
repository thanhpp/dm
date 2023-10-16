use tokio::process::Command;

mod executor;

#[tokio::main]
async fn main() {
    tokio::spawn(async move {
        let output = Command::new("go").arg("env").output().await.unwrap();

        println!("{}", String::from_utf8(output.stdout).unwrap());
    })
    .await
    .unwrap();
}

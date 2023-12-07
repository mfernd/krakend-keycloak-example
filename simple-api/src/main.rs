use uuid::Uuid;

#[tokio::main]
async fn main() {
    let mut apps = Vec::new();
    let interface = Some("0.0.0.0");

    for i in 0..=1 {
        let port = 3000 + i;
        let uuid = Uuid::new_v4();
        apps.push(simple_api::create_app(interface, port, uuid).await);
    }
    let app_count = apps.len();

    let mut servers = Vec::new();
    for app in apps {
        if let Ok(app) = app {
            servers.push(tokio::spawn(async move { app.await.unwrap() }));
        } else {
            eprintln!("Failed to create app");
        }
    }

    println!("Starting {app_count} app(s)...");
    for server in servers {
        let _ = server
            .await
            .map_err(|e| println!("Failed to launch an app... {:?}", e));
    }
}

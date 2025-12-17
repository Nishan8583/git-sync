use dotenvy::dotenv;
use std::{env, process::Command};

#[tokio::main]
async fn main() {

    // We are taking the directory to store repositories as a command line argument
    let args : Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <directory_to_store_repos>", args[0]);
        std::process::exit(1);
    }

    // If the directory does not exist, create it
    if std::path::Path::new(&args[1]).exists() == false {
        std::fs::create_dir_all(&args[1]).expect("Failed to create directory");
    }

    // Load environment variables from .env file
    dotenv().unwrap();
    let token = env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN not found in .env");

    // Create octocrab instance with the token
    let oct = octocrab::OctocrabBuilder::new()
        .personal_token(token)
        .build()
        .unwrap();

    let user = oct.current().user().await.unwrap().login;
    println!("Authenticated as {}", user);

    // list the repos available for the authenticated user
    let mut stream = oct
        .current()
        .list_repos_for_authenticated_user()
        .per_page(100)
        .send()
        .await
        .into_iter();

        while let Some(repos) = stream.next() {
            for repo in repos {
                let owner = repo.owner.unwrap();
                let forked = repo.fork.unwrap();
                // we ignore forked project and the repos which are not owned by us
                if owner.login != user  || forked {
                    continue;
                }
                let path = format!("{}/{}",args[1], repo.name);
                if let Some(url) = repo.clone_url {
                    if let Err(e) = clone_or_pull(&path,url.as_str()) {
                    eprintln!(
                        "Failed to clone or pull repository {}: {}",
                        repo.name, e
                    );
                    }
                } else {
                    eprintln!("Repository {} does not have a git URL", repo.name);
                }
            }
        }
}

// To clone or pull a repository we use git command line tool instead of crate, as it is simpler
// available crates seems to make things complicated
fn clone_or_pull(path: &str, url:&str) -> std::io::Result<std::process::ExitStatus> {
    if std::path::Path::new(path).exists() {
        println!("Pulling updates in {}", path);
        Command::new("git")
            .arg("-C")
            .arg(path)
            .arg("pull")
            .status()
    } else {
        println!("Cloning {} into {}", url, path);
      
        Command::new("git")
            .arg("clone")
            .arg(url)
            .arg(path)
            .status()
    }

}


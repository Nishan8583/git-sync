# git-sync
A really really really simple tool to backup all remote github repositories locally. It only clones/pulls repos that is owned by a user that owns personal token inserted in .env file.
For cloning and pulling, it expects git to be installed and authenticated. Cloning and pulling using crate seems unnecessarily complicated using available crates.

### Usage
1. Create a `.env` file with your Personal Access Token. Content of the file should look something like this:
`GITHUB_TOKEN=your_token_here`
2. Have SSH instaleed and authenticated.
3. `cargo run ./dest_folder`
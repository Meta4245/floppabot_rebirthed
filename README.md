# FloppaBot Rebirthed
this is my own personal bot made in rust <br />
deployed with [shuttle](https://www.shuttle.rs/) and is what i use <br />
# Setup Tutorial
make a shuttle project, remove all the boilerplate and put in this code <br />
add the file `src/secrets.rs` and add the following in it substituting the default values with your values
```rust
pub static DISCORD_TOKEN: &'static str = "No";
pub static CAT_API_KEY: &'static str = "Nuh uh";
pub static DOG_API_KEY: &'static str = "Replace it";
```
# Running
run `deploy.bat` or `deploy` after logging into shuttle
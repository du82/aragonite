# Aragonite on Tor (Hiddenite)
## PRE-REQS
- See `Tor Hidden Service Setup` SiYuan documentation for full initial setup instructions.
- Nightly Rust installation.
- Git installed on Debian Server.
- Tor hidden services setup.
- Clone the Aragonite repository.

## Setup
1] `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
2] `apt install cargo`
3] Switch to root user, install `sudo`
4] `sudo visudo` and give `user` privileges. Then `exit` to home/user.
5] `cd aragonite` & `mkdir index`
6] `sudo apt-install build-essential`
7] `sudo chown -R $USER:$USER /home/user/aragonite`
8] `cargo run`
9] Check Terminal to find port Rocket launched on after successful run.

Final] Check that the .onion is working and looking for the correct port. Go to the URL in Tor, and prime the index with an initial search query.

## Automatic updating (Git supply chain)

Tbd.

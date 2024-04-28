# Reminder: Update Config.toml before launch

# AWS Interface
Create instance 
    -> Amazon Linux

Security 
    -> Open HTTP/HTTPS
    -> Open porn 12345 [temporary]
    -> Set inbound rules

# Shell Commands
Run the following commands in the following order to get a functional instance installed and ready for staging.

Install Git and clone the project\
`sudo yum install git -y`\

# Access Private Repo:
1) https://github.com/settings/tokens
2) Click -> Personal access tokens (classic) -> Generate new token
3) Select `Repo` when setting scope
4) optional: Fork Repo if necessary
5) Sync Fork inside GitHub GUI if necessary

`sudo git clone https://username:[Github-Token]@github.com/username/rust-login-page`

Install Rust lang and configure build env\
`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`\
    -> proceed with standard installation
`sudo dnf install make automake gcc gcc-c++ kernel-devel`\
`exit`
    -> return to conssole
`rustup override set nightly`

Set ownership of the directory you created\
`sudo chown -R ec2-user /home/ec2-user/rust-login-page/`

Install neofetch (optional)\
`cd /tmp`\
`git clone https://github.com/dylanaraps/neofetch.git`\
`sudo cp neofetch/neofetch /usr/bin/neofetch`\
`rm -rf neofetch`\
`neofetch`

Make the index directory\
`cd rust-login-page`\
`mkdir index`

Run the project with the staging config\
`ROCKET_ENV=staging cargo run`

# Persistant Instance

Run before booting up an instance you want to keep online.

`screen -S session_name`
    -> this names a virtual terminal session that persists after you logout
`screen -r session_name`
    -> Reattach/Return to your session

[*note]
1) If you forgot to give the session a name, run `screen -r`
2) If you forgot to give the session a name, and multiple sessions are running, run `screen -r pid.tty.host`'
3) To view `pid.tty.host` run `screen -ls`

# Updating the Repo

`cd repo_name`
`git pull`

# Enable Neofetch for Mirror

`nano Config.toml`
    -> `neofetch_enabled = true`

# Mirror Domains

1) Copy Public IP from EC2 Instance
2) Go to Route 53 Dashboard -> `Hosted zone`
3) -> `Hosted zone name`
4) Click -> `Create record`
5) Set record type `A`, include name, paste in public IP
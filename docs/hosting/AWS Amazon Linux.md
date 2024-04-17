# AWS Interface
Create instance 
    -> Amazon Linux

Security 
    -> Open ports / set inbound rules

# Shell Commands
Run the following commands in the following order to get a functional instance installed and ready for staging.

Install Git and clone the project\
`sudo yum install git -y`\

Access Private Repo:
1) https://github.com/settings/tokens
2) Click -> Personal access tokens (classic) -> Generate new token
3) Select `Repo` when setting scope
4) optional: Fork Repo if necessary

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

# Updating the Repo

You can run `git pull` directly inside of the EC2 instance

# Mirror Domains

1) Copy Public IP from EC2 Instance
2) Go to Route 53 Dashboard -> `Hosted zone`
3) -> `Hosted zone name`
4) Click -> `Create record`
5) Set record type `A`, include name, paste in public IP
# AWS Interface
Create instance 
    -> Amazon Linux

Security 
    -> Open ports / set inbound rules

# Shell Commands
Run the following commands in the following order to get a functional instance installed and ready for staging.

Install Git and clone the project\
`sudo yum install git -y`\
`sudo git clone https://github.com/du82/rust-login-page`\

Install Rust lang and configure build env\
`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`\
`sudo dnf install make automake gcc gcc-c++ kernel-devel`\
`rustup override set nightly`\

Set ownership of the directory you created\
`sudo chown -R ec2-user /home/ec2-user/rust-login-page/`\

Install neofetch (optional)\
`cd /tmp`\
`git clone https://github.com/dylanaraps/neofetch.git`\
`sudo cp neofetch/neofetch /usr/bin/neofetch`\
`rm -rf neofetch`\
`neofetch`\

Make the index directory\
`cd rust-login-page`\
`mkdir index`\

Run the project with the staging config\
`ROCKET_ENV=staging cargo run`\

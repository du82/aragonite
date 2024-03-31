- Create instance
- Security -> Open ports / set inbound rules

#---------- Commands Needed ----------

1. `sudo yum install git -y`
2. `sudo git clone https://github.com/du82/rust-login-page`
3. `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
4.  `sudo dnf install make automake gcc gcc-c++ kernel-devel`
5.  `rustup override set nightly`
6.  `ROCKET_ENV=staging cargo run`
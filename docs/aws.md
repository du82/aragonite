#---------- AWS Interface ----------

Create instance 
    -> Amazon Linux

Security 
    -> Open ports / set inbound rules

#---------- AWS Terminal ----------

sudo yum install git -y

sudo git clone https://github.com/du82/rust-login-page

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

sudo dnf install make automake gcc gcc-c++ kernel-devel

rustup override set nightly

sudo chown -R ec2-user /home/ec2-user/rust-login-page/

cd /tmp
git clone https://github.com/dylanaraps/neofetch.git
sudo cp neofetch/neofetch /usr/bin/neofetch
rm -rf neofetch
neofetch

ROCKET_ENV=staging cargo run

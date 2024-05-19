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

# ---------------Maintenance---------------

# Persistant Instance
`screen -S session_name` (make)
`screen -r session_name` (retrieve)
`screen -ls` (list)
`screen -X -S session_name quit` (kill)

# Updating the Repo
kill the screen
`cd rust-login-page`
`git reset --hard`
`git fetch origin`
`git pull`
    -> `(D) Delete` if warning shows up
press `i`
press `esc`
type `:wq` => press Enter
`git push`
`clear`
`cd rust-login-page`
`rm -r index`
`mkdir index`
`nano Config.toml`
    -> `neofetch_enabled = true`
    -> `mirror_updated = "correct_date"`
`cd src`
`nano search.rs`
ln 87 manually add {mirror_update}, save&exit    
`clear`
make new screen
`ROCKET_ENV=staging cargo run`
enter test query to index (better UX for end user)  

# ---------------Domains---------------

1) Copy Public IP from EC2 Instance
2) Go to Route 53 Dashboard -> `Hosted zone`
3) -> `Hosted zone name`
4) Click -> `Create record`
5) Set record type `A`, include name, paste in public IP

# GitHub Ed25519 Deployment Key forceful-throat-fuck-update

Do this inside of AWS to avoid the headache. Don't CD into any repo, it makes it more complicated

1) `ssh-keygen -t ed25519 -C "your_email@example.com"`
    -> Leave it all blank.
2) `ssh-keygen -t ed25519 -C "your_email@example.com"`
3) Go to https://github.com/[USERNAME]/rust-login-page/settings/keys/new
4) Slap that pussy bitch into the `Key` section && `Chekcbox` allow write accesss
5) Run `ssh -T git@github.com` in AWS to see if it's working
    -> it will [SHIT] it self and take a couple of seconds. Do nothing
6) `Are you sure you want to continue connecting (yes/no/[fingerprint])?` type in `yes` hit enter
7) It's still not going to work. run `git remote -v`
8) `git remote set-url origin git@github.com:[USERNAME]/[REPO-NAME].git`
9) run `git remote -v` again to ensure the formatting is correct.
    -> Output should look like this:
        `origin  git@github.com:Garrett96/rust-login-page.git (fetch)`
        `origin  git@github.com:Garrett96/rust-login-page.git (push)`
10) Do nothing. Just give it a minute. It's a little retarded.
11) Run `git fetch origin` and if it works, good job, if not, good luck.
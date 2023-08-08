# git-update-protect-hook

A simple example of how to make a update hook to protect a single file in a git repo from overwrites

Currently it is designed to protect only a single file in the root of the repository. Patches are welcomed. The itch that I scratched was that I wanted to disallow changes to .gitlab-ci.yml in one of my repos. 

## How to setup

Setup the binary to ba called as update hook https://git-scm.com/docs/githooks/2.27.0

When the hook is called it looks for a single line from a file with the same name as the binary with .lock extension.  
The line is matched against all blobs in root tree and if change is detected a error code is returned thus rejecting the commit  

Simply put:
* symlink the binary to `<repo_dir>.git/hooks/update`
* echo 'file_to_protect' > <repo_dir>.git/update.lock

## I know there is more to be done. Patches are welcomed!
* allow multiple files
* allow non-root files
* allow external disable somehow

Have fun!

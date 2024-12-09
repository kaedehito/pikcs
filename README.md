# pikcs
A package manager that builds and installs programs from github repositories


# How to install
```sh
git clone https://github.com/kaedehito/pikcs
cd pikcs
./install.sh
```
This script creates a .pikcs directory in your home directory and adds a path to pikcs to .bashrc and .zshrc

# How to use 

## update the packagelist
```sh
pikcs update <username>
```
Clone `https://github.com/<username>/packagelist` and update the package list

If there are no arguments, clone the previously cloned package list repository again

## install the package
```sh
pikcs install <package>
```
Install the specified package. 
Checks if the package specified in packagelist exists, and if so, clones the repository, builds, and installs it. If it does not exist, output an error message and exit.

## remove the package
```sh
pikcs remove <package>
```
removes the specified package

If the package was installed, this program runs the uninstall script and removes the package's executable file. 
If it does not exist, output an error message and exit


## search the package
```sh
pikcs search <package>
```

Checks if the specified package exists 

If it does not exist, output an error message and exit.

If it exists, print the package information and exit

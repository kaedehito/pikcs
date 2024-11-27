printf "checking cargo..."
which cargo > /dev/null
echo "ok"
printf "checking rustc..."
which rustc > /dev/null
echo "ok"
printf "checking pkg-config..."
which pkg-config > /dev/null
echo "ok"
printf "checking openssl-dev..."
if pkg-config --modversion openssl > /dev/null; then
	echo "ok"
 else
 	printf "\nopenssl development library is not installed. please install libssl-dev\n"
  	exit 1
 fi


if [ -e "$HOME/.bashrc" ]; then
	echo "export PATH=\$PATH:\$HOME/.pikcs/bin" >> ~/.bashrc
fi

if [ -e "$HOME/.zshrc" ]; then
	printf "export PATH=\$PATH:\$HOME/.pikcs/bin" >> ~/.zshrc
fi

cd pikcs/main/

if cargo build --release; then
	if ./target/release/pikcs update kaedehito; then
		mv ./target/release/pikcs $HOME/.pikcs/bin
		echo "install is end!"	
		exit 0
	fi
fi

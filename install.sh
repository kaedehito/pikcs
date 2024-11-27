sudo true

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

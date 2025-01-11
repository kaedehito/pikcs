# Pikcs: A Lightweight Package Manager for Developers

Pikcs is a simple and efficient package manager designed to allow developers to share and install packages hosted on GitHub with minimal setup. It aims to empower individual developers by streamlining the process of distributing their tools and libraries.


---

## Features

- Customizable: Install packages from your own GitHub repositories.

- Easy Integration: Just add a packagelist.json to your repository.

- Dependency Management: Specify required tools and dependencies.

- Cross-Language Support: Works with Rust, C, and more.

- Flexible Build System: Define build, install, and removal steps using .build files.



---

## How It Works

1. Create a packagelist repository in your GitHub account.


2. Add a packagelist.json file with the packages you want to share.


3. Each package repository must include a .build file to define build, install, and removal instructions.


4. Use Pikcs to install, manage, and remove packages easily.




---

### Example: packagelist.json

```json
{
  "version": 1,
  "user": "kaedehito",
  "list": [
    {
      "name": "cocoa",
      "url": "https://github.com/kaedehito/cocoa",
      "license": "MIT",
      "language": "Rust",
      "desc": "Modern shell made of rust",
      "dependencies": ["cargo", "rustc"],
      "version": "1.0.0"
    },
    {
      "name": "qase",
      "url": "https://github.com/kaedehito/qase",
      "license": "MIT",
      "language": "C",
      "desc": "Quick Audio Stream Emulator",
      "dependencies": ["gcc", "cmake", "pkg-config"],
      "version": "1.0.0"
    }
  ]
}

```


### Example: .build

```json
{
  "build": [
    "cargo build --release"
  ],
  "install": [
    "cargo run --release",
    "mv target/release/rfoc ."
  ],
  "remove": [
    "rm -rf ~/.config/rfoc/"
  ]
}
```

---

# Getting Started

## Installation

Clone the Pikcs repository and build it:
```
git clone https://github.com/kaedehito/pikcs
cd pikcs
./install.sh
```

## Usage

Install a package:

```
pikcs install <package_name>
```

Remove a package:

```
pikcs remove <package_name>
```

Update the package list:

```
pikcs update
```


---

## Contributing

We welcome contributions to Pikcs! If you have ideas for improvement, feel free to submit issues or pull requests.


---

## License

Pikcs is licensed under the MIT License. See the LICENSE file for details.


---

## Contact

If you have any questions or suggestions, please contact kaedehito.


# RustyShield: Password Strength Tester
A simple CLI tool written in Rust to evaluate password strength based on length, lowercase, uppercase, digit---
## ■ Table of Contents
- [About](#about)
- [Features](#features)
- [Requirements](#requirements)
- [Installation](#installation)
- [Usage](#usage)
- [Configuration](#configuration)
- [Contributing](#contributing)
- [License](#license)
- [Contact](#contact)
---
## About
RustyShield helps you quickly check how strong your password is, scoring up to 5 points and offering sugge---
## Features
- Checks password length (≥ 8 characters)
- Detects presence of lowercase letters
- Detects presence of uppercase letters
- Detects presence of digits
- Detects presence of special symbols
- Classifies strength as Weak, Moderate, or Strong
- Provides improvement suggestions for non-strong passwords
---
## Requirements
- Rust (1.56+)
- Cargo
- `regex` crate (v1)
- `colored` crate (v2)
---
## Installation
```bash
# Clone the repo
git clone https://github.com/code-by-sahib/RustyShield-Password-Strength-Tester.git
# Change directory
cd RustyShield-Password-Strength-Tester
# (Optional) Update dependencies
cargo update
```
---
## Usage
Run the program via Cargo:
```bash
cargo run
```
Enter your password when prompted:
```text
RustyShield: Password Strength Tester
Enter a password to check its strength:
> P@ssw0rd123
Password strength: Strong (Score: 5/5)
```
---
## Configuration
No additional configuration is required. All dependencies are managed via `Cargo.toml`.
---
## Contributing
Contributions are welcome! Please follow these steps:
1. Fork the repository.
2. Create your feature branch: `git checkout -b feature/my-feature`
3. Commit your changes: `git commit -m "Add my feature"`
4. Push to the branch: `git push origin feature/my-feature`
5. Open a pull request.
---
## License
This project is released under the MIT License. See [LICENSE](LICENSE) for details.
---
## Contact
GitHub: [code-by-sahib](https://github.com/code-by-sahib)

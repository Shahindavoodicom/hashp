

Strong Password Generator
----------------------------------------
This command-line program generates strong passwords that are easy to remember but not simplistic.

Installation and Usage
----------------------------------------
Rename sample.env to .env.
Replace the placeholder secret in the .env file with your own secret.
Run the program using Cargo:
```bash
Copy code
cargo run
```
Or, for a system-wide installation on Linux:

```bash
Copy code
cargo build --release
sudo cp target/release/your_program_name /usr/local/sbin
```
Now you can run the program from anywhere

How It Works
This program combines the input provided by the user with the secret key to create a unique password. By doing so, you no longer need to memorize your passwords. However, please note that this does not guarantee absolute security. It's recommended to change your secret key periodically for added safety.


for quite program input q and press inter
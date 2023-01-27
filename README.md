# Let's start playing with Rust
## _One ,Two,Three .. Take rust ;)_





## Making the playground (For Windows)

- Install   [Visual Studio](https://code.visualstudio.com/download)
- Install   [Install Visual C++ build tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/)
- Install [Rust](https://rustup.rs/)

## Hello World

# Create a new directory to organize your code
Start by creating a directory for storing all the code in this learning path (rust-learning-path), and then creating a new subdirectory to keep the source code for this exercise.
For the Windows command prompt, run the following commands:

```sh
mkdir "%USERPROFILE%\rust-learning-path"
cd /d "%USERPROFILE%\rust-learning-path"
mkdir hello-world
cd hello-world
```

# Write your first Rust program
Next, create a new file named main.rs and use your editor to write the following code into it:
```sh
fn main() {
	println!("Hello, world!");
}
```
## Compile and run your program
Your source code is ready. Now it's time to compile your program into an executable file. Return to your terminal window and enter the following commands to compile and run the file.
 run the following commands:
```sh
rustc main.rs
.\main.exe
```
You should see the following output:
```sh
Hello, world!
```
# Create a project with Cargo
Now let's use Cargo to write and run the same program.

> Note: The commands in the following sections work on all platforms.

- To start, we use Cargo to make a new project.
- Make sure your terminal is at your rust-learning-path directory
run the following command:
```sh
cargo new hello-cargo
```
This command generates a new directory named hello-cargo 

Build and run your program with Cargo
To execute the boilerplate program, we'll move into the new directory hello-cargo, and then use the cargo run command.
Run the following commands in your terminal:
```sh
cd hello-cargo
cargo run
```
You should see the following output in your terminal:
```sh
  Compiling hello-cargo v0.1.0 (/tmp/.OFUp/hello-cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 1.59s
      Running `target/debug/hello-cargo`

Hello, world!
```
Congratulations, you've written your first Rust program and learned how to initialize your first Rust project with Cargo!



> Note: Just google if you are facing any error!

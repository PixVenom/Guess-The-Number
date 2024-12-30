# Guess the Number Game

A simple command-line game written in Rust where the user guesses a randomly generated number between 1 and 50.

## Features
- Random number generation using the `rand` crate.
- User-friendly input handling with clear instructions and error messages.
- Provides feedback on whether the guessed number is too high, too low, or correct.
- Infinite attempts allowed until the user guesses the correct number.

## Requirements
- [Rust](https://www.rust-lang.org/tools/install) (ensure you have it installed).
- A terminal or command-line interface.

## How to Run
1. Clone this repository to your local machine:
   ```bash
   git clone <repository-url>
   cd <repository-folder>
   ```

2. Run the game using `cargo`:
   ```bash
   cargo run
   ```

3. Follow the on-screen instructions to play the game.

## Code Breakdown
### Key Sections
1. **Random Number Generation:**
   ```rust
   let secret_num: u32 = rand::thread_rng().gen_range(1..=50);
   ```
   This generates a random number between 1 and 50.

2. **Input Handling:**
   ```rust
   let mut guess: String = String::new();
   io::stdin().read_line(&mut guess).expect("Failed to read user input");
   ```
   Reads and processes user input, ensuring it's a valid number.

3. **Comparison Logic:**
   ```rust
   match guess.cmp(&secret_num) {
       Ordering::Equal => {
           println!("You Won!!");
           break;
       }
       Ordering::Greater => println!("Number is greater"),
       Ordering::Less => println!("Number is small"),
   }
   ```
   Provides feedback on whether the guess is correct, too high, or too low.

## Example Gameplay
```
Guess the number!
Please input your guess:
25
You guessed 25
Number is small
Please input your guess:
40
You guessed 40
Number is greater
Please input your guess:
30
You guessed 30
You Won!!
```

## Customization
You can change the range of numbers by modifying this line:
```rust
let secret_num: u32 = rand::thread_rng().gen_range(1..=50);
```
For example, to set the range from 1 to 100:
```rust
let secret_num: u32 = rand::thread_rng().gen_range(1..=100);
```

## Known Issues
- The program does not handle non-numeric input gracefully beyond a simple error message.

## Contributing
Feel free to fork this repository and submit a pull request with improvements or additional features.

## License
This project is open-source and available under the [MIT License](LICENSE).


# Hangman Game Development Tutorial

We will develop a Hangman text based console game within this tutorial.

You'll be creating your individual flavour of the game!

The steps with the corresponding tags are just an orientation, when you are stuck.

Please follow these steps to get the most benefit out of this learning experience.

* Read the instructions at each tag
* Try to implement what this step means
* If you're stuck: First read the hints
* If you don't understand a Rust concept, search the internet and learn about it before you try to conquer the next step. (I know this could be sometimes tadious, but the more effort you put in, the more you will benefit)
* If your code gets to messy, start out fresh at this step from the master branch

Just implement as many additional `println!()` statements to see helpful messages during runtime to get a feeling which line of code the program currently executes. (They are our log statements.) We'll get rid of them  until the end of the tutorial.

# Prerequesites

I assume, you're familiar with `git`.

I recommend you to have [IntelliJ Community Edition](https://www.jetbrains.com/idea/download/) installed.
When you this IDE, make sure you have the Rust plugin installed.

This IDE in combination with the Rust plugin is very powerful and makes coding really easy.

I also assume, you already have run an executable like the `hello world` program. So you're familiar with `cargo new` and `cargo run`.

I strongly recommend to install `clippy`.

```> cargo install cargo-clippy```

It is a linter and gives you hints to optimize your source code.

Usage: `> cargo clippy`

When this passes you can perform a program run: `> cargo run`

# Walkthrough

Start with the initial commit.

Checkout the inital commit: `git checkout v0.0.0 -b develop`

Now you're on the develop branch. This will help you develop your own solution and don't mess with the master branch.

### (tag: v0.0.0) inital commit

Just checkout out this commit and examine the code. It's your starting point.

_HINT_:
You don't need to bother about the lazy-static macro and crate. (I introduced it to save lots of confusing startup code not relevant to the problem.)
You can look it up in the internet if you want to, but it's not necessary to understand the underlying code.

### (tag: v0.0.1) enter a message

How can we gather input from the user?

The user needs to be prompted to enter a line of text, which you're going to diplay in the next line.

_HINT_:
Look into the Rust API at module `std::io`.
What can be useful?

### (tag: v0.0.2) get first charachter of input

Wow, this is quite challenging.

We need to split the string into the first character and the rest.

We are going to put all the string function in a separate module: `src/strings.rs`.

Rust uses UTF-8 encoded strings internally, but stores them into an 8-bit vector `Vec<u8>`, which means that each character can possibly consume up to 5 bytes.

pub fn get_first_unicode_char(s: &str) -> (&str, &str) {

    for i in 1..5 {
        if let Some(head) = s.get(0..i) { return (head, &s[i..]) }
    }

    (&s[0..0], s)
}

_HINT_:
* Use this function in your `main.rs` to display the first character and the rest.
* Use the `destructuring` pattern. (Look it up)

### (tag: v0.0.3) loop it

Now loop all this.

if the user types in `quit`, the loop should break and exit the program.

### (tag: v0.0.4) to upper case

Your Hangman game will only process upper-case letters, so let's implement this.


### (tag: v0.0.5) compare entry with search string to win

At this stage, you should also create a function within the `strings.rs` module to get a distinct string to have a counterpart for comparsion of your entries.

The loop should exit with the message `You win` when the entry and the search string match.


### (tag: v0.0.6) prepare hint

Now, we will display the hint.

The hint will show in a later step the already matching letters and obfuscate the not matching.

An example:
* search_string is [ HANGMAN ]
* the hint will be [ H_N___N ] if you already matched H and N

_HINT_:
* create a function `prepare` in the `strings` module
* within this function, use the external crate regex to replace any character in the search string with an underscore
* display search string and hint for debug purpose


### (tag: v0.0.7) implement find char in string unit test

At this step, we're doing test driven development.

Copy and Paste this snippet (developed by your test department) into the `strings` module:

```rust
#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_find_char_in_string() {
        assert_eq!(false, find_char_in_string("", "hallo"));
        assert_eq!(false, find_char_in_string("c", "hallo"));
        assert_eq!(true, find_char_in_string("a", "hallo"));
        assert_eq!(true, find_char_in_string("h", "hallo"));
        assert_eq!(true, find_char_in_string("o", "hallo"));
    }
}
```

Your job is to implement the find_char_in_string() function.

_HINT_:
Use a recursive approach by utilizing `get_first_unicode_char()`. (You can doit!)


### (tag: v0.0.8) show character hit or miss

Use this new function in the `main.rs` to print out, if the user performed a hit or not.

### (tag: v0.0.9) show already matched characters

Introduce a variable which holds all the hits in a non-redundant string.

Display at least following log messages:

* Character Hit!
* Character Miss!
* And when the variable doesn't already contain the character, the matched characters string. (You need to add the last hit to this string before)


### (tag: v0.0.10) process hint

New unit test in `strings` module: 

```rust
    #[test]
    fn test_create_hint() {
        assert_eq!("_____", create_hint("hallo", "").as_str());
        assert_eq!("_all_", create_hint("hallo", "la").as_str());
        assert_eq!("hallo", create_hint("hallo", "hola").as_str());
    }
```

Create the create_hint function.

_HINT_:
Use a regular expression to do this.

Go to [http://creates.io](http://creates.io) and add the `regex` create to your Cargo.toml dependencies.

( _HINT 2_:
Create the regular expression by concatination of strings. )

### (tag: v0.0.11) loose after 7 wrong tries

Introduce an integer variable which holds the amount f misses.

Acceptance Criteria:
* Exit with message `You Loose` when the tries reach 7.

(The hangman has 7 body parts)


### (tag: v0.0.12) add more graphics

Feel free to copy the solution into your program !

Reason about the enum definition:
* Why did we do it like this?


### (tag: v0.0.13) refactor display of graphic

Add the convenience method `display` to the Graphic type and use it in the main.

As you have seen in the step 0.0.0 that we use a hashmap to store the graphics. We won't write the whole function calls every time we want to display the graphic.

_HINT_: 
Just move the display call to this (static) method.

### (tag: v0.0.14) display graphics during game

Change the state to the appropriate graphic and voilà we'll get the right graphic displayed.

### (tag: v0.0.15) add quit graphics

Add the `good bye` graphics.

Feel free to look in the solution, but implement it by your own !

### (tag: v0.0.16) show searched string on exit

As the commit message says, show what hidden word was queried to.

### (tag: v0.0.17) show hints in graphic

This one is a bit more complex to grasp.

We have in each `error` graphics a sequence `...........................`.

It needs to be substituted with the hint so we can get rid of plain text message.

_HINT_:
Implement `len_of_string()` by using, once again, recursion.

```rust
    #[test]
    fn test_len_of_string() {
        assert_eq!(0, len_of_string(""));
        assert_eq!(5, len_of_string("hallo"));
        assert_eq!(3, len_of_string("olá"));
    }
```

_HINT 2_:
* Change the `display` function to have the state argument and the additional hint argument.
* Feed the hint into the `draw` function. It doesn't have this parameter, yet, but the main programming effort has to happen within it.
* there you use the `len_of_string()` function

### (tag: v0.0.18) clean up

Get rid of all unnecessary log messages.

*AND YOU'RE DONE !!!*

### (tag: v0.1.0, origin/master, master) bugfix

I have to admit, that I was not careful enough in step 0.0.17 and have a bug.

I fixed it and published version `v0.1.0`.

# Further Thoughts

Our program currently one disguises one single word (HANGMAN).

It's open to you to create a file with search strings from which we can randomly choose one word to feed it into our game at startup.

This would certainly improve the fun factor when playing it several times.

Also we could `prettify` our hint by introducing spaces between the letters.
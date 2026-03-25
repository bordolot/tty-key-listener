# tty-key-listener

tty-key-listener is a pure rust, minimal, zero-dependency, terminal keyboard event listener library. It supports these UNIX terminals that are controlled by the `dev/tty` special file. 

## Features

- epoll-based
- ArrowUp, ArrowDown, ArrowRigt, ArrowLeft
- ASCII encoded characters

## Getting Started

```rust
use tty_key_listener::TtyListener;
use tty_key_listener::event::KeyEvent::{
    ArrowDown, ArrowLeft, ArrowRight, ArrowUp, KeyChar, Undefined,
};

fn main() -> std::io::Result<()> {
    let mut lis = TtyListener::new()?;
    loop {
        match lis.wait_for_next_event()? {
            ArrowUp => println!("ArrowUp\r"),
            ArrowDown => println!("ArrowDown\r"),
            ArrowLeft => println!("ArrowLeft\r"),
            ArrowRight => println!("ArrowRight\r"),
            KeyChar(char) => match char {
                'q' => break,
                char => println!("ASCII char {char}\r"),
            },
            Undefined => {
                println!("Undefined\r");
            }
        }
    }
    lis.clean()?;
    Ok(())
}
```
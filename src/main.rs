use std::{
    io::{stdout, Write},
    task::{Context, Poll},
    pin::Pin,
    future::Future,
};

use pasts::prelude::*;

use futures_core::stream::Stream;

use crossterm::{
    cursor::position,
    event::{DisableMouseCapture, EnableMouseCapture, Event, EventStream, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode},
    Result
};

const HELP: &str = r#"EventStream based on futures_core::stream::Stream with pasts
 - Keyboard, mouse and terminal resize events enabled
 - Hit "c" to print current cursor position
 - Use Esc to quit
"#;

/// Needed in order to await the stream.
struct EventFuture<S: Stream<Item = Result<Event>> + Unpin>(S);

impl<S: Stream<Item = Result<Event>> + Unpin> Future for EventFuture<S> {
    type Output = Option<Result<Event>>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Pin::new(&mut self.0).poll_next(cx)
    }
}

async fn print_events() {
    let mut reader = EventFuture(EventStream::new());

    loop {
        match (&mut reader).await {
            Some(Ok(event)) => {
                println!("Event::{:?}\r", event);

                if event == Event::Key(KeyCode::Char('c').into()) {
                    println!("Cursor position: {:?}\r", position());
                }

                if event == Event::Key(KeyCode::Esc.into()) {
                    break;
                }
            }
            Some(Err(e)) => println!("Error: {:?}\r", e),
            None => break,
        }
    }
}

async fn async_main() {
    println!("{}", HELP);

    enable_raw_mode().unwrap();

    let mut stdout = stdout();
    execute!(stdout, EnableMouseCapture).unwrap();

    print_events().await;

    execute!(stdout, DisableMouseCapture).unwrap();

    disable_raw_mode().unwrap();
}

fn main() {
    exec!(async_main());
}

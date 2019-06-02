mod mac_addr;
mod rtnetlink;
mod rtsocket;

#[macro_use]
extern crate num_derive;
extern crate chrono;

use chrono::prelude::*;
use rtsocket::RtMsg;
use rtsocket::*;
use std::error::Error;
use std::sync::mpsc::{self, TryRecvError};
use std::thread;

/* Example using callback */
// fn receive<T>(socket: &RtSocket, should_exit: T)
// where
//     T: Fn() -> bool,
// {
//     socket.receive(|msg| {
//         if let Some(RtMsg { event, time, data }) = msg {
//             let datetime: DateTime<Utc> = time.into();
//             println!(
//                 "{} *{:?}* {:?}\n",
//                 datetime.format("%d.%m.%Y %T.%3f"),
//                 event,
//                 data
//             );
//         }
//     });

//     if !should_exit() {
//         receive(socket, should_exit);
//     }
// }

pub fn run() -> Result<(), Box<dyn Error>> {
    println!("Press Enter to exit");
    
    let socket = RtSocket::new();

    // let should_exit = || false;
    // let handle = thread::spawn(move || {
    //     receive(&socket, should_exit);
    // });
    // handle.join().unwrap();

    let (tx, rx) = mpsc::channel();

    let _handle = thread::spawn(move || {
        use rtsocket::RtData::*;

        socket
            .take_while(|_| match rx.try_recv() {
                Ok(_) | Err(TryRecvError::Disconnected) => false,
                Err(TryRecvError::Empty) => true,
            })
            .filter(|msg| match msg {
                RtMsg {
                    data: Unsuported(_),
                    ..
                } => false,
                _ => true,
            })
            .for_each(|msg| {
                let RtMsg { event, time, data } = msg;
                let datetime: DateTime<Local> = time.into();
                println!(
                    "{} *{:?}* {:?}\n",
                    datetime.format("%d.%m.%Y %T.%3f"),
                    event,
                    data
                );
            });
    });

    // Exit on enter, wait for last message
    use std::io::{self, BufRead};
    let mut line = String::new();
    let stdin = io::stdin();
    let _ = stdin.lock().read_line(&mut line);
    let _ = tx.send(());
    tx.send(()).unwrap();

    // handle.join().unwrap();

    Ok(())
}

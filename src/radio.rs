
use hal::peripherals::RADIO;
use hal::radio::ieee802154;
use rtic_sync::channel::{ Channel, ReceiveError, Receiver, Sender, TrySendError};
use crate::proto::{  
    self, 
    Command, 
    Event,
    Command_,
    Send,
    Configure,
};

const COMMANDS_SIZE: usize = 4;
const EVENTS_SIZE: usize = 4;

pub type Radio = ieee802154::Radio<'static, RADIO>;

pub struct RadioAllocator {
    commands: Channel<crate::proto::Command, COMMANDS_SIZE>,
    events: Channel<crate::proto::Event, EVENTS_SIZE>,
}

pub struct RadioClient<'a> {
    pub commands: Sender<'a, crate::proto::Command, COMMANDS_SIZE>,
    pub events: Receiver<'a, crate::proto::Event, EVENTS_SIZE>,
}

pub struct RadioTask<'a> {
    commands: Receiver<'a, crate::proto::Command, COMMANDS_SIZE>,
    events: Sender<'a, crate::proto::Event, EVENTS_SIZE>,
    radio: Radio,
}

impl RadioAllocator {
    pub const fn new() -> Self {
        RadioAllocator {
            commands: Channel::new(),
            events: Channel::new(),
        }
    }

    pub fn allocate<'a> (&'a mut self, radio: Radio) -> (RadioClient<'a>, RadioTask<'a>) {

        let (command_sender, command_receiver) = self.commands.split();
        let (event_sender, event_receiver) = self.events.split();
        (
            RadioClient {
                commands: command_sender,
                events: event_receiver,
            },
            RadioTask {
                commands: command_receiver,
                events: event_sender,
                radio
            }
        )
    }
}

impl <'a> RadioClient<'a> {

}

impl <'a> RadioTask<'a> {
    pub async fn run(&mut self) {
        loop {
            match self.commands.recv().await {
                Ok(Command { command: Some(command) }) => {
                    match command {
                        Command_::Command::Send(Send { payload }) => {
                            let mut packet = ieee802154::Packet::new();
                            packet.copy_from_slice(&payload);
                            self.radio.try_send(&mut packet).await.unwrap();
                        },
                        _ => {
                            // Handle other commands
                        }
                    }
                },
                Err(ReceiveError::NoSender) => break,
                Ok(Command { command: None }) | Err(ReceiveError::Empty) => continue,
            }
        }
    }
}

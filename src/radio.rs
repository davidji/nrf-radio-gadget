
use futures::{select_biased, FutureExt};
use defmt::{ debug, warn, info };
use heapless::Vec;
use hal::ieee802154::{self, Packet};
use rtic_sync::channel::{ Channel, ReceiveError, Receiver, Sender };
use crate::proto::{  
    self, 
    Command, 
    Event,
    Command_,
    Transmit,
    Configure,
};


pub const COMMANDS_SIZE: usize = 4;
pub const EVENTS_SIZE: usize = 4;

pub type Radio = ieee802154::Radio<'static>;

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
            let mut packet = Packet::new();
            select_biased! {
                result = self.radio.recv_non_blocking(&mut packet).fuse() => {
                    match result {
                        Ok(_) => self.received(&mut packet).await,
                        Err(crc) => debug!("CRC check failed: {:?}", crc),
                    }
                },

                result = self.commands.recv().fuse() => {
                    match result {
                        Ok(Command { command }) => self.command(command),
                        Err(ReceiveError::NoSender) => break,
                        Err(ReceiveError::Empty) => warn!("Empty result awaiting command"),
                    }
                }
            };
        }

        warn!("Radio task ending");
    }

    async fn received(&mut self, packet: &mut Packet) {
        self.events.send(Event {
            event: Some(proto::Event_::Event::Received(proto::Received { 
                payload: Vec::from_slice(packet.as_ref()).unwrap(),
                link_quality_indicator: packet.lqi() as u32,
            })),
        }).await.unwrap();
    }

    fn command(&mut self, command: Option<Command_::Command>) {
        match command {
            Some(Command_::Command::Transmit(Transmit { payload })) => {
                let mut packet = ieee802154::Packet::new();
                packet.copy_from_slice(&payload);
                info!("Transmitting packet");
                self.radio.send(&mut packet);
                info!("Transmit complete");
            },
            Some(Command_::Command::Configure(Configure { channel, tx_power })) => {
                info!("Configuring radio");
                self.radio.set_channel(channel.try_into().unwrap());
                self.radio.set_txpower(tx_power.try_into().unwrap());
                info!("Configuration complete");
            },
            None => {
                warn!("Received command without payload");
            }
        }
    }
}

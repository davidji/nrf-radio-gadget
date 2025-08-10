#![no_main]
#![no_std]


use panic_probe as _;
use defmt_rtt as _;

mod proto;
mod radio;


use rtic_monotonics::nrf::timer::prelude::*;
const MONO_RATE: u32 = 1_000_000;
nrf_timer0_monotonic!(Mono, MONO_RATE);

#[rtic::app(device = hal::pac, dispatchers = [ UARTE0_UART0, UARTE1, SPIM3 ])]
mod app {

    use crate::Mono;
    use crate::MONO_RATE;
    use crate::radio;
    use crate::proto;

    use defmt::debug;
    use embedded_hal::digital::OutputPin;
    use gadget::usb::UsbBusAllocator;
    use gadget::{
        codec,
        stream::{
            self,
            channel::{ChannelSink,ChannelStream},
        },
        usb::{ Clock, Gadget, GadgetStorage, NetworkChannelStorage, RecvChannel, SendChannel },
    };
    use hal::{
        pac::FICR,
        clocks::{ self, Clocks, },
        ieee802154,
        gpio::{ Level, Output, PushPull},
        Rng,
        usbd::{ UsbPeripheral, Usbd },
    };

    use micropb::MessageEncode;
    use rtic_monotonics::{
        fugit::ExtU32, 
        Monotonic
    };
    
    impl Clock for Mono {
        type Instant = fugit::Instant<u64, 1, MONO_RATE>;
        fn now() -> Self::Instant {
            <Mono as Monotonic>::now()
        }
    }

    type UsbBus = Usbd<UsbPeripheral<'static>>;
    type CommandDecoder = codec::Decoder<
            ChannelStream<'static, u8, CHANNEL_CAPACITY>,
            proto::Command,
            { proto::Command::MAX_SIZE.unwrap() }>;
    type EventEncoder = codec::Encoder<
            proto::Event,
            ChannelSink<'static, u8, CHANNEL_CAPACITY>,
            { proto::Event::MAX_SIZE.unwrap() }>;

    const SOCKETS: usize = 2; // DHCP + Radio
    const CHANNEL_CAPACITY: usize = 2*gadget::usb::MTU as usize;
    const CHANNELS: usize = 1; // Radio channel

    #[shared]
    struct Shared {
        gadget: Gadget<'static, Mono, UsbBus>,
    }


    #[local]
    struct Local {
        radio_command_decoder: CommandDecoder,
        radio_event_encoder: EventEncoder,
        radio_commands: ChannelSink<'static, proto::Command, { radio::COMMANDS_SIZE }>,
        radio_events: ChannelStream<'static, proto::Event, { radio::EVENTS_SIZE }>,
        radio_task: radio::RadioTask<'static>,
        network_send: [SendChannel<'static, CHANNEL_CAPACITY>; CHANNELS],
        network_recv: [RecvChannel<'static, CHANNEL_CAPACITY>; CHANNELS],
        led: hal::gpio::p0::P0_15<Output<PushPull>>,
    }
    
    #[init(local = [
        clocks: Option<Clocks<
            clocks::ExternalOscillator,
            clocks::Internal,
            clocks::LfOscStopped>> = None,
        radio: radio::RadioAllocator = radio::RadioAllocator::new(),
        gadget_storage: GadgetStorage<'static, UsbBus, SOCKETS> = GadgetStorage::new(),
        radio_channel_storage: NetworkChannelStorage<CHANNEL_CAPACITY> = NetworkChannelStorage::new(),
    ])]
    fn init(cx: init::Context) -> (Shared, Local) {
        let peripherals = cx.device;
        let ficr = &peripherals.FICR;
        
        cx.local.clocks.replace(Clocks::new(peripherals.CLOCK).enable_ext_hfosc());
        let clocks = cx.local.clocks.as_ref().unwrap();

        Mono::start(peripherals.TIMER0);

        let usb_device = Usbd::new(UsbPeripheral::new(peripherals.USBD, clocks));
        
        let usb_bus_allocator = UsbBusAllocator::new(
                usb_device);

        let mut gadget = Gadget::new(
            b"radio",
            mac_address("interface", ficr),
            mac_address("gadget", ficr),
            cx.local.gadget_storage,
            usb_bus_allocator,
            Rng::new(peripherals.RNG).random_u64());
 

        let radio = ieee802154::Radio::init(peripherals.RADIO, clocks);
        let (radio_task_client, radio_task) = cx.local.radio.allocate(radio);
        let radio_channel = gadget.channel(1338, cx.local.radio_channel_storage);

        let port0 = hal::gpio::p0::Parts::new(peripherals.P0);
        let led = port0.p0_15.into_push_pull_output(Level::Low);

        radio_send_receive::spawn().unwrap();
        usb::spawn().unwrap();
        blink::spawn().unwrap();

        debug!("init done");

        (
            Shared { gadget }, 
            Local {
                radio_command_decoder: codec::Decoder::new(ChannelStream::new(radio_channel.app.recv)),
                radio_event_encoder: codec::Encoder::new(ChannelSink::new(radio_channel.app.send)),
                radio_commands: ChannelSink::new(radio_task_client.commands),
                radio_events: ChannelStream::new(radio_task_client.events),
                radio_task,
                network_recv: [ radio_channel.net.recv],
                network_send : [ radio_channel.net.send],
                led,
            }
        )
    }


    #[task(shared = [ gadget ], local = [ network_send, network_recv ], priority=2)]
    async fn usb(cx: usb::Context) {
        // I can't find an example that enables the USB interrupt, so for now, polling
        let send_channels = cx.local.network_send;
        let recv_channels = cx.local.network_recv;
        let mut shared = cx.shared.gadget;

        loop {
            let start = <Mono as Monotonic>::now();
            shared.lock(|gadget| {
                gadget.poll(send_channels, recv_channels);
            });

            Mono::delay_until(start + 500.micros()).await;
        }
    }

    #[task(local=[radio_commands, radio_command_decoder])]
    async fn radio_commands_decode(cx: radio_commands_decode::Context) {
        stream::relay(cx.local.radio_command_decoder, cx.local.radio_commands).await.unwrap();
    }

    #[task(local=[radio_events, radio_event_encoder])]
    async fn radio_events_encode(cx: radio_events_encode::Context) {
        stream::relay(cx.local.radio_events, cx.local.radio_event_encoder).await.unwrap();
    }
    

    #[task(local = [radio_task], priority = 1)]
    async fn radio_send_receive(cx: radio_send_receive::Context) {
        let radio_task = cx.local.radio_task;
        radio_task.run().await;
    }

    #[task(local = [ led ])]
    async fn blink(cx: blink::Context) {
        loop {
            Mono::delay(1000.millis().into()).await;
            cx.local.led.set_high().unwrap();
            Mono::delay(1000.millis().into()).await;
            cx.local.led.set_low().unwrap();
        }
    }

    pub fn mac_address(seed: &str, ficr: &FICR) -> [u8; 6] {
        use sha2::{ Digest, Sha256 };
        let mut digest = Sha256::new();
        digest.update(seed.as_bytes());
        digest.update(ficr.deviceid[0].read().bits().to_le_bytes());
        digest.update(ficr.deviceid[1].read().bits().to_le_bytes());
        let hash = digest.finalize();
        let mut mac = [0u8; 6];
        mac.copy_from_slice(&hash[0..6]);
        // Set the second-least-significant bit to indicate a locally administered address
        mac[0] |= 0b00000010;
        // the least significant bit in the most significant octet of a MAC address is the multicast bit
        mac[0] &= 0b11111110;
        mac
    }

}



#![no_main]
#![no_std]


use panic_probe as _;
use defmt_rtt as _;

pub mod proto {
    #![allow(clippy::all)]
    #![allow(nonstandard_style, unused, irrefutable_let_patterns)]
    include!(concat!(env!("OUT_DIR"), "/proto.rs"));
}

mod radio;

#[rtic::app(device = hal::pac, dispatchers = [ UARTE1 ])]
mod app {

    use crate::radio;

    use hal::{
        clocks::{ self, Clocks, },
        ieee802154,
        usbd::{ UsbPeripheral, Usbd },
        
    };

    use usb_device::{
        bus::UsbBusAllocator, 
        device::{ 
            StringDescriptors, 
            UsbDevice, 
            UsbDeviceBuilder, 
            UsbVidPid }};

    type Radio = ieee802154::Radio<'static>;
    type UsbBus = Usbd<UsbPeripheral<'static>>;

    #[shared]
    struct Shared {
        usb_dev: UsbDevice<'static, UsbBus>,
        serial: usbd_serial::SerialPort<'static, UsbBus>,
        radio_client: radio::RadioClient<'static>,
    }

    #[local]
    struct Local {
        radio_task: radio::RadioTask<'static>
    }
    
    #[init(local = [
        usb_bus: Option<usb_device::bus::UsbBusAllocator<Usbd<UsbPeripheral<'static>>>> = None,
        ep_memory: [u32; 4096] = [0; 4096],
        clocks: Option<Clocks<
            clocks::ExternalOscillator,
            clocks::ExternalOscillator,
            clocks::LfOscStarted,
        >> = None,
        radio: radio::RadioAllocator = radio::RadioAllocator::new(),
        ])]
    fn init(cx: init::Context) -> (Shared, Local) {
        let peripherals = cx.device;
        
        cx.local.clocks.replace(Clocks::new(peripherals.CLOCK)
            .enable_ext_hfosc()
            .set_lfclk_src_external(clocks::LfOscConfiguration::NoExternalNoBypass)
            .start_lfclk()
            .enable_ext_hfosc());
        let clocks = cx.local.clocks.as_ref().unwrap();
        
        cx.local.usb_bus.replace(
            UsbBusAllocator::new(
                Usbd::new(UsbPeripheral::new(peripherals.USBD, clocks))));
               
        let usb_bus: &UsbBusAllocator<Usbd<UsbPeripheral<'static>>> = cx.local.usb_bus.as_ref().unwrap();

        let serial = usbd_serial::SerialPort::new(&usb_bus);
        let usb_dev = UsbDeviceBuilder::new(
            &usb_bus,
            UsbVidPid(0x16c0, 0x27dd),
        )
        .device_class(usbd_serial::USB_CLASS_CDC)
        .strings(&[StringDescriptors::default()
            .manufacturer("Fake Company")
            .product("Product")
            .serial_number("TEST")])
        .unwrap()
        .build();


        let radio = ieee802154::Radio::init(peripherals.RADIO, clocks);
        let (radio_client, radio_task) = cx.local.radio.allocate(radio);
        

        (Shared { usb_dev, serial, radio_client }, Local { radio_task })
    }

    #[task(binds = USBD, shared = [usb_dev, serial])]
    fn usb_tx(cx: usb_tx::Context) {

    }

    #[task(local = [radio_task], shared = [radio_client])]
    async fn radio_task(cx: radio_task::Context) {
        let radio_task = cx.local.radio_task;
    }
}

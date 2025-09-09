#!.venv/bin/python
import client, asyncio
import client.ieee_802_15_4_pb2 as radio

async def connect():
    radio1 = await client.connect('192.168.3.1')
    radio2 = await client.connect('192.168.4.1')

    radio1.configure(channel=radio.C15, tx_power=radio.P0dBm)
    radio2.configure(channel=radio.C15, tx_power=radio.P0dBm)
    async def send():
        for i in range(10):
            radio1.transmit(b'Hello from radio 1')
            await asyncio.sleep(3)
        radio1.fan_set_duty(0.2)

    await asyncio.gather(radio2.print_received(), send())


#!.venv/bin/python
import client, asyncio
import client.ieee_802_15_4_pb2 as radio

async def connect(sender_address, receiver_address):
    radio1 = await client.connect(sender_address)
    radio2 = await client.connect(receiver_address)

    radio1.configure(channel=radio.C15, tx_power=radio.POS8D_BM)
    radio2.configure(channel=radio.C15, tx_power=radio.POS8D_BM)
    async def send():
        for i in range(10):
            radio1.transmit(b'Hello from radio 1')
            await asyncio.sleep(3)

    await asyncio.gather(radio1.print_received(), radio2.print_received(), send())

def main(args):
    asyncio.run(connect(args[0], args[1]))

if __name__ == "__main__":
    import sys
    main(sys.argv[1:])

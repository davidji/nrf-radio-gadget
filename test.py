#!.venv/bin/python
import client, asyncio
import client.ieee_802_15_4_pb2 as radio

async def connect(sender_address, receiver_address):
    radio1 = await client.connect(sender_address)
    radio2 = await client.connect(receiver_address)

    
    async def send():
        for channel in (radio.C11, radio.C12, radio.C13, radio.C14, radio.C15, radio.C16,
                        radio.C17, radio.C18, radio.C19, radio.C20, radio.C21, radio.C22,
                        radio.C23, radio.C24, radio.C25, radio.C26):
            print("Setting channel", channel)
            radio1.configure(channel=channel, tx_power=radio.POS2D_BM)
            radio2.configure(channel=channel, tx_power=radio.POS2D_BM)
            for i in range(20):
                print("Sending packet", i)
                radio1.transmit(b'Hello from radio 1')
                print("Sent packet", i)
                await asyncio.sleep(0.1)

    await asyncio.gather(radio1.print_received(), radio2.print_received(), send())

def main(args):
    asyncio.run(connect(args[0], args[1]))

if __name__ == "__main__":
    import sys
    main(sys.argv[1:])

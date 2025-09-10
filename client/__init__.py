from . import ieee_802_15_4_pb2 as radio
from cobs import cobs
import asyncio

class RadioClient:
    def __init__(self, 
                 address: str,
                 reader: asyncio.StreamReader, 
                 writer: asyncio.StreamWriter):
        self.address = address
        self.reader = reader
        self.writer = writer

    async def recv(self) -> radio.Event:
        response = radio.Event()
        response.ParseFromString(cobs.decode((await self.reader.readuntil(b'\0'))[:-1]))
        return response

    def send(self, message: radio.Command):
        message = cobs.encode(message.SerializeToString()) + b'\0'
        self.writer.write(message)
    
    async def print_received(self):
        while not self.reader.at_eof():
            message = await self.recv()
            print(self.address, self.protobuf_to_json(message))

    def configure(self, channel, tx_power):
        request = radio.Command()
        request.configure.channel = channel
        request.configure.tx_power = tx_power
        self.send(request)

    def transmit(self, payload: bytes):
        request = radio.Command()
        request.transmit.payload = payload
        self.send(request)

    @staticmethod
    def protobuf_to_json(message):
        from google.protobuf.json_format import MessageToJson
        return MessageToJson(message, indent=2)

async def connect(gadget) -> RadioClient:
    (reader, writer) = await asyncio.open_connection(gadget, 1338)
    return RadioClient(gadget, reader, writer)

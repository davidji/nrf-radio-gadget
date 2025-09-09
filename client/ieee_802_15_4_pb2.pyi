from google.protobuf.internal import enum_type_wrapper as _enum_type_wrapper
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from typing import ClassVar as _ClassVar, Mapping as _Mapping, Optional as _Optional, Union as _Union

DESCRIPTOR: _descriptor.FileDescriptor

class Channel(int, metaclass=_enum_type_wrapper.EnumTypeWrapper):
    __slots__ = ()
    NONE: _ClassVar[Channel]
    C11: _ClassVar[Channel]
    C12: _ClassVar[Channel]
    C13: _ClassVar[Channel]
    C14: _ClassVar[Channel]
    C15: _ClassVar[Channel]
    C16: _ClassVar[Channel]
    C17: _ClassVar[Channel]
    C18: _ClassVar[Channel]
    C19: _ClassVar[Channel]
    C20: _ClassVar[Channel]
    C21: _ClassVar[Channel]
    C22: _ClassVar[Channel]
    C23: _ClassVar[Channel]
    C24: _ClassVar[Channel]
    C25: _ClassVar[Channel]
    C26: _ClassVar[Channel]

class TxPower(int, metaclass=_enum_type_wrapper.EnumTypeWrapper):
    __slots__ = ()
    POS8D_BM: _ClassVar[TxPower]
    POS7D_BM: _ClassVar[TxPower]
    POS6D_BM: _ClassVar[TxPower]
    POS5D_BM: _ClassVar[TxPower]
    POS4D_BM: _ClassVar[TxPower]
    POS2D_BM: _ClassVar[TxPower]
    _0D_BM: _ClassVar[TxPower]
    NEG4D_BM: _ClassVar[TxPower]
    NEG8D_BM: _ClassVar[TxPower]
    NEG12D_BM: _ClassVar[TxPower]
    NEG16D_BM: _ClassVar[TxPower]
    NEG20D_BM: _ClassVar[TxPower]
    NEG40D_BM: _ClassVar[TxPower]
NONE: Channel
C11: Channel
C12: Channel
C13: Channel
C14: Channel
C15: Channel
C16: Channel
C17: Channel
C18: Channel
C19: Channel
C20: Channel
C21: Channel
C22: Channel
C23: Channel
C24: Channel
C25: Channel
C26: Channel
POS8D_BM: TxPower
POS7D_BM: TxPower
POS6D_BM: TxPower
POS5D_BM: TxPower
POS4D_BM: TxPower
POS2D_BM: TxPower
_0D_BM: TxPower
NEG4D_BM: TxPower
NEG8D_BM: TxPower
NEG12D_BM: TxPower
NEG16D_BM: TxPower
NEG20D_BM: TxPower
NEG40D_BM: TxPower

class Command(_message.Message):
    __slots__ = ("configure", "transmit")
    CONFIGURE_FIELD_NUMBER: _ClassVar[int]
    TRANSMIT_FIELD_NUMBER: _ClassVar[int]
    configure: Configure
    transmit: Transmit
    def __init__(self, configure: _Optional[_Union[Configure, _Mapping]] = ..., transmit: _Optional[_Union[Transmit, _Mapping]] = ...) -> None: ...

class Event(_message.Message):
    __slots__ = ("received",)
    RECEIVED_FIELD_NUMBER: _ClassVar[int]
    received: Received
    def __init__(self, received: _Optional[_Union[Received, _Mapping]] = ...) -> None: ...

class Received(_message.Message):
    __slots__ = ("payload", "link_quality_indicator")
    PAYLOAD_FIELD_NUMBER: _ClassVar[int]
    LINK_QUALITY_INDICATOR_FIELD_NUMBER: _ClassVar[int]
    payload: bytes
    link_quality_indicator: int
    def __init__(self, payload: _Optional[bytes] = ..., link_quality_indicator: _Optional[int] = ...) -> None: ...

class Transmit(_message.Message):
    __slots__ = ("payload",)
    PAYLOAD_FIELD_NUMBER: _ClassVar[int]
    payload: bytes
    def __init__(self, payload: _Optional[bytes] = ...) -> None: ...

class Configure(_message.Message):
    __slots__ = ("channel", "tx_power")
    CHANNEL_FIELD_NUMBER: _ClassVar[int]
    TX_POWER_FIELD_NUMBER: _ClassVar[int]
    channel: Channel
    tx_power: TxPower
    def __init__(self, channel: _Optional[_Union[Channel, str]] = ..., tx_power: _Optional[_Union[TxPower, str]] = ...) -> None: ...

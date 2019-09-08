// Copyright (C) 2018, Cloudflare, Inc.
// Copyright (C) 2018, Alessandro Ghedini
// All rights reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are
// met:
//
//     * Redistributions of source code must retain the above copyright notice,
//       this list of conditions and the following disclaimer.
//
//     * Redistributions in binary form must reproduce the above copyright
//       notice, this list of conditions and the following disclaimer in the
//       documentation and/or other materials provided with the distribution.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS
// IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO,
// THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
// PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR
// CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
// EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
// PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
// PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF
// LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING
// NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
// SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

extern crate serde;
extern crate serde_json;

use serde::ser::{/*Serialize,*/ SerializeStruct, Serializer};

macro_rules! enum_str {
    ($name:ident { $($variant:ident($str:expr), )* }) => {
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        pub enum $name {
            $($variant,)*
        }

        impl ::serde::Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where S: ::serde::Serializer,
            {
                // Serialize the enum as a string.
                serializer.serialize_str(match *self {
                    $( $name::$variant => $str, )*
                })
            }
        }

        impl<'de> ::serde::Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where D: ::serde::Deserializer<'de>,
            {
                struct Visitor;

                impl<'de> ::serde::de::Visitor<'de> for Visitor {
                    type Value = $name;

                    fn expecting(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                        write!(formatter, "a string for {}", stringify!($name))
                    }

                    fn visit_str<E>(self, value: &str) -> Result<$name, E>
                        where E: ::serde::de::Error,
                    {
                        match value {
                            $( $str => Ok($name::$variant), )*
                            _ => Err(E::invalid_value(::serde::de::Unexpected::Other(
                                &format!("unknown {} variant: {}", stringify!($name), value)
                            ), &self)),
                        }
                    }
                }

                // Deserialize the enum from a string.
                deserializer.deserialize_str(Visitor)
            }
        }
    }
}

enum_str!(EventCategory {
    Connectivity("connectivity"),

    Security("security"),

    Transport("transport"),

    Recovery("recovery"),

});

enum_str!(ConnectivtyEventType {
    ConnectionAttempt("connection_attempt"),

    ConnectionNew("connection_new"),

    ConnectionIdUpdate("connection_id_update"),

    SpinBitUpdate("spin_bit_update"),

    ConnectionClose("connection_close"),

});

enum_str!(ConnectivityEventTrigger {
    Line("line"),
});

enum_str!(TransportEventType {
    DatagramSent("datagram_sent"),

    DatagramReceived("datagram_received"),

    PacketSent("packet_sent"),

    PacketReceived("packet_received"),

    PacketDropped("packet_dropped"),

    PacketBuffered("packet_buffered"),

    StreamStateUpdate("stream_state_update"),

    FlowControlUpdate("flow_control_update"),

    VersionUpdate("version_update"),

    TransportParametersUpdate("transport_parameter_update"),

    AlpnUpdate("ALPN_update"),
});

enum_str!(TransportEventTrigger {
    Line("line"),

    Retransmit("retransmit"),

    KeysUnavailable("keys_unavailable"),
});

enum_str!(SecurityEventType {
    CipherUpdate("cipher_update"),

    KeyUpdate("key_update"),

    KeyRetire("key_retire"),
});

enum_str!(SecurityEventTrigger {
    Tls("tls"),

    Implicit("implicit"),

    RemoteUpdate("remote_update"),

    LocalUpdate("local_update"),
});

enum_str!(RecoveryEventType {
    StateUpdate("state_update"),

    MetricUpdate("metric_update"),

    LossAlarmSet("loss_alarm_set"),

    LossAlarmTriggered("loss_alarm_triggered"),

    PacketLost("packet_lost"),

    PacketAcknowledged("packet_acknowledged"),

    PacketRetransmit("packet_retransmit"),
});

enum_str!(RecoveryEventTrigger {
    AckReceived("ack_received"),

    PacketSent("packet_sent"),

    Alarm("alarm"),

    Unknown("unknown"),
});

// ================================================================== //

enum_str!(KeyType {
    ServerInitialSecret("server_initial_secret"),

    ClientInitialSecret("client_initial_secret"),

    ServerHandshakeSecret("server_handhshake_secret"),

    ClientHandshakeSecret("client_handshake_secret"),

    Server0RttSecret("server_0rtt_secret"),

    Client0RttSecret("client_0rtt_secret"),

    Server1RttSecret("server_1rtt_secret"),

    Client1RttSecret("client_1rtt_secret"),
});

// ================================================================== //
// CONNECTIVITY

pub struct EventListening {
    ip: String,
    port: u64,
    quic_versions: Option<Vec<String>>,
    alpn_values: Option<Vec<String>>
}

pub struct EventConnectionNew {
    ip_verison: String,
    src_ip: String,
    dst_ip: String,

    transport_protocol: Option<String>,
    src_port: u64,
    dst_port: u64,

    quic_version: Option<String>,
    src_cid: Option<String>,
    dst_cid: Option<String>
}

pub struct EventConnectionIdUpdate {
    src_old: Option<String>,
    src_new: Option<String>,

    dst_old: Option<String>,
    dst_new: Option<String>,
}

pub struct EventSpinBitUpdate {
    state: bool
}

pub struct EventConnectionClose {
    src_id: String,
}

// ================================================================== //
// SECURITY

pub struct EventCipherUpdate {
    cipher_type: String,
}

pub struct EventKeyRetire {
    key_type: KeyType,
    key: String,
    generation: Option<String>
}

pub struct EventKeyUpdate {
    key_type: KeyType,
    old: Option<String>,
    new: String,
    generation: Option<String>
}

// ================================================================== //
// TRANSPORT

pub struct EventDatagramReceived {
    count: Option<u64>,
    byte_length: u64
}

pub struct EventDatagramSent {
    count: Option<u64>,
    byte_length: u64
}

pub struct EventPacketReceived {
    raw_encrypted: Option<String>,

    packet_type: PacketType,
    header: Option<PacketHeader>,
    frames:  Option<Vec<QuicFrame>>,
}

#[derive(serde::Deserialize)]
pub struct EventPacketSent {
    pub raw_encrypted: Option<String>,

    pub packet_type: PacketType,
    pub header: Option<PacketHeader>,
    pub frames: Option<Vec<QuicFrame>>,
}

impl serde::Serialize for EventPacketSent {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("EventPacketSent", 4)?;
        if let Some(raw_encrypted) = &self.raw_encrypted {
            s.serialize_field("raw_encrypted", &format!("{}", raw_encrypted))?;
        }

        s.serialize_field("packet_type", &serde_json::to_string(&self.packet_type).unwrap())?;

        if let Some(header) = &self.header {
            s.serialize_field("header", &serde_json::to_string(&header).unwrap())?;
        }

        if let Some(frames) = &self.frames {
            s.serialize_field("frames", &serde_json::to_string(&frames).unwrap())?;
        }

        s.end()
    }
}

pub struct EventPacketBuffered {
    packet_type: PacketType,
    packet_number: String
}

pub struct EventVersionUpdate {
    old: String,
    new: String
}

pub struct AlpnUpdate {
    old: String,
    new: String
}

enum_str!(PacketType {
    Initial("initial"),

    Handshake("handshake"),

    ZeroRtt("0RTT"),

    OneRtt("1RTT"),

    Retry("retry"),

    VersionNegotiation("version_negotiation"),

    Unknown("unknown"),
});

// ================================================================== //
// RECOVERY

pub struct EventMetricUpdate {
    cwnd: Option<u64>,
    bytes_in_flight: Option<u64>,

    min_rtt: Option<u64>,
    smoothed_rtt: Option<u64>,
    latest_rtt: Option<u64>,
    max_ack_delay: Option<u64>,

    rtt_variance: Option<u64>,
    ssthresh: Option<u64>,

    pacing_rate: Option<u64>,
}

pub struct EventPacketLost {
    packet_type: PacketType,
    packet_number: String,

    header: Option<PacketHeader>,
    frames: Vec<QuicFrame>,
}

// ================================================================== //
// HTTP/3

enum_str!(HTTP3EventType {
    StreamStateUpdate("stream_state_update"),

    StreamTypeUpdate("stream_type_update"),

    FrameCreated("frame_created"),

    FrameParsed("frame_parsed"),

    DataMoved("data_moved"),

    DatagramReceived("data_received"),
});

pub struct EventH3FrameCreated {
    stream_id: String,
    frame: Http3Frame,
    byte_length: String,

    raw: Option<String>
}

pub struct EventH3FrameParsed {
    stream_id: String,
    frame: Http3Frame,
    byte_length: String,

    raw: Option<String>
}

pub struct EventH3DataMoved {
    stream_id: String,
    offset_start: String,
    offset_end: String,

    recipient: String
}

pub struct EventH3DataReceived {
    stream_id: String,
    offset_start: String,
    offset_end: String,

    source: String
}

// ================================================================== //
// Based on QUIC draft-22
// ================================================================== //

enum_str!(QuicFrameTypeName {
    Padding("padding"),

    Ping("ping"),

    Ack("ack"),

    ResetStream("reset_stream"),

    StopSending("stop_sending"),
    Crypto("crypto"),
    NewToken("new_token"),
    Stream("stream"),
    MaxData("max_data"),
    MaxStreamData("max_stream_data"),
    MaxStreams("max_streams"),
    DataBlocked("data_blocked"),
    StreamDataBlocked("stream_data_blocked"),
    StreamsBlocked("streams_blocked"),
    NewConnectionId("new_connection_id"),
    RetireConnectionId("retire_connection_id"),
    PathChallenge("path_challenge"),
    PathResponse("path_response"),
    ConnectionClose("connection_close"),
    ApplicationClose("application_close"),
    UnknownFrameType("unkown_frame_type"),
});

// TODO: search for pub enum Error { to see how best to encode errors in qlog.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PacketHeader {
    pub packet_number: String,
    pub packet_size: Option<u64>,
    pub payload_length: Option<u64>,

    pub version: Option<String>,
    pub scil: Option<String>,
    pub dcil: Option<String>,
    pub scid: Option<String>,
    pub dcid: Option<String>,
}

enum_str!(StreamType {
    Bidirectional("bidirectional"),
    Unidirectional("unidirectional"),
});

enum_str!(ErrorSpace {
    TransportError("transport_error"),
    ApplicationError("application_error"),
});

#[derive(serde::Serialize, serde::Deserialize)]
pub enum QuicFrame {
    Padding {
        frame_type: QuicFrameTypeName,
    },

    Ping {
        frame_type: QuicFrameTypeName,
    },

    Ack {
        frame_type: QuicFrameTypeName,
        ack_delay: String,
        acked_ranges: Vec<(u64,u64)>,
        ect1: Option<String>,
        ect0: Option<String>,
        ce: Option<String>
    },

    ResetStream {
        frame_type: QuicFrameTypeName,
        id: String,
        error_code: u64,
        final_size: String
    },

    StopSending {
        frame_type: QuicFrameTypeName,
        id: String,
        error_code: u64,
    },

    Crypto {
        frame_type: QuicFrameTypeName,
        offset: String,
        length: String,
    },

    NewToken {
        frame_type: QuicFrameTypeName,
        length: String,
        token: String,
    },

    Stream {
        frame_type: QuicFrameTypeName,
        id: String,
        offset: String,
        length: String,
        fin: bool,
        raw: Option<String>,
    },

    MaxData {
        frame_type: QuicFrameTypeName,
        maximum: String,
    },

    MaxStreamData {
        frame_type: QuicFrameTypeName,
        id: String,
        maximum: String,
    },

    MaxStreams {
        frame_type: QuicFrameTypeName,
        stream_type: StreamType,
        maximum: String,
    },

    Blocked {
        frame_type: QuicFrameTypeName,
        limit: String,
    },

    StreamDataBlocked {
        frame_type: QuicFrameTypeName,
        id: String,
        limit: String,
    },

    StreamsBlocked {
        frame_type: QuicFrameTypeName,
        stream_type: StreamType,
        limit: String,
    },

    NewConnectionId {
        frame_type: QuicFrameTypeName,
        sequence_number: String,
        retire_prior_to: String,
        length: u64,
        connection_id: String,
        reset_token: String
    },

    RetireConnectionId {
        frame_type: QuicFrameTypeName,
        sequence_number: String,
    },

    PathChallenge {
        frame_type: QuicFrameTypeName,
        data: Option<String>,
    },

    PathResponse {
        frame_type: QuicFrameTypeName,
        data: Option<String>,
    },

    ConnectionClose {
        frame_type: QuicFrameTypeName,
        error_space: ErrorSpace,
        error_code: u64,
        raw_error_code: u64,
        reason: String,

        trigger_frame_type: Option<u64>
    },

    Unknown {
        frame_type: QuicFrameTypeName,
        raw_frame_type: u64
    }
}

// ================================================================== //

enum_str!(HTTP3FrameTypeName {
    Data("data"),

    Headers("headers"),

    CancelPush("cencel_push"),

    Settings("settings"),

    PushPromise("push_promise"),
    Goaway("goaway"),
    MaxPushId("max_push_id"),
    DuplicatePush("duplicate_push"),
    Reserved("Reserved"),
    Unknown("unknown"),
});

pub struct HttpHeader {
    name: String,
    content: String,
}

pub struct Setting {
    name: String,
    content: String,
}

pub enum Http3Frame {
    Data {
        frame_type: HTTP3FrameTypeName,
    },

    Headers {
        frame_type: HTTP3FrameTypeName,
        fields: Vec<HttpHeader>,
    },

    CancelPush {
        frame_type: HTTP3FrameTypeName,
        id: String
    },

    Settings {
        frame_type: HTTP3FrameTypeName,
        fields: Vec<Setting>
    },

    PushPromise {
        frame_type: HTTP3FrameTypeName,
        id: String,
        fields: Vec<HttpHeader>,
    },

    GoAway {
        frame_type: HTTP3FrameTypeName,
        id: String,
    },

    MaxPushId {
        frame_type: HTTP3FrameTypeName,
        id: String,
    },

    Reserved {
        frame_type: HTTP3FrameTypeName,
    },
}
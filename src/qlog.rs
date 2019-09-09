// Copyright (C) 2019, Cloudflare, Inc.
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

// use serde::ser::{
// Serialize,
// SerializeStruct,
// Serializer,
// };

#[serde_with::skip_serializing_none]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Qlog {
    pub qlog_version: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub summary: Option<String>,

    pub traces: Vec<Trace>,
}

#[serde_with::skip_serializing_none]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Trace {
    pub vantage_point: VantagePoint,
    pub title: Option<String>,
    pub description: Option<String>,

    pub configuration: Option<Configuration>,

    pub common_fields: Option<CommonFields>,
    pub event_fields: Vec<String>,

    pub events: Vec<Vec<EventField>>,
}

impl Trace {
    fn push_event(
        &mut self, relative_time: String, category: EventCategory,
        event: EventType, trigger: EventTrigger, data: EventData,
    ) {
        self.events.push(vec![
            EventField::RelativeTime(relative_time),
            EventField::Category(category),
            EventField::Event(event),
            EventField::Trigger(trigger),
            EventField::Data(data),
        ]);
    }

    pub fn push_connectivity_event(
        &mut self, relative_time: String, event: ConnectivityEventType,
        trigger: ConnectivityEventTrigger, data: EventData,
    ) {
        self.push_event(
            relative_time,
            EventCategory::Connectivity,
            EventType::ConnectivityEventType(event),
            EventTrigger::ConnectivityEventTrigger(trigger),
            data,
        );
    }

    pub fn push_transport_event(
        &mut self, relative_time: String, event: TransportEventType,
        trigger: TransportEventTrigger, data: EventData,
    ) {
        self.push_event(
            relative_time,
            EventCategory::Transport,
            EventType::TransportEventType(event),
            EventTrigger::TransportEventTrigger(trigger),
            data,
        );
    }

    pub fn push_security_event(
        &mut self, relative_time: String, event: SecurityEventType,
        trigger: SecurityEventTrigger, data: EventData,
    ) {
        self.push_event(
            relative_time,
            EventCategory::Security,
            EventType::SecurityEventType(event),
            EventTrigger::SecurityEventTrigger(trigger),
            data,
        );
    }

    pub fn push_recovery_event(
        &mut self, relative_time: String, event: RecoveryEventType,
        trigger: RecoveryEventTrigger, data: EventData,
    ) {
        self.push_event(
            relative_time,
            EventCategory::Recovery,
            EventType::RecoveryEventType(event),
            EventTrigger::RecoveryEventTrigger(trigger),
            data,
        );
    }
}

#[serde_with::skip_serializing_none]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct VantagePoint {
    pub name: Option<String>,

    #[serde(rename = "type")]
    pub ty: VantagePointType,

    pub flow: Option<VantagePointType>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VantagePointType {
    Client,
    Server,
    Network,
    Unknown,
}

#[allow(dead_code)]
#[serde_with::skip_serializing_none]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Configuration {
    pub time_offset: Option<String>,
    pub time_units: Option<String>,

    pub original_uris: Option<Vec<String>>,
}

#[serde_with::skip_serializing_none]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct CommonFields {
    pub group_id: Option<String>,
    pub group_ids: Option<Vec<String>>,
    pub protocol_type: Option<String>,

    pub reference_time: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum EventType {
    ConnectivityEventType(ConnectivityEventType),

    TransportEventType(TransportEventType),

    SecurityEventType(SecurityEventType),

    RecoveryEventType(RecoveryEventType),

    Http3EventType(Http3EventType),
}

#[derive(Debug)]
pub enum FunnyField {
    A(X),
    B(Y),
    C(Z),
}

#[derive(Debug)]
pub enum A {
    Alpha(X),
    Beta(X),
}

#[derive(Debug)]
pub enum B {
    Delta(Y),
    Gamma(Y),
}

#[derive(Debug)]
pub enum C {
    Epsilon(Z),
    Zeta(Z),
}

#[derive(Debug)]
pub enum X {
    Up(String),
    Down(String),
}

#[derive(Debug)]
pub enum Y {
    Charm(u8),
    Strange(u8),
}

#[derive(Debug)]
pub enum Z {
    Top(u16),
    Bottom(u16),
}

#[allow(dead_code)]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum EventTrigger {
    ConnectivityEventTrigger(ConnectivityEventTrigger),

    TransportEventTrigger(TransportEventTrigger),

    SecurityEventTrigger(SecurityEventTrigger),

    RecoveryEventTrigger(RecoveryEventTrigger),
}

#[allow(dead_code)]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum EventField {
    RelativeTime(String),

    Category(EventCategory),

    Event(EventType),

    Trigger(EventTrigger),

    Data(EventData),
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EventCategory {
    Connectivity,
    Security,
    Transport,
    Recovery,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConnectivityEventType {
    ConnectionAttempt,
    ConnectionNew,
    ConnectionIdUpdate,
    SpinBitUpdate,
    ConnectionClose,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConnectivityEventTrigger {
    Line,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TransportEventType {
    DatagramSent,
    DatagramReceived,
    PacketSent,
    PacketReceived,
    PacketDropped,
    PacketBuffered,
    StreamStateUpdate,
    FlowControlUpdate,
    VersionUpdate,
    TransportParametersUpdate,
    AlpnUpdate,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TransportEventTrigger {
    Line,
    Retransmit,
    KeysUnavailable,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SecurityEventType {
    CipherUpdate,
    KeyUpdate,
    KeyRetire,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SecurityEventTrigger {
    Tls,
    Implicit,
    RemoteUpdate,
    LocalUpdate,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecoveryEventType {
    StateUpdate,
    MetricUpdate,
    LossAlarmSet,
    LossAlarmTriggered,
    PacketLost,
    PacketAcknowledged,
    PacketRetransmit,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecoveryEventTrigger {
    AckReceived,
    PacketSent,
    Alarm,
    Unknown,
}

// ================================================================== //

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum KeyType {
    ServerInitialSecret,
    ClientInitialSecret,
    ServerHandshakeSecret,
    ClientHandshakeSecret,
    Server0RttSecret,
    Client0RttSecret,
    Server1RttSecret,
    Client1RttSecret,
}

#[serde_with::skip_serializing_none]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum EventData {
    // ================================================================== //
    // CONNECTIVITY
    Listening {
        ip: String,
        port: u64,

        quic_versions: Option<Vec<String>>,
        alpn_values: Option<Vec<String>>,
    },

    ConnectionNew {
        ip_verison: String,
        src_ip: String,
        dst_ip: String,

        transport_protocol: Option<String>,
        src_port: u64,
        dst_port: u64,

        quic_version: Option<String>,
        src_cid: Option<String>,
        dst_cid: Option<String>,
    },

    ConnectionIdUpdate {
        src_old: Option<String>,
        src_new: Option<String>,

        dst_old: Option<String>,
        dst_new: Option<String>,
    },

    SpinBitUpdate {
        state: bool,
    },

    ConnectionClose {
        src_id: String,
    },

    // ================================================================== //
    // SECURITY
    CipherUpdate {
        cipher_type: String,
    },

    KeyRetire {
        key_type: KeyType,
        key: String,
        generation: Option<String>,
    },

    KeyUpdate {
        key_type: KeyType,
        old: Option<String>,
        new: String,
        generation: Option<String>,
    },

    // ================================================================== //
    // TRANSPORT
    DatagramReceived {
        count: Option<u64>,
        byte_length: u64,
    },

    DatagramSent {
        count: Option<u64>,
        byte_length: u64,
    },

    PacketReceived {
        raw_encrypted: Option<String>,

        packet_type: PacketType,
        header: Option<PacketHeader>,
        frames: Option<Vec<QuicFrame>>,
    },

    PacketSent {
        raw_encrypted: Option<String>,

        packet_type: PacketType,
        header: Option<PacketHeader>,
        frames: Option<Vec<QuicFrame>>,
    },

    PacketBuffered {
        packet_type: PacketType,
        packet_number: String,
    },

    VersionUpdate {
        old: String,
        new: String,
    },

    // ================================================================== //
    // RECOVERY
    MetricUpdate {
        cwnd: Option<u64>,
        bytes_in_flight: Option<u64>,

        min_rtt: Option<u64>,
        smoothed_rtt: Option<u64>,
        latest_rtt: Option<u64>,
        max_ack_delay: Option<u64>,

        rtt_variance: Option<u64>,
        ssthresh: Option<u64>,

        pacing_rate: Option<u64>,
    },

    PacketLost {
        packet_type: PacketType,
        packet_number: String,

        header: Option<PacketHeader>,
        frames: Vec<QuicFrame>,
    },

    // ================================================================== //
    // HTTP/3
    H3FrameCreated {
        stream_id: String,
        frame: Http3Frame,
        byte_length: String,

        raw: Option<String>,
    },

    H3FrameParsed {
        stream_id: String,
        frame: Http3Frame,
        byte_length: String,

        raw: Option<String>,
    },

    H3DataMoved {
        stream_id: String,
        offset_start: String,
        offset_end: String,

        recipient: String,
    },

    H3DataReceived {
        stream_id: String,
        offset_start: String,
        offset_end: String,

        source: String,
    },
}

#[allow(dead_code)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct AlpnUpdate {
    old: String,
    new: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PacketType {
    Initial,
    Handshake,

    #[serde(rename = "0RTT")]
    ZeroRtt,

    #[serde(rename = "1RTT")]
    OneRtt,

    Retry,
    VersionNegotiation,
    Unknown,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Http3EventType {
    StreamStateUpdate,
    StreamTypeUpdate,
    FrameCreated,
    FrameParsed,
    DataMoved,
    DatagramReceived,
}

// ================================================================== //
// Based on QUIC draft-22
// ================================================================== //

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QuicFrameTypeName {
    Padding,
    Ping,
    Ack,
    ResetStream,
    StopSending,
    Crypto,
    NewToken,
    Stream,
    MaxData,
    MaxStreamData,
    MaxStreams,
    DataBlocked,
    StreamDataBlocked,
    StreamsBlocked,
    NewConnectionId,
    RetireConnectionId,
    PathChallenge,
    PathResponse,
    ConnectionClose,
    ApplicationClose,
    Unknown,
}

// TODO: search for pub enum Error { to see how best to encode errors in qlog.
#[serde_with::skip_serializing_none]
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

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StreamType {
    Bidirectional,
    Unidirectional,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ErrorSpace {
    TransportError,
    ApplicationError,
}

#[serde_with::skip_serializing_none]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
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
        acked_ranges: Vec<(u64, u64)>,

        ect1: Option<String>,

        ect0: Option<String>,

        ce: Option<String>,
    },

    ResetStream {
        frame_type: QuicFrameTypeName,
        id: String,
        error_code: u64,
        final_size: String,
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

    DataBlocked {
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
        reset_token: String,
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

        trigger_frame_type: Option<u64>,
    },

    Unknown {
        frame_type: QuicFrameTypeName,
        raw_frame_type: u64,
    },
}

impl QuicFrame {
    pub fn padding() -> Self {
        QuicFrame::Padding {
            frame_type: QuicFrameTypeName::Padding,
        }
    }

    pub fn ping() -> Self {
        QuicFrame::Ping {
            frame_type: QuicFrameTypeName::Ping,
        }
    }

    pub fn ack(
        ack_delay: String, acked_ranges: Vec<(u64, u64)>, ect1: Option<String>,
        ect0: Option<String>, ce: Option<String>,
    ) -> Self {
        QuicFrame::Ack {
            frame_type: QuicFrameTypeName::Ack,
            ack_delay,
            acked_ranges,
            ect1,
            ect0,
            ce,
        }
    }

    pub fn reset_stream(id: String, error_code: u64, final_size: String) -> Self {
        QuicFrame::ResetStream {
            frame_type: QuicFrameTypeName::ResetStream,
            id,
            error_code,
            final_size,
        }
    }

    pub fn stop_sending(id: String, error_code: u64) -> Self {
        QuicFrame::StopSending {
            frame_type: QuicFrameTypeName::StopSending,
            id,
            error_code,
        }
    }

    pub fn crypto(offset: String, length: String) -> Self {
        QuicFrame::Crypto {
            frame_type: QuicFrameTypeName::Crypto,
            offset,
            length,
        }
    }

    pub fn new_token(length: String, token: String) -> Self {
        QuicFrame::NewToken {
            frame_type: QuicFrameTypeName::NewToken,
            length,
            token,
        }
    }

    pub fn stream(
        id: String, offset: String, length: String, fin: bool,
        raw: Option<String>,
    ) -> Self {
        QuicFrame::Stream {
            frame_type: QuicFrameTypeName::Stream,
            id,
            offset,
            length,
            fin,
            raw,
        }
    }

    pub fn max_data(maximum: String) -> Self {
        QuicFrame::MaxData {
            frame_type: QuicFrameTypeName::MaxData,
            maximum,
        }
    }

    pub fn max_stream_data(id: String, maximum: String) -> Self {
        QuicFrame::MaxStreamData {
            frame_type: QuicFrameTypeName::MaxStreamData,
            id,
            maximum,
        }
    }

    pub fn max_streams(stream_type: StreamType, maximum: String) -> Self {
        QuicFrame::MaxStreams {
            frame_type: QuicFrameTypeName::MaxStreams,
            stream_type,
            maximum,
        }
    }

    pub fn data_blocked(limit: String) -> Self {
        QuicFrame::DataBlocked {
            frame_type: QuicFrameTypeName::DataBlocked,
            limit,
        }
    }

    pub fn stream_data_blocked(id: String, limit: String) -> Self {
        QuicFrame::StreamDataBlocked {
            frame_type: QuicFrameTypeName::StreamDataBlocked,
            id,
            limit,
        }
    }

    pub fn streams_blocked(stream_type: StreamType, limit: String) -> Self {
        QuicFrame::StreamsBlocked {
            frame_type: QuicFrameTypeName::StreamsBlocked,
            stream_type,
            limit,
        }
    }

    pub fn new_connection_id(
        sequence_number: String, retire_prior_to: String, length: u64,
        connection_id: String, reset_token: String,
    ) -> Self {
        QuicFrame::NewConnectionId {
            frame_type: QuicFrameTypeName::NewConnectionId,
            sequence_number,
            retire_prior_to,
            length,
            connection_id,
            reset_token,
        }
    }

    pub fn retire_connection_id(sequence_number: String) -> Self {
        QuicFrame::RetireConnectionId {
            frame_type: QuicFrameTypeName::RetireConnectionId,
            sequence_number,
        }
    }

    pub fn path_challenge(data: Option<String>) -> Self {
        QuicFrame::PathChallenge {
            frame_type: QuicFrameTypeName::PathChallenge,
            data,
        }
    }

    pub fn path_response(data: Option<String>) -> Self {
        QuicFrame::PathResponse {
            frame_type: QuicFrameTypeName::PathResponse,
            data,
        }
    }

    pub fn connection_close(
        error_space: ErrorSpace, error_code: u64, raw_error_code: u64,
        reason: String, trigger_frame_type: Option<u64>,
    ) -> Self {
        QuicFrame::ConnectionClose {
            frame_type: QuicFrameTypeName::ConnectionClose,
            error_space,
            error_code,
            raw_error_code,
            reason,
            trigger_frame_type,
        }
    }

    pub fn unknown(raw_frame_type: u64) -> Self {
        QuicFrame::Unknown {
            frame_type: QuicFrameTypeName::Unknown,
            raw_frame_type,
        }
    }
}

// ================================================================== //
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Http3FrameTypeName {
    Data,
    Headers,
    CancelPush,
    Settings,
    PushPromise,
    Goaway,
    MaxPushId,
    DuplicatePush,
    Reserved,
    Unknown,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct HttpHeader {
    name: String,
    content: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Setting {
    name: String,
    content: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub enum Http3Frame {
    Data {
        frame_type: Http3FrameTypeName,
    },

    Headers {
        frame_type: Http3FrameTypeName,
        fields: Vec<HttpHeader>,
    },

    CancelPush {
        frame_type: Http3FrameTypeName,
        id: String,
    },

    Settings {
        frame_type: Http3FrameTypeName,
        fields: Vec<Setting>,
    },

    PushPromise {
        frame_type: Http3FrameTypeName,
        id: String,
        fields: Vec<HttpHeader>,
    },

    Goaway {
        frame_type: Http3FrameTypeName,
        id: String,
    },

    MaxPushId {
        frame_type: Http3FrameTypeName,
        id: String,
    },

    Reserved {
        frame_type: Http3FrameTypeName,
    },

    Unknown {
        frame_type: Http3FrameTypeName,
    },
}

impl Http3Frame {
    fn data() -> Self {
        Http3Frame::Data {
            frame_type: Http3FrameTypeName::Data,
        }
    }

    fn headers(fields: Vec<HttpHeader>) -> Self {
        Http3Frame::Headers {
            frame_type: Http3FrameTypeName::Headers,
            fields,
        }
    }

    fn cancel_push(id: String) -> Self {
        Http3Frame::CancelPush {
            frame_type: Http3FrameTypeName::CancelPush,
            id,
        }
    }

    fn settings(fields: Vec<Setting>) -> Self {
        Http3Frame::Settings {
            frame_type: Http3FrameTypeName::Settings,
            fields,
        }
    }

    fn push_promise(id: String, fields: Vec<HttpHeader>) -> Self {
        Http3Frame::PushPromise {
            frame_type: Http3FrameTypeName::PushPromise,
            id,
            fields,
        }
    }

    fn goaway(id: String) -> Self {
        Http3Frame::Goaway {
            frame_type: Http3FrameTypeName::Goaway,
            id,
        }
    }

    fn max_push_id(id: String) -> Self {
        Http3Frame::MaxPushId {
            frame_type: Http3FrameTypeName::MaxPushId,
            id,
        }
    }

    fn reserved() -> Self {
        Http3Frame::Reserved {
            frame_type: Http3FrameTypeName::Reserved,
        }
    }

    fn unknown() -> Self {
        Http3Frame::Unknown {
            frame_type: Http3FrameTypeName::Unknown,
        }
    }
}

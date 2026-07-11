//! Round-trip witnesses for the Listener meta signal contract.

use meta_signal_listener::{
    ConfigurationGeneration, ConfigurationRejected, ConfigurationRejectionReason, Frame, FrameBody,
    Generation, Input, OperationKind, Output, Reason, RejectionReason, RequestUnimplemented,
    UnimplementedOperationKind, UnimplementedReason,
};
use nota::{NotaDecode, NotaEncode, NotaSource};
use signal_frame::{
    ExchangeIdentifier, ExchangeLane, LaneSequence, NonEmpty, Reply, RequestPayload, SessionEpoch,
    SubReply,
};
use signal_listener::{
    CaptureStoreDirectory, InputSource, ListenerDaemonConfiguration, MetaSocketMode,
    MetaSocketPath, OutputTarget, OutputTargets, SocketMode, TranscriptionMode, WirePath,
    WorkingSocketMode, WorkingSocketPath,
};

struct ListenerConfigurationFixture;

impl ListenerConfigurationFixture {
    fn exchange() -> ExchangeIdentifier {
        ExchangeIdentifier::new(
            SessionEpoch::new(1),
            ExchangeLane::Connector,
            LaneSequence::first(),
        )
    }

    fn path(value: &str) -> WirePath {
        WirePath::new(value.to_owned())
    }

    fn configuration() -> ListenerDaemonConfiguration {
        ListenerDaemonConfiguration {
            working_socket_path: WorkingSocketPath::new(Self::path("/run/persona/X/listener.sock")),
            working_socket_mode: WorkingSocketMode::new(SocketMode::new(0o660)),
            meta_socket_path: MetaSocketPath::new(Self::path("/run/persona/X/listener-meta.sock")),
            meta_socket_mode: MetaSocketMode::new(SocketMode::new(0o600)),
            capture_store_directory: CaptureStoreDirectory::new(Self::path(
                "/var/lib/persona/listener/captures",
            )),
            input_source: InputSource::SystemDefault,
            transcription_mode: TranscriptionMode::BatchOnStop,
            output_targets: OutputTargets::new(vec![OutputTarget::SystemClipboard]),
        }
    }

    fn assert_request_round_trips(request: Input) {
        let frame = Frame::new(FrameBody::Request {
            exchange: Self::exchange(),
            request: request.clone().into_request(),
        });
        let bytes = frame.encode_length_prefixed().expect("encode request");
        let decoded = Frame::decode_length_prefixed(&bytes).expect("decode request");
        match decoded.into_body() {
            FrameBody::Request {
                request: decoded_request,
                ..
            } => assert_eq!(decoded_request.payloads().head(), &request),
            other => panic!("expected request frame, got {other:?}"),
        }
    }

    fn assert_reply_round_trips(reply: Output) {
        let frame = Frame::new(FrameBody::Reply {
            exchange: Self::exchange(),
            reply: Reply::committed(NonEmpty::single(SubReply::Ok(reply.clone()))),
        });
        let bytes = frame.encode_length_prefixed().expect("encode reply");
        let decoded = Frame::decode_length_prefixed(&bytes).expect("decode reply");
        match decoded.into_body() {
            FrameBody::Reply {
                reply: decoded_reply,
                ..
            } => match decoded_reply {
                Reply::Accepted { per_operation, .. } => match per_operation.into_head() {
                    SubReply::Ok(payload) => assert_eq!(payload, reply),
                    other => panic!("expected accepted reply payload, got {other:?}"),
                },
                Reply::Rejected { reason } => panic!("unexpected rejected reply: {reason:?}"),
            },
            other => panic!("expected reply frame, got {other:?}"),
        }
    }

    fn assert_nota_round_trips<Value>(value: &Value)
    where
        Value: NotaEncode + NotaDecode + PartialEq + std::fmt::Debug,
    {
        let text = value.to_nota();
        let recovered = NotaSource::new(&text).parse::<Value>().expect("decode");
        assert_eq!(&recovered, value);
    }
}

#[test]
fn configure_request_carries_listener_daemon_configuration() {
    let request = Input::Configure(ListenerConfigurationFixture::configuration());
    assert_eq!(request.kind(), OperationKind::Configure);
    ListenerConfigurationFixture::assert_request_round_trips(request.clone());
    ListenerConfigurationFixture::assert_nota_round_trips(&request);
}

#[test]
fn reply_variants_round_trip() {
    let replies = [
        Output::accepted(Generation::new(ConfigurationGeneration::new(7))),
        Output::Rejected(ConfigurationRejected::new(RejectionReason::new(
            ConfigurationRejectionReason::OwnerAuthorityRequired,
        ))),
        Output::Unimplemented(RequestUnimplemented {
            unimplemented_operation_kind: UnimplementedOperationKind::new(OperationKind::Configure),
            reason: Reason::new(UnimplementedReason::DependencyNotReady),
        }),
    ];

    for reply in replies {
        ListenerConfigurationFixture::assert_reply_round_trips(reply.clone());
        ListenerConfigurationFixture::assert_nota_round_trips(&reply);
    }
}

#[test]
fn configuration_generation_projects_to_integer() {
    let generation = ConfigurationGeneration::new(11);
    assert_eq!(generation.value(), 11);
}

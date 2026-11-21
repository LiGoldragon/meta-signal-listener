#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    clippy::type_complexity
)]
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Set {
    Backend(listener_library::Backend),
    RemoveBackend(listener_library::BackendName),
    Order(std::vec::Vec<listener_library::BackendName>),
    Delivery(listener_library::Delivery),
    Sequence(listener_library::Sequence),
    AudioKept(listener_library::Retention),
    TextKept(listener_library::Retention),
    AddTerm(listener_library::Term),
    RemoveTerm(listener_library::Term),
    Kilobits(listener_library::Kilobits),
    Source(listener_library::Source),
    HistoryLength(listener_library::HistoryLength),
    PasteProgram(listener_library::Program),
    CopyProgram(listener_library::Program),
    SecretProgram(listener_library::Program),
    AudioDirectory(listener_library::Directory),
    Budget(translator::DatomReadingBudget),
    RecordingSettings(listener_library::RecordingSettings),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Remember_Data {
    pub moment: listener_library::Moment,
    pub text: listener_library::Text,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Input {
    Configure(Set),
    Configuration,
    Remember(Remember_Data),
    Subscribe,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Refused_Data {
    UnknownBackend(listener_library::UnknownBackend),
    InvalidValue,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Output {
    Configured(Set),
    Configuration(listener_library::Configuration),
    Remembered(listener_library::JobId),
    Subscribed(signal::Frame<listener_library::Configuration, Set>),
    Refused(Refused_Data),
}
#[rustfmt::skip]
#[allow(clippy::large_enum_variant)]
pub enum Answer {
    Once(Output),
    Subscribed(signal::Stream<listener_library::Configuration, Set>),
}
#[rustfmt::skip]
#[allow(clippy::result_large_err, clippy::ptr_arg, clippy::borrowed_box)]
pub trait Answering {
    fn configure(&self, set: &Set) -> std::result::Result<Set, Output>;
    fn configuration(
        &self,
    ) -> std::result::Result<listener_library::Configuration, Output>;
    fn remember(
        &self,
        remember: &Remember_Data,
    ) -> std::result::Result<listener_library::JobId, Output>;
    fn subscribe(
        &self,
    ) -> std::result::Result<
        signal::Stream<listener_library::Configuration, Set>,
        Output,
    >;
}
#[rustfmt::skip]
pub trait Answerable {
    type Answer;
    fn answer<Nexus: Answering>(&self, nexus: &Nexus) -> Self::Answer;
}
#[rustfmt::skip]
impl Answerable for Input {
    type Answer = Answer;
    fn answer<Nexus: Answering>(&self, nexus: &Nexus) -> Answer {
        match self {
            Input::Configure(carried) => {
                Answer::Once(
                    nexus
                        .configure(carried)
                        .map_or_else(|refusal| refusal, Output::Configured),
                )
            }
            Input::Configuration => {
                Answer::Once(
                    nexus
                        .configuration()
                        .map_or_else(|refusal| refusal, Output::Configuration),
                )
            }
            Input::Remember(carried) => {
                Answer::Once(
                    nexus
                        .remember(carried)
                        .map_or_else(|refusal| refusal, Output::Remembered),
                )
            }
            Input::Subscribe => {
                nexus.subscribe().map_or_else(Answer::Once, Answer::Subscribed)
            }
        }
    }
}

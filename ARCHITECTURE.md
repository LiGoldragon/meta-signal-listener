# meta-signal-listener - architecture

`meta-signal-listener` is the owner/meta wire contract for privileged Listener
configuration. It is the authority companion to `signal-listener`.

## Role

The meta surface starts with the single operation needed by the first Listener
slice:

```text
MetaListenerOperation                    MetaListenerReply
Configure(ListenerDaemonConfiguration)   Configured(ConfigurationGeneration)
                                          ConfigurationRejected(reason)
                                          RequestUnimplemented(reason)
```

The configuration record is imported from `signal-listener` so daemon startup
and meta reconfiguration share one typed identity. The initial configuration
names the working socket, meta socket, capture store directory, default input,
batch-on-stop transcription mode, and configured output targets starting with
`SystemClipboard`.

## Owned

- Listener owner/meta wire vocabulary.
- The `Configure(ListenerDaemonConfiguration)` operation.
- Typed configuration acknowledgement, rejection, and unimplemented replies.
- Optional NOTA projection behind `nota-text`.

## Not Owned

- Ordinary start/stop/status capture traffic lives in `signal-listener`.
- Audio capture, durable disk write, transcription execution, clipboard
  mutation, sockets, and daemon state live in `listener`.
- Schema generation machinery lives in `schema-rust`.

## Code Map

- `schema/lib.schema` is the authored meta contract vocabulary.
- `build.rs` imports the `signal-listener` schema directory through Cargo
  metadata and runs `schema-rust`.
- `src/schema/lib.rs` is the generated checked-in artifact.
- `src/lib.rs` re-exports generated nouns and adds small accessors/aliases.
- `tests/round_trip.rs` proves frame and NOTA round trips.

## Invariants

- This crate is wire-only: no daemon runtime, no actors, no storage, no Tokio.
- The meta contract reuses `signal_listener::ListenerDaemonConfiguration`; it
  does not mirror the configuration record.
- Default builds are binary-first; NOTA projection is behind `nota-text`.

# meta-signal-listener

`meta-signal-listener` is the owner/meta Signal contract for privileged Listener
configuration. Ordinary capture and transcription traffic stays in
`signal-listener`.

The checked-in generated schema artifact is refreshed with:

```sh
META_SIGNAL_LISTENER_UPDATE_SCHEMA_ARTIFACTS=1 cargo build --all-features
```

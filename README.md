# beevulyk-service-sdk-macros

Proc-macro helpers for [`beevulyk-service-sdk`](https://github.com/BeeVulyk/beevulyk-service-sdk).

Provides:

- `#[derive(SdkSettingsTraits)]` — auto-implements `ServiceInfo` (name/version from `CARGO_PKG_*`).
- `#[derive(AutoGenerateSettingsTraits)]` — auto-implements `SqlxSettings`, `RedisSettings`, `KafkaSettings` based on enabled features.
- `generate_settings_signature!()` — expands to the `Arc<impl ServiceInfo + ...>` bound expected by the SDK bootstrap.
- `generate_grpc_service!(App)` — emits an `SdkGrpcService` wrapper around a shared app handle.
- `use_settings!()`, `use_grpc_client!()`, `use_grpc_server!()`, `use_redis!()`, `use_kafka!()` — convenience `use` batches so downstream services don't have to spell out every import.

## Features

- `grpc`, `postgresql`, `sqlite`, `redis`, `kafka` — mirror the feature flags of the parent `beevulyk-service-sdk` crate.

## Fork origin

This crate is a fork of the `yft-service-sdk-macros` sub-crate from
[ITYFT/yft-service-sdk](https://github.com/ITYFT/yft-service-sdk), rebranded for
the BeeVulyk stack. Legacy `use_my_postgres!`, `use_my_http_server!`,
`use_signal_r_json_contract!`, and `use_signal_r_subscriber!` macros were
dropped — those referenced upstream crates that are not part of the BeeVulyk
fork.

## License

MIT OR Apache-2.0

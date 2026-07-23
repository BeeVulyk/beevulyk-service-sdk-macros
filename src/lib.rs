extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro]
#[allow(unused_mut)]
pub fn generate_settings_signature(_item: TokenStream) -> TokenStream {
    let mut traits: Vec<proc_macro2::TokenStream> = vec![];

    #[cfg(feature = "redis")]
    traits.push(quote::quote!(+ RedisSettings));

    #[cfg(feature = "kafka")]
    traits.push(quote::quote!(+ KafkaSettings));

    let result = quote::quote! {
       Arc<impl ServiceInfo #(#traits)* + Send + Sync + 'static>
    };

    result.into()
}

#[proc_macro_derive(SdkSettingsTraits)]
pub fn generate_sdk_settings_traits(input: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(input as syn::DeriveInput);
    let name = &ast.ident;
    quote::quote! {
        #[beevulyk_service_sdk::external::async_trait::async_trait]
        impl beevulyk_service_sdk::ServiceInfo for #name {
            fn get_service_name(&self) -> &str {
                env!("CARGO_PKG_NAME")
            }
            fn get_service_version(&self) -> &str {
                env!("CARGO_PKG_VERSION")
            }
        }
    }
    .into()
}

#[proc_macro_derive(AutoGenerateSettingsTraits)]
#[allow(unused_mut)]
pub fn auto_generate_settings_traits(input: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(input as syn::DeriveInput);
    let name = &ast.ident;
    let mut auto_generates: Vec<proc_macro2::TokenStream> = Vec::new();

    #[cfg(any(feature = "postgresql", feature = "sqlite"))]
    auto_generates.push(quote::quote!(
        #[beevulyk_service_sdk::external::async_trait::async_trait]
        impl beevulyk_service_sdk::SqlxSettings for #name {
            async fn get_connection_string(&self) -> String {
                self.sqlx_connection.clone()
            }
        }
    ));

    #[cfg(feature = "redis")]
    auto_generates.push(quote::quote!(
        #[beevulyk_service_sdk::external::async_trait::async_trait]
        impl beevulyk_service_sdk::RedisSettings for #name {
            async fn get_redis_url(&self) -> String {
                self.redis_url.clone()
            }
        }
    ));

    #[cfg(feature = "kafka")]
    auto_generates.push(quote::quote!(
        #[beevulyk_service_sdk::external::async_trait::async_trait]
        impl beevulyk_service_sdk::KafkaSettings for #name {
            async fn get_kafka_brokers(&self) -> Vec<String> {
                self.kafka_brokers.clone()
            }
        }
    ));

    quote::quote! {
        #(#auto_generates)*
    }
    .into()
}

#[proc_macro]
pub fn use_settings(_input: TokenStream) -> TokenStream {
    quote::quote! {
        use beevulyk_service_sdk::external::async_trait::async_trait;
        use beevulyk_service_sdk::external::envy;
        use beevulyk_service_sdk::macros::SdkSettingsTraits;
        use beevulyk_service_sdk::macros::AutoGenerateSettingsTraits;
    }
    .into()
}

#[proc_macro]
pub fn use_grpc_client(_input: TokenStream) -> TokenStream {
    quote::quote! {
        use beevulyk_service_sdk::external::beevulyk_grpc_extensions;
        use beevulyk_service_sdk::external::beevulyk_grpc_extensions::GrpcClientSettings;
        use beevulyk_service_sdk::external::beevulyk_grpc_extensions::client::generate_grpc_client;
        use beevulyk_service_sdk::external::async_trait;
    }
    .into()
}

#[proc_macro]
pub fn use_grpc_server(_input: TokenStream) -> TokenStream {
    quote::quote! {
        use beevulyk_service_sdk::external::beevulyk_grpc_extensions;
        use beevulyk_service_sdk::external::futures_core;
        use beevulyk_service_sdk::external::async_trait::async_trait;
        use beevulyk_service_sdk::external::beevulyk_grpc_extensions::server::generate_server_stream;
        use beevulyk_service_sdk::external::beevulyk_grpc_extensions::server::with_result_as_stream;
        use beevulyk_service_sdk::external::beevulyk_rust_extensions;
        use beevulyk_service_sdk::external::beevulyk_grpc_extensions::tonic;
    }
    .into()
}

#[proc_macro]
pub fn use_redis(_input: TokenStream) -> TokenStream {
    quote::quote! {
        use beevulyk_service_sdk::external::redis;
        use beevulyk_service_sdk::external::redis::AsyncCommands;
    }
    .into()
}

#[proc_macro]
pub fn use_kafka(_input: TokenStream) -> TokenStream {
    quote::quote! {
        use beevulyk_service_sdk::external::rdkafka;
        use beevulyk_service_sdk::KafkaPublisher;
        use beevulyk_service_sdk::KafkaMessageHandler;
        use beevulyk_service_sdk::KafkaStartOffset;
    }
    .into()
}

#[proc_macro]
pub fn generate_grpc_service(input: TokenStream) -> TokenStream {
    let input: proc_macro2::TokenStream = input.into();

    quote::quote! {
        #[derive(Clone)]
        pub struct SdkGrpcService {
            pub app: std::sync::Arc<#input>,
        }

        impl SdkGrpcService {
            pub fn new(app: std::sync::Arc<#input>) -> Self {
                Self { app }
            }
        }
    }
    .into()
}

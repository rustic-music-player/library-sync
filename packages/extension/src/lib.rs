use std::collections::HashMap;

use rustic_extension_api::*;

#[derive(Debug)]
pub struct LibrarySyncExtension {
    server: String,
}

impl ExtensionLibrary for LibrarySyncExtension {
    fn metadata() -> ExtensionMetadata {
        ExtensionMetadata {
            id: String::from("library-sync"),
            name: String::from("Library Synchronization"),
            version: crate_version!(),
        }
    }

    fn new(config: HashMap<String, ExtensionConfigValue>) -> Box<dyn Extension> {
        let extension = LibrarySyncExtension {
            server: LibrarySyncExtension::get_server_url(config),
        };
        Box::new(extension)
    }
}

impl Extension for LibrarySyncExtension {}

impl ExtensionApi for LibrarySyncExtension {}

impl LibrarySyncExtension {
    fn get_server_url(config: HashMap<String, ExtensionConfigValue>) -> String {
        config
            .get("server")
            .and_then(|value| {
                if let ExtensionConfigValue::String(server_url) = value {
                    Some(server_url.as_str())
                } else {
                    None
                }
            })
            .unwrap_or("https://sync.rustic.maxjoehnk.me")
            .to_string()
    }
}

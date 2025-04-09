//! A logger that emits updates from the indexing engine to a blob in Azure Storage.

use std::any::Any;
use std::collections::HashMap;
use std::path::Path;

use azure_identity::DefaultAzureCredential;
use azure_storage_blob::{BlobClient, BlobServiceClient};
use chrono::prelude::{DateTime, Utc};

use crate::callbacks::workflow_callbacks::WorkflowCallbacks;

/// A logger that writes to a blob storage account.
pub struct BlobWorkflowCallbacks {
    _connection_string: Option<String>,
    _storage_account_blob_url: Option<String>,
    _blob_service_client: BlobServiceClient,
    _blob_name: String,
    _container_name: Option<String>,
    _max_block_count: usize, // 25k blocks per blob
    _blob_client: BlobClient,
    _num_blocks: usize,
}

impl BlobWorkflowCallbacks {
    /// Create a new instance of the BlobStorageReporter class.
    pub fn new(
        connection_string: Option<String>,
        container_name: Option<String>,
        blob_name: &str,                          // = "",
        base_dir: Option<String>,                 // = None,
        storage_account_blob_url: Option<String>, // = None,
    ) -> BlobWorkflowCallbacks {
        assert!(
            container_name.is_some(),
            "No container name provided for blob storage.",
        );
        assert!(
            connection_string.is_some() || storage_account_blob_url.is_some(),
            "No storage account blob url provided for blob storage.",
        );
        let blob_service_client = if let Some(connection_string) = connection_string {
            BlobServiceClient::from_connection_string(connection_string)
        } else {
            assert!(
                storage_account_blob_url.is_some(),
                "Either connection_string or storage_account_blob_url must be provided.",
            );

            BlobServiceClient::new(
                storage_account_blob_url,
                DefaultAzureCredential::new().unwrap(),
                None,
            )
        };

        if blob_name == "" {
            let now: DateTime<Utc> = Utc::now();
            let now = now.format("%Y-%m-%d-%H:%M:%S:%f");
            blob_name = format!("report/{now}.logs.json").as_str();
        }

        let blob_name = Path::new(&base_dir.unwrap_or_default())
            .join(blob_name)
            .to_string_lossy()
            .to_string();
        let blob_client = blob_service_client.get_blob_client(container_name, blob_name);
        if !blob_client.exists() {
            blob_client.create_append_blob();
        }

        BlobWorkflowCallbacks {
            _connection_string: connection_string,
            _storage_account_blob_url: storage_account_blob_url,
            _blob_service_client: blob_service_client,
            _blob_name: blob_name,
            _container_name: container_name,
            _max_block_count: 25000,
            _blob_client: blob_client,
            _num_blocks: 0, // refresh block counter
        }
    }

    fn _write_log(&self, log: HashMap<String, Box<dyn Any>>) {
        // create a new file when block count hits close 25k
        if self._num_blocks >= self._max_block_count {
            // Check if block count exceeds 25k
            self.__init__(
                self._connection_string,
                self._container_name,
                self._storage_account_blob_url,
            )
        }

        let blob_client = self
            ._blob_service_client
            .get_blob_client(self._container_name, self._blob_name);
        blob_client.append_block(serde_json::to_string(log) + "\n");

        // update the blob's block count
        self._num_blocks += 1;
    }
}

impl WorkflowCallbacks for BlobWorkflowCallbacks {
    /// Report an error.
    fn error(
        &self,
        message: String,
        cause: Option<Box<dyn std::error::Error>>,
        stack: Option<String>,
        details: Option<HashMap<String, String>>,
    ) {
        self._write_log(HashMap::from([
            ("type", "error"),
            ("data", message),
            ("cause", cause.map(|e| e.to_string())),
            ("stack", stack),
            ("details", details),
        ]))
    }

    /// Report a warning.
    fn warning(&self, message: String, details: Option<HashMap<String, String>>) {
        self._write_log(HashMap::from([
            ("type", "warning"),
            ("data", message),
            ("details", details),
        ]))
    }

    /// Report a generic log message.
    fn log(&self, message: String, details: Option<HashMap<String, String>>) {
        self._write_log(HashMap::from([
            ("type", "log"),
            ("data", message),
            ("details", details),
        ]))
    }
}

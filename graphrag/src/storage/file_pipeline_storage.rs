//! A module containing 'FileStorage' and 'FilePipelineStorage' models.

use std::any::Any;
use std::collections::HashMap;
use std::path::{PathBuf, Path};

use log::info;

//::aiofiles
// from aiofiles.os::remove
// from aiofiles.ospath::exists

use crate::logger::base::ProgressLogger;
use crate::logger::progress::Progress;
use crate::storage::pipeline_storage::{
    PipelineStorage,
    get_timestamp_formatted_with_local_tz,
};

/// File storage class definition.
pub struct FilePipelineStorage {
    _root_dir: String,
    _encoding: String,
}

impl FilePipelineStorage {
    /// Init method definition.
    pub fn new(root_dir: &str /* = "" */, encoding: &str /*  = "utf-8" */) -> Self {
        std::fs::create_dir_all(root_dir).unwrap();
        FilePipelineStorage {
            _root_dir: root_dir.to_string(),
            _encoding: encoding.to_string(),
        }
    }

    /// Read the contents of a file.
    async fn _read_file<P: AsRef<Path>>(
        self,
        path: P,
        as_bytes: bool, // | None = False,
        encoding: Option<String> /* = None */,
    ) -> Any {
        let read_type = "rb" if as_bytes else "r";
        let encoding = None if as_bytes else (encoding or self._encoding);

        async with aiofiles.open(
            path,
            cast("Any", read_type),
            encoding=encoding,
        ) as f:
            return await f.read()
    }
}

impl<T> PipelineStorage<T> for FilePipelineStorage {
    /// Find files in the storage using a file pattern, as well as a custom filter function.
    fn find(
        &self,
        file_pattern: String, //re::Pattern[String],
        base_dir: Option<String>,
        progress: Option<impl ProgressLogger>,
        file_filter: Option<HashMap<String, Box<dyn Any>>>,
        max_count: usize, // = -1,
    ) -> impl Iterator<Item = (String, HashMap<String, Box<dyn Any>>)> {
        fn item_filter(item: HashMap<String, Box<dyn Any>>) -> bool{
            if file_filter.is_none() {
                return True
            }
            return all(
                re.search(value, item[key]) for key, value in file_filter.items()
            )
        }

        let search_path = Path::new(&self._root_dir).join(base_dir.as_deref().unwrap_or(""));
        info!("search {} for files matching {}", search_path, file_pattern.pattern);
        let all_files = list(search_path.rglob("**/*"));
        let mut num_loaded = 0;
        let num_total = all_files.len();
        let mut num_filtered = 0;
        for file in all_files {
            let match = file_pattern.search(f"{file}");
            if match:
                group = match.groupdict()
                if item_filter(group):
                    filename = f"{file}".replace(self._root_dir, "")
                    if filename.startswith(os.sep):
                        filename = filename[1:]
                    yield (filename, group)
                    num_loaded += 1
                    if max_count > 0 and num_loaded >= max_count:
                        break
                else:
                    num_filtered += 1
            else:
                num_filtered += 1
            if progress is not None:
                progress(_create_progress_status(num_loaded, num_filtered, num_total))
        }
    }

    /// Get method definition.
    async fn get(&self, key: String, as_bytes: Option<bool>, encoding: Option<String>) -> T {
        let file_path = join_path(self._root_dir, key);

        if self.has(key).await {
            return self._read_file(file_path, as_bytes, encoding).await;
        }
        if exists(key).await {
            // Lookup for key, as it is pressumably a new file loaded from inputs
            // and not yet written to storage
            return self._read_file(key, as_bytes, encoding).await;
        }

        None
    }

    /// Set method definition.
    async fn set(&mut self, key: &str, value: T, encoding: Option<String>) {
        let is_bytes = isinstance(value, bytes);
        let write_type = "wb" if is_bytes else "w";
        let encoding = None if is_bytes else encoding or self._encoding;
        async with aiofiles.open(
            join_path(self._root_dir, key),
            cast("Any", write_type),
            encoding=encoding,
        ) as f:
            await f.write(value)
    }

    /// Has method definition.
    async fn has(&self, key: &str) -> bool {
        join_path(&self._root_dir, key).exists()
    }

    /// Delete method definition.
    async fn delete(&mut self, key: &str) {
        if self.has(key).await {
            join_path(&self._root_dir, key).remove()
        }
    }

    /// Clear method definition.
    async fn clear(&mut self) {
        for file in Path::new(self._root_dir).glob("*") {
            if file.is_dir() {
                shutil.rmtree(file)
            } else {
                file.unlink()
            }
        }
    }

    /// Create a child storage instance.
    fn child(&self, name: Option<String>) -> impl PipelineStorage<T> {
        if name.is_none() {
            return self
        }
        FilePipelineStorage::new(
            Path::new(&self._root_dir).join(name.unwrap()),
        )
    }

    /// Return the keys in the storage.
    fn keys(&self) -> Vec<String> {
        std::fs::read_dir(Path::new(&self._root_dir))
            .unwrap()
            .filter_map(Result::ok)
            .filter(|entry| entry.file_type().map(|ft| ft.is_file()).unwrap_or(false))
            .filter_map(|entry| entry.file_name().to_str().map(String::from))
            .collect()
    }

    /// Get the creation date of a file.
    async fn get_creation_date(&self, key: String) -> String {
        let file_path = join_path(&self._root_dir, &key);

        let creation_timestamp = file_path.stat().st_ctime;
        let creation_time_utc = datetime.fromtimestamp(creation_timestamp, tz=timezone.utc);

        get_timestamp_formatted_with_local_tz(creation_time_utc)
    }
}

/// Join a path and a file. Independent of the OS.
pub fn join_path(file_path: &str, file_name: &str) -> PathBuf {
    let path = Path::new(file_path);
    let filename_path = Path::new(file_name);
    path.join(filename_path.parent().unwrap())
        .join(filename_path.file_name().unwrap())
}

/// Create a file based storage.
pub fn create_file_storage<T>(**kwargs: Any) -> impl PipelineStorage<T> {
    let base_dir = kwargs["base_dir"];
    info!("Creating file storage at %s", base_dir);
    FilePipelineStorage {
        root_dir: base_dir,
    }
}

fn _create_progress_status(
    num_loaded: usize, num_filtered: usize, num_total: usize,
) -> Progress {
    Progress {
        percent: None,
        description: Some(format!("{num_loaded} files loaded ({num_filtered} filtered)")),
        total_items: Some(num_total),
        completed_items: Some(num_loaded + num_filtered),
    }
}

use async_std::fs;
use flv_future_aio::task::run_block_on;
use std::io::{Error, ErrorKind};
use std::path::PathBuf;

const LOCAL_CACHE_FILE: &'static str = "rust-mysql-cdc.ofs";

#[derive(Debug)]
pub struct LocalStore {
    file: PathBuf,
    offset: i64,
}

impl LocalStore {
    pub fn init(base_dir: &PathBuf) -> Result<LocalStore, Error> {
        let file = run_block_on(get_or_create_file(&base_dir))?;
        let offset = run_block_on(read_offset(&file))?;

        Ok(Self { file, offset })
    }

    pub fn offset(&self) -> i64 {
        self.offset
    }

    pub fn increment_offset(&mut self) -> Result<(), Error> {
        let new_offset = self.offset + 1;
        run_block_on(write_offset(&self.file, new_offset))?;
        self.offset = new_offset;

        Ok(())
    }
}

async fn get_or_create_file(base_dir: &PathBuf) -> Result<PathBuf, Error> {
    let path = base_dir.join(LOCAL_CACHE_FILE);
    match path.exists() {
        true => Ok(path),
        false => {
            fs::create_dir_all(&base_dir).await?;
            fs::write(&path, "0").await?;
            Ok(path)
        }
    }
}

async fn read_offset(file: &PathBuf) -> Result<i64, Error> {
    let bytes = fs::read(file).await?;
    let data = String::from_utf8(bytes)
        .map_err(|err| Error::new(ErrorKind::InvalidData, format!("{}", err)))?;
    let result = data
        .trim()
        .parse::<i64>()
        .map_err(|err| Error::new(ErrorKind::InvalidData, format!("{}", err)))?;

    Ok(result)
}

async fn write_offset(file: &PathBuf, offset: i64) -> Result<(), Error> {
    fs::write(file, offset.to_string()).await?;
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    use flv_future_aio::task::run_block_on;
    use std::fs;

    const TEST_PATH: &'static str = "test_files";

    fn get_base_dir() -> PathBuf {
        let program_dir = std::env::current_dir().unwrap();
        return program_dir.join(TEST_PATH);
    }

    fn cleanup(file: PathBuf) {
        fs::remove_file(file).expect("delete file failed");
    }

    #[test]
    fn test_local_store_all() {
        run_block_on(test_create_file_write_and_read_offset());
        test_local_store();
    }

    async fn test_create_file_write_and_read_offset() {
        let local_file = get_or_create_file(&get_base_dir()).await;
        if let Err(err) = &local_file {
            println!("{:?}", err);
        };
        assert!(local_file.is_ok());

        let store_file = get_base_dir().join(LOCAL_CACHE_FILE);
        assert_eq!(local_file.unwrap(), store_file);

        let seq = read_offset(&store_file).await;
        assert!(seq.is_ok());
        assert_eq!(seq.unwrap(), 0);

        let res = write_offset(&store_file, 1).await;
        assert!(res.is_ok());

        let seq = read_offset(&store_file).await;
        assert_eq!(seq.unwrap(), 1);

        cleanup(get_base_dir().join(LOCAL_CACHE_FILE));
    }

    fn test_local_store() {
        let base_dir = get_base_dir();
        let local_store = LocalStore::init(&base_dir);
        assert!(local_store.is_ok());

        let mut local_store = local_store.unwrap();
        assert_eq!(local_store.offset(), 0);

        let res = local_store.increment_offset();
        assert!(res.is_ok());
        assert_eq!(local_store.offset(), 1);

        let local_store2 = LocalStore::init(&base_dir);
        assert!(local_store2.is_ok());
        assert_eq!(local_store2.unwrap().offset(), 1);

        cleanup(get_base_dir().join(LOCAL_CACHE_FILE));
    }
}

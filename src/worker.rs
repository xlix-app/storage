use std::future::Future;
use std::sync::Arc;
use super::*;

pub trait IntoStorageWorker<T> {
    fn into_worker(self) -> Arc<T>;
}

pub trait StorageWorkerCore {
    type Args;

    fn new(args: Self::Args) -> Self;
}

pub trait StorageWorker: StorageWorkerCore {
    type Error;
    type UploadMetadata;

    fn is_working(&self) -> bool;
    fn upload_pack(&self, meta: Self::UploadMetadata, data: Vec<u8>) -> impl Future<Output = Result<Pack, Self::Error>> + Send;
    fn download_pack(&self, location: Location) -> impl Future<Output = Result<Vec<u8>, Self::Error>> + Send;
    fn remove_pack(&self, location: Location) -> impl Future<Output = Result<(), Self::Error>> + Send;
}

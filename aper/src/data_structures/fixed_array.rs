use crate::{AperSync, StoreHandle};
use bytes::Bytes;
use serde::{de::DeserializeOwned, Serialize};

pub struct FixedArray<const N: u32, T: Serialize + DeserializeOwned + Default> {
    map: StoreHandle,
    _phantom: std::marker::PhantomData<T>,
}

impl<const N: u32, T> Clone for FixedArray<N, T>
where
    T: Serialize + DeserializeOwned + Default,
{
    fn clone(&self) -> Self {
        Self {
            map: self.map.clone(),
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<const N: u32, T: Serialize + DeserializeOwned + Default> AperSync for FixedArray<N, T> {
    fn attach(map: StoreHandle) -> Self {
        Self {
            map,
            _phantom: std::marker::PhantomData,
        }
    }

    fn listen<F: Fn() -> bool + 'static + Send + Sync>(&self, listener: F) {
        self.map.listen(listener)
    }
}

impl<const N: u32, T: Serialize + DeserializeOwned + Default> FixedArray<N, T> {
    pub fn get(&self, index: u32) -> T {
        if let Some(bytes) = self.map.get(&Bytes::from(index.to_be_bytes().to_vec())) {
            bincode::deserialize(&bytes).unwrap()
        } else {
            T::default()
        }
    }

    pub fn set(&mut self, index: u32, value: T) {
        assert!(index < N);
        let value = Bytes::from(bincode::serialize(&value).unwrap());
        self.map
            .set(Bytes::from(index.to_be_bytes().to_vec()), value);
    }

    pub fn iter(&self) -> FixedArrayIterator<T> {
        FixedArrayIterator {
            tree_ref: self.map.clone(),
            index: 0,
            stop: N,
            _phantom: std::marker::PhantomData,
        }
    }
}

pub struct FixedArrayIterator<T: Serialize + DeserializeOwned + Default> {
    tree_ref: StoreHandle,
    index: u32,
    stop: u32,
    _phantom: std::marker::PhantomData<T>,
}

impl<T: Serialize + DeserializeOwned + Default> Iterator for FixedArrayIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index == self.stop {
            return None;
        }

        let key = self.index.to_be_bytes().to_vec();
        let value = self.tree_ref.get(&Bytes::from(key));
        self.index += 1;

        Some(
            value
                .map(|bytes| bincode::deserialize(&bytes).unwrap())
                .unwrap_or_default(),
        )
    }
}

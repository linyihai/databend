// Copyright 2020-2022 Jorge C. Leitão
// Copyright 2021 Datafuse Labs
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::any::Any;

use crate::arrow::array::Array;
use crate::arrow::array::MutableArray;
use crate::arrow::bitmap::Bitmap;
use crate::arrow::bitmap::MutableBitmap;
use crate::arrow::datatypes::DataType;
use crate::arrow::datatypes::PhysicalType;
use crate::arrow::error::Error;

/// The concrete [`Array`] of [`DataType::Null`].
#[derive(Clone)]
pub struct NullArray {
    data_type: DataType,
    length: usize,
}

impl NullArray {
    /// Returns a new [`NullArray`].
    /// # Errors
    /// This function errors iff:
    /// * The `data_type`'s [`crate::arrow::datatypes::PhysicalType`] is not equal to [`crate::arrow::datatypes::PhysicalType::Null`].
    pub fn try_new(data_type: DataType, length: usize) -> Result<Self, Error> {
        if data_type.to_physical_type() != PhysicalType::Null {
            return Err(Error::oos(
                "NullArray can only be initialized with a DataType whose physical type is Boolean",
            ));
        }

        Ok(Self { data_type, length })
    }

    /// Returns a new [`NullArray`].
    /// # Panics
    /// This function errors iff:
    /// * The `data_type`'s [`crate::arrow::datatypes::PhysicalType`] is not equal to [`crate::arrow::datatypes::PhysicalType::Null`].
    pub fn new(data_type: DataType, length: usize) -> Self {
        Self::try_new(data_type, length).unwrap()
    }

    /// Returns a new empty [`NullArray`].
    pub fn new_empty(data_type: DataType) -> Self {
        Self::new(data_type, 0)
    }

    /// Returns a new [`NullArray`].
    pub fn new_null(data_type: DataType, length: usize) -> Self {
        Self::new(data_type, length)
    }

    impl_sliced!();
    impl_into_array!();
}

impl NullArray {
    /// Returns a slice of the [`NullArray`].
    /// # Panic
    /// This function panics iff `offset + length > self.len()`.
    pub fn slice(&mut self, offset: usize, length: usize) {
        assert!(
            offset + length <= self.len(),
            "the offset of the new array cannot exceed the arrays' length"
        );
        unsafe { self.slice_unchecked(offset, length) };
    }

    /// Returns a slice of the [`NullArray`].
    /// # Safety
    /// The caller must ensure that `offset + length < self.len()`.
    pub unsafe fn slice_unchecked(&mut self, _offset: usize, length: usize) {
        self.length = length;
    }

    #[inline]
    fn len(&self) -> usize {
        self.length
    }
}

impl Array for NullArray {
    impl_common_array!();

    fn validity(&self) -> Option<&Bitmap> {
        None
    }

    fn with_validity(&self, _: Option<Bitmap>) -> Box<dyn Array> {
        panic!("cannot set validity of a null array")
    }
}

#[derive(Debug)]
/// A distinct type to disambiguate
/// clashing methods
pub struct MutableNullArray {
    inner: NullArray,
}

impl MutableNullArray {
    /// Returns a new [`MutableNullArray`].
    /// # Panics
    /// This function errors iff:
    /// * The `data_type`'s [`crate::arrow::datatypes::PhysicalType`] is not equal to [`crate::arrow::datatypes::PhysicalType::Null`].
    pub fn new(data_type: DataType, length: usize) -> Self {
        let inner = NullArray::try_new(data_type, length).unwrap();
        Self { inner }
    }
}

impl From<MutableNullArray> for NullArray {
    fn from(value: MutableNullArray) -> Self {
        value.inner
    }
}

impl MutableArray for MutableNullArray {
    fn data_type(&self) -> &DataType {
        &DataType::Null
    }

    fn len(&self) -> usize {
        self.inner.length
    }

    fn validity(&self) -> Option<&MutableBitmap> {
        None
    }

    fn as_box(&mut self) -> Box<dyn Array> {
        self.inner.clone().boxed()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_mut_any(&mut self) -> &mut dyn Any {
        self
    }

    fn push_null(&mut self) {
        self.inner.length += 1;
    }

    fn reserve(&mut self, _additional: usize) {
        // no-op
    }

    fn shrink_to_fit(&mut self) {
        // no-op
    }
}

impl std::fmt::Debug for NullArray {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "NullArray({})", self.len())
    }
}

#[cfg(feature = "arrow")]
mod arrow {
    use arrow_data::ArrayData;
    use arrow_data::ArrayDataBuilder;

    use super::*;
    impl NullArray {
        /// Convert this array into [`arrow_data::ArrayData`]
        pub fn to_data(&self) -> ArrayData {
            let builder = ArrayDataBuilder::new(arrow_schema::DataType::Null).len(self.len());

            // Safety: safe by construction
            unsafe { builder.build_unchecked() }
        }

        /// Create this array from [`ArrayData`]
        pub fn from_data(data: &ArrayData) -> Self {
            Self::new(DataType::Null, data.len())
        }
    }
}

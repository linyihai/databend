use std::fmt;
use std::io::Cursor;

use bytes::Buf;
use bytes::BytesMut;
use common_exception::ErrorCode;
use common_io::prelude::BinaryRead;
use common_io::prelude::BinaryWriteBuf;
use common_meta_sled_store::sled::IVec;
use common_meta_sled_store::SledOrderedSerde;
use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;

const DB_LOOKUP_KEY_DELIMITER: char = '🐋'; // we love whale:)

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DatabaseLookupKey {
    tenant_id: Uuid,
    delimiter: char,
    database_name: String,
}

impl DatabaseLookupKey {
    pub fn new(tenant_id: Uuid, database_name: String) -> Self {
        DatabaseLookupKey {
            tenant_id,
            delimiter: DB_LOOKUP_KEY_DELIMITER,
            database_name,
        }
    }

    pub fn get_database_name(&self) -> String {
        self.database_name.clone()
    }
}

impl SledOrderedSerde for DatabaseLookupKey {
    fn ser(&self) -> Result<IVec, ErrorCode> {
        let mut buf = BytesMut::new();

        if buf.write_binary(self.tenant_id).is_ok()
            && buf.write_scalar(&self.delimiter).is_ok()
            && buf.write_string(&self.database_name).is_ok()
        {
            return Ok(IVec::from(buf.to_vec()));
        }
        Err(ErrorCode::MetaStoreDamaged("invalid key IVec"))
    }

    fn de<V: AsRef<[u8]>>(v: V) -> Result<Self, ErrorCode>
    where Self: Sized {
        let mut buf_read = Cursor::new(v);
        let tenant_id = buf_read.read_uuid();
        if let Ok(tenant_id) = tenant_id {
            buf_read.advance(4); // skip delimiter
            let database_name_result = buf_read.read_string();
            if let Ok(database_name) = database_name_result {
                return Ok(DatabaseLookupKey {
                    tenant_id,
                    delimiter: DB_LOOKUP_KEY_DELIMITER,
                    database_name,
                });
            }
        }
        Err(ErrorCode::MetaStoreDamaged("invalid key IVec"))
    }
}

impl fmt::Display for DatabaseLookupKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "DatabaseLookupKey_{}-{}",
            self.tenant_id, self.database_name
        )
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct DatabaseLookupValue(pub u64);

impl fmt::Display for DatabaseLookupValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

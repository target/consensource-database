// Copyright 2018 Bitwise IO
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

extern crate r2d2;
extern crate r2d2_diesel;
use diesel::pg::PgConnection;
use errors::DatabaseError;
use r2d2::{Pool, PooledConnection};
use r2d2_diesel::ConnectionManager;

pub type DieselConnection = PooledConnection<ConnectionManager<PgConnection>>;

pub struct ConnectionPool {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl ConnectionPool {
    pub fn connect(dsn: &str) -> Result<ConnectionPool, DatabaseError> {
        let full_dsn = String::from("postgres://") + dsn;
        let manager = ConnectionManager::<PgConnection>::new(full_dsn.clone());

        let cp = ConnectionPool {
            pool: r2d2::Pool::builder().build(manager)?,
        };
        info!("Successfully connected to database.");
        Ok(cp)
    }

    pub fn get_connection(&self) -> Result<DieselConnection, DatabaseError> {
        self.pool.get().map_err(DatabaseError::ConnError)
    }
}

use std::sync::OnceLock;
use include_dir::include_dir;
use rusqlite::Transaction;
use crate::db::util::{Migration, MigrationError, MigrationRunner};

pub fn runner() -> &'static MigrationRunner<'static> {
    static M: OnceLock<MigrationRunner> = OnceLock::new();
    M.get_or_init(|| {
        let m = MigrationRunner::new(include_dir!("$CARGO_MANIFEST_DIR/src/db/cmd/schemas"));
        m.register([
            // commit: 
            Migration::to("0.1.0").up(up_v0_1_0),
            // commit:
            Migration::to("0.2.0").up(up_v0_2_0)
        ])
    })
}

pub fn up_v0_1_0(tx: &Transaction) {
    
}

pub fn up_v0_2_0(tx: &Transaction) {
    
}



use std::env::current_dir;
use std::fs::create_dir_all;

use cosmwasm_schema::{export_schema, remove_schemas, schema_for};

use cw20::{AllowanceResponse, BalanceResponse, Cw20HandleMsg, MetaResponse, MinterResponse, Cw20QueryMsg};

fn main() {
    let mut out_dir = current_dir().unwrap();
    out_dir.push("schema");
    create_dir_all(&out_dir).unwrap();
    remove_schemas(&out_dir).unwrap();

    export_schema(&schema_for!(Cw20HandleMsg), &out_dir);
    export_schema(&schema_for!(Cw20QueryMsg), &out_dir);
    export_schema(&schema_for!(AllowanceResponse), &out_dir);
    export_schema(&schema_for!(BalanceResponse), &out_dir);
    export_schema(&schema_for!(MetaResponse), &out_dir);
    export_schema(&schema_for!(MinterResponse), &out_dir);
}

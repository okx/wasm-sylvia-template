use cosmwasm_schema::write_api;
use {{crate_name}}::contract::{ContractExecMsg, ContractQueryMsg, InstantiateMsg};
use std::env::current_dir;
use std::fs::create_dir_all;
use cosmwasm_schema::{export_schema, remove_schemas, schema_for,write_api};


#[cfg(not(tarpaulin_include))]
fn main() {
    write_api! {
        instantiate: InstantiateMsg,
        execute: ContractExecMsg,
        query: ContractQueryMsg,
    }


    let mut out_dir = current_dir().unwrap();
    out_dir.push("schema");
    create_dir_all(&out_dir).unwrap();
    remove_schemas(&out_dir).unwrap();

    export_schema(&schema_for!(InstantiateMsg), &out_dir);
    export_schema(&schema_for!(ContractExecMsg), &out_dir);
    export_schema(&schema_for!(ContractQueryMsg), &out_dir);
}

use std::{env, fs};
use near_units::parse_near;
use serde_json::json;
use workspaces::{Account, AccountId, Contract, Worker};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let wasm_arg: &str = &(env::args().nth(1).unwrap());
    let wasm_filepath = fs::canonicalize(env::current_dir()?.join(wasm_arg))?;

    let worker = workspaces::sandbox().await?;

    // deploy contract
    let wasm = std::fs::read(wasm_filepath)?;
    let contract = worker.dev_deploy(&wasm).await?;

    // create accounts
    let root = worker.dev_create_account().await?;
    let alice = create_sub(&root, "alice").await;
    let bob = create_sub(&root, "bob").await;

    // Initialize the contract
    let message = contract
        .call("new")
        .args_json(json!({"_height": 25, "_amount": 10}))
        .transact()
        .await
        .unwrap();

    println!("Contract Init Logs - {:?}", message.logs());

    let (res, gas) = deposit(&alice, contract.id(), "123123123").await;
    println!("Alice Deposit: {} - {}", res, gas);

    Ok(())
}

// --------------------------------------------------------------
// HELPER METHODS

async fn create_sub(account: &Account, name: &str) -> Account {
    account
        .create_subaccount(name)
        .initial_balance(parse_near!("30 N"))
        .transact()
        .await
        .unwrap()
        .result
}

async fn deposit(account: &Account, contract_id: &AccountId, commitment: &str) -> (bool, u64) {
    let output = account
        .call(contract_id, "deposit")
        .args_json(json!({ "commitment": commitment }))
        .max_gas()
        .transact()
        .await
        .unwrap();

    (output.is_success(), output.total_gas_burnt)
}
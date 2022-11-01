use std::{env, fs};
use near_units::parse_near;
use serde_json::{json, Value};
use workspaces::{Account, AccountId, Contract, Worker};
// use workspaces::prelude::*;


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

    let relayer = create_sub(&root, "relayer").await;

    // Tazmania parameters
    const height:u32 = 25;
    const denomination:u128 = parse_near!("10 N");

    // Initialize the contract
    let message = contract
        .call("new")
        .args_json(json!({"_height": height, "_amount": denomination}))
        .transact()
        .await
        .unwrap();

    println!("Contract Init Logs - {:?}", message.logs());

    println!(" --- Test a deposit and withdraw from different account ---");

    // information for scenario
    let sc1 = scenario1();

    // Print everyones balances
    println!("Relayer balance - {}",  relayer.view_account().await?.balance);
    println!("Alice balance - {}", alice.view_account().await?.balance);
    println!("Bob balance - {}", bob.view_account().await?.balance);

    // Alice deposits 10 N
    let (res, gas) = deposit(&alice, contract.id(), parse_near!("10 N"), &sc1.commitment).await;
    println!("Alice deposits 10 N: success:{} - gas:{}", res, gas);

    let args = json!({       
        "public": &sc1.public,
        "proof": &sc1.proof,

        "nullifier_hash": "0xf34f",
        "root": "0x2c377e89053e21e148edf782ee0db4d6dd3d7d3473d063fb385556640697529d",
        "fee": parse_near!("2 N"),

        "receipt_address": bob.id(),
        "relayer_address": relayer.id(),
    });

    // Relayer performs withdraw and sends to Bob
    let (res, gas) = withdraw(&relayer, contract.id(), args).await;
    println!("Relayer initiated withdraw with 2 N fee: success:{} - gas:{}", res, gas);

    // Print everyones balances
    println!("Relayer balance - {}",  relayer.view_account().await?.balance);
    println!("Alice balance - {}", alice.view_account().await?.balance);
    println!("Bob balance - {}", bob.view_account().await?.balance);


    println!("Check n_leaves");
    let (res, gas) = n_leaves(&relayer, contract.id()).await;
    println!("Relayer initiated n_leaves: success:{} - gas:{}", res, gas);

    println!("Check get_leaves");
    let (res, gas) = get_leaves(&relayer, contract.id()).await;
    println!("Relayer initiated n_leaves: success:{} - gas:{}", res, gas);

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

async fn get_leaves(account: &Account,
    contract_id: &AccountId) -> (bool, u64)
{
    println!("get_leaves...");

    let output = account
        .call(contract_id, "get_leaves")
        .args_json({})
        .max_gas()
        .transact()
        .await
        .unwrap();

    println!("ALL - {:?}", output);
    println!("Logs - {:?}", output.logs());

    (output.is_success(), output.total_gas_burnt)
}

async fn n_leaves(account: &Account,
    contract_id: &AccountId) -> (bool, u64)
{
    println!("n_leaves...");

    let output = account
        .call(contract_id, "n_leaves")
        .args_json({})
        .max_gas()
        .transact()
        .await
        .unwrap();

    println!("ALL - {:?}", output);
    println!("Logs - {:?}", output.logs());

    (output.is_success(), output.total_gas_burnt)
}

async fn deposit(account: &Account, 
    contract_id: &AccountId, 
    deposit_amnt: u128, 
    commitment: &str) -> (bool, u64) 
{
    println!("Depositing...");

    let output = account
        .call(contract_id, "deposit")
        .args_json(json!({ "commitment": commitment }))
        .deposit(deposit_amnt)
        .max_gas()
        .transact()
        .await
        .unwrap();

    println!("Logs - {:?}", output.logs());

    (output.is_success(), output.total_gas_burnt)
}

async fn withdraw(account: &Account, 
    contract_id: &AccountId, 
    args:Value) -> (bool, u64) 
{
    println!("Withdrawing...");

    let output = account
        .call(contract_id, "withdraw")
        .args_json(args)
        .max_gas()
        .transact()
        .await
        .unwrap();

    println!("Logs - {:?}", output.logs());

    (output.is_success(), output.total_gas_burnt)
}

// --------------------------------------------------------------
// PRECOMPUTED SCENARIOS

struct Scenario {
    commitment: String,
    public: String,
    proof: String
}

// Valid proof 1
fn scenario1() -> Scenario{
    return Scenario {
        commitment: "0x2801df893f26880e22ba3f8d12ab1f86d60ddb15fc4b2c37cf9568938d635d08".to_string(),
        proof: r#"
            {
             "pi_a": [
              "17772920403238248857670273748401790957305337216263917766408222028257472629543",
              "13544841171173056131320358966466540294231436724263645109818430838067779891255",
              "1"
             ],
             "pi_b": [
              [
               "2769250724708795008992712992417237183704401784156955041975780872295023423312",
               "9080659390345063047791542615820910243319517584136903723302203553828104024921"
              ],
              [
               "12145710151975954520323397144603605694968580889694438615712809042420777984273",
               "20832724998261050658542443997025462028705907649510553585478042999023427391277"
              ],
              [
               "1",
               "0"
              ]
             ],
             "pi_c": [
              "1337427774081430480053632807053658320655957932852225170684104186158760277594",
              "1032101386290912699594418471909210257260006265674306795568551883659147477806",
              "1"
             ],
             "protocol": "groth16",
             "curve": "bn128"
            }
            "#.to_string(),

        public: r#"
            [
             "14114388566649458089567037548032873808524665301324522922664751829136768599953",
             "11778569245147538738994022322712725094916340624517029213976247073592110043835"
            ]"#.to_string(),
    };
}





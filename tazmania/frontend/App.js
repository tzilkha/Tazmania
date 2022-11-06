import 'regenerator-runtime/runtime';
import React from 'react';

import './assets/global.css';

import { EducationalText, SignInPrompt, SignOutButton } from './ui-components';
import { create_deposit } from './proof';
import { get_commitments, send_proof} from './relayer';
import { MerkleTree } from './merkle';
import { MimcSpongeHasher } from './hashers';

import path from "path";
import * as snarkjs from 'snarkjs';
import vk from './assets/zk/verification_key.json';

import {t} from './test';


export default function App({ isSignedIn, tazmania, wallet }) {

  // Local storage for nullifier
  const [valueNullifierStorage, setValueNullifierStorage] = React.useState(() => {
    const initialValue = localStorage.getItem("nullifier");
    return initialValue || "";
  });

  // Local storage for secret
  const [valueSecretStorage, setValueSecretStorage] = React.useState(() => {
    const initialValue = localStorage.getItem("secret");
    return "";
  });

    // Local storage for isDeposit
  const [valueIsDepositStorage, setIsDepositStorage] = React.useState(() => {
    const initialValue = localStorage.getItem("isDeposit");
    return false;
  });

  // Field for nulifier which reads from local storage and deletes
  const [valueNullifier, setValueNullifier] = React.useState(() => {
    const initialValue = localStorage.getItem("nullifier");
    localStorage.removeItem("nullifier");
    return initialValue || "";
  });

  // Field for secret which reads from local storage and deletes
  const [valueSecret, setValueSecret] = React.useState(() => {
    const initialValue = localStorage.getItem("secret");
    localStorage.removeItem("secret");
    return initialValue || "";
  });

    // Field for isDeposit which reads from local storage and deletes
  const [valueIsDeposit, setIsDeposit] = React.useState(() => {
    const initialValue = localStorage.getItem("isDeposit");
    localStorage.removeItem("isDeposit");
    return initialValue || false;
  });

  // is Deposit or withdraw
  // const [valueIsDeposit, setIsDeposit] = React.useState(false);
  const [valueIsWithdraw, setIsWithdraw] = React.useState(false);

  const [valueWithdrawState, setValueWithdrawState] = React.useState("Enter your secret and nullifier.");

  const [valueRelayer, setValueRelayer] = React.useState("");

  const [valueWSecret, setValueWSecret] = React.useState("");
  const [valueWNullifier, setValueWNullifier] = React.useState("");



  /// If user not signed-in with wallet - show prompt
  if (!isSignedIn) {
    // Sign-in flow will reload the page later
    return <SignInPrompt onClick={() => wallet.signIn()}/>;
  }

  async function deposit(e) {

    localStorage.setItem("isDeposit", true);
    setIsWithdraw(false);

    var deposit = await create_deposit();
    var secret =  deposit.get_secret();
    var nullifier = deposit.get_nullifier();
    var commitment = deposit.get_commitment();
    
    var res = tazmania.deposit({"commitment": commitment}, 
      '10000000000000000000000000');

    // To persist values, we store them in local storage and then
    // clear them after we load them so no trace is saved locally
    localStorage.setItem("secret", secret);
    localStorage.setItem("nullifier", nullifier);
    setValueSecret(valueSecretStorage);
    setValueNullifier(valueNullifierStorage);

    return res
  }

  async function handle_relayer_change(e) {
    setValueRelayer(e.target.value);
  }

  async function handle_wsecret_change(e) {
    setValueWSecret(e.target.value);
  }

  async function handle_wnullifier_change(e) {
    setValueWNullifier(e.target.value);
  }

  async function handle_wcommitment_change(e) {
    setValueWCommitment(e.target.value);
  }

  async function withdraw(e) {
    setIsWithdraw(true);
    setIsDeposit(false);

  }

  async function execute_withdraw(e){
    //TODO: check valid inputs

    //TODO: instantiate commitment
    var old_deposit = await create_deposit({secret: valueWSecret, nullifier: valueWNullifier});

    //TODO: grab info from postgres
    setValueWithdrawState("Grabbing commitments from relayer..")
    console.log(valueRelayer);
    var commitments = await get_commitments(valueRelayer);
    console.log(commitments);

    //TODO: create merkle and merkleproof
    setValueWithdrawState("Creating merkle proof..")

    var ms = new MimcSpongeHasher();
    await ms.build();
    var mt = new MerkleTree(25, commitments, ms);

    // Find which index commitment is in    
    var index = commitments.indexOf(old_deposit.get_commitment());

    // Create merkle proof
    var mp = mt.get_proof(index);

    // Create input
    setValueWithdrawState("Generating input..");
    const input = {
        "root": mp.root,
        "nullifierHash": old_deposit.get_nullifier_hash(),
        "nullifier": old_deposit.get_nullifier(),
        "secret": old_deposit.get_secret(),
        "pathElements": mp.path_elements,
        "pathIndices": mp.path_indices
    }

    //TODO: zksnark to create proof
    setValueWithdrawState("Generating witness..");
    let wtns = {type: "mem"};
    await snarkjs.wtns.calculate(input, "./circuit.wasm", wtns);
    console.log(wtns);

    // zksnark proof
    setValueWithdrawState("Generating proof..");
    const proof_res = await snarkjs.groth16.prove("./circuit_final.zkey", wtns);
    var proof = proof_res.proof;
    var publicSignals = proof_res.publicSignals;
    console.log(proof);
    console.log(publicSignals);

    // verify proof
    setValueWithdrawState("Verifying proof..");
    const verify_res = await snarkjs.groth16.verify(vk, publicSignals, proof);
    console.log("Verification result:", verify_res);

    // sending proof to relayer
    setValueWithdrawState("Sending proof to relayer..");
    var rel_res = send_proof(valueRelayer, publicSignals, proof);
    console.log(rel_res);


    //TODO: send to relayer
  }


  // TODO
  async function get_root() {
    console.log( tazmania.root());
  }

  // function get_root() {
  //   e.preventDefault();
  //   setUiPleaseWait(true);
  //   helloNEAR.setGreeting(greetingInput.value)
  //     .then(async () => {return helloNEAR.get_root();})
  //     .then(setValueFromBlockchain)
  //     .finally(() => {
  //       setUiPleaseWait(false);
  //     });
  // }

  return (
    <>
      <SignOutButton accountId={wallet.accountId} onClick={() => wallet.signOut()}/>
      <main >
        <h1>
        </h1>
          <div className="buttonscontainer">
            <button onClick={deposit}>
              <span>Deposit</span>
              <div className="loader"></div>
            </button>

            <button onClick={withdraw}>
              <span>Wtihdraw</span>
              <div className="loader"></div>
            </button>
          </div>

          {valueIsDeposit && !valueIsWithdraw &&
          <div>
            <div>
              Secret:
              <textarea className="deposit_text" readOnly={true} value={valueSecret}>
              </textarea>
            </div>
            <div>
              Nullifier:
              <textarea className="deposit_text" readOnly={true} value={valueNullifier}>
              </textarea>
            </div>
            <p>
              Make sure to save the SECRET and NULLIFIER somewhere safe!
            </p>
          </div>}
        
          {valueIsWithdraw && !valueIsDeposit &&
          <div>
            <div>
              Secret:
              <textarea className="withdraw_text" onChange={handle_wsecret_change} value={valueWSecret}>
              </textarea>
            </div>
            <div>
              Nullifier:
              <textarea className="withdraw_text" onChange={handle_wnullifier_change} value={valueWNullifier}>
              </textarea>
            </div>
            <div>
              Relayer:
              <textarea className="withdraw_relayer_text" onChange={handle_relayer_change} value={valueRelayer}>
              </textarea>
            </div>

            <button onClick={execute_withdraw}>
              <span>Execute Withdraw</span>
              <div className="loader"></div>
            </button>

              <p>
                {valueWithdrawState}
              </p>
          </div>}

      </main>
    </>
  );
}

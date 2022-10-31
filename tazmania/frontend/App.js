import 'regenerator-runtime/runtime';
import React from 'react';

import './assets/global.css';

import { EducationalText, SignInPrompt, SignOutButton } from './ui-components';
import { create_deposit } from './proof';

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

  /// If user not signed-in with wallet - show prompt
  if (!isSignedIn) {
    // Sign-in flow will reload the page later
    return <SignInPrompt onClick={() => wallet.signIn()}/>;
  }

  async function deposit(e) {
    var deposit = await create_deposit();
    var secret =  deposit.get_secret();
    var nullifier = deposit.get_nullifier();
    var commitment = deposit.get_commitment();
    
    var res = tazmania.deposit({"commitment": commitment}, 
      '10000000000000000000000000');

    console.log(res);

    // To persist values, we store them in local storage and then
    // clear them after we load them so no trace is saved locally
    localStorage.setItem("secret", secret);
    localStorage.setItem("nullifier", nullifier);
    setValueSecret(valueSecretStorage);
    setValueNullifier(valueNullifierStorage);

    return res
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
          <div className="deposit">
            <button onClick={deposit}>
              <span>Deposit</span>
              <div className="loader"></div>
            </button>
          </div>
          <div>
            <div>
              Secret:
              <textarea className="sensitive" readOnly={true} value={valueSecret}>
              </textarea>
            </div>
            <div>
              Nullifier:
              <textarea className="sensitive" readOnly={true} value={valueNullifier}>
              </textarea>
            </div>
          </div>
        <EducationalText/>
      </main>
    </>
  );
}

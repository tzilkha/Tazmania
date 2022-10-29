import React from 'react';

export function SignInPrompt({onClick}) {
  return (
    <main>
      <h1>
        Welcome to Tazmania
      </h1>
      <p>
        Tazmania is a mixer protocol developed on NEAR.
      </p>
      <p>
        Do not worry, this app runs in the test network ("testnet"). It works
        just like the main network ("mainnet"), but using NEAR Tokens that are
        only for testing!
      </p>
      <br/>
      <p style={{ textAlign: 'center' }}>
        <button onClick={onClick}>Sign in with NEAR Wallet</button>
      </p>
    </main>
  );
}

export function SignOutButton({accountId, onClick}) {
  return (
    <button style={{ float: 'right' }} onClick={onClick}>
      Sign out {accountId}
    </button>
  );
}

export function EducationalText() {
  return (
    <>
      <p>
        Once you deposit make sure to save the secret and nullifier!
      </p>
    </>
  );
}

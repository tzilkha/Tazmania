/* Talking with a contract often involves transforming data, we recommend you to encapsulate that logic into a class */

export class Tazmania {
  constructor({ contractId, walletToUse }) {
    this.contractId = contractId;
    this.wallet = walletToUse;    
  }

  async deposit(args, deposit) {
    return await this.wallet.callMethod({ 
      contractId: this.contractId, 
      method: 'deposit', 
      args: args, 
      deposit: deposit,
      gas: '300000000000000'
    });
  }

  async root() {
    return await this.wallet.viewMethod({ 
      contractId: this.contractId, 
      method: 'get_root'
    });
  }
}
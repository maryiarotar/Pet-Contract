/* A helper file that simplifies using the wallet selector */

// near api js
import { providers } from 'near-api-js';

// wallet selector UI
import '@near-wallet-selector/modal-ui/styles.css';
import { setupModal } from '@near-wallet-selector/modal-ui';
import LedgerIconUrl from '@near-wallet-selector/ledger/assets/ledger-icon.png';
import MyNearIconUrl from '@near-wallet-selector/my-near-wallet/assets/my-near-wallet-icon.png';

// wallet selector options
import { setupWalletSelector } from '@near-wallet-selector/core';
import { setupLedger } from '@near-wallet-selector/ledger';
import { setupMyNearWallet } from '@near-wallet-selector/my-near-wallet';

const THIRTY_TGAS = '30000000000000';
const NO_DEPOSIT = '0';

// Wallet that simplifies using the wallet selector
export class Wallet {
  walletSelector;
  wallet;
  network;
  createAccessKeyFor;

  constructor({ createAccessKeyFor = undefined, network = 'testnet' }) {
    // Login to a wallet passing a contractId will create a local
    // key, so the user skips signing non-payable transactions.
    // Omitting the accountId will result in the user being
    // asked to sign all transactions.
    this.createAccessKeyFor = createAccessKeyFor;
    this.network = network;
    this.walletSelector = setupWalletSelector({
      network: this.network,
      modules: [setupMyNearWallet({ iconUrl: MyNearIconUrl }),
      setupLedger({ iconUrl: LedgerIconUrl })],
    });
  }


  // To be called when the website loads
  async startUp() {
  /*
  //const isSignedIn = true;
  //const accountId = isSignedIn ? walletSelector.store.getState().accounts[0].accountId : '';

    //const isSignedIn = this.walletSelector.isSignedIn();
    walletSelector.store.observable
    .pipe(
      map(state => state.accounts),
      distinctUntilChanged()
    )
    .subscribe(accounts => {
      const signedAccount = accounts.find((account) => account.active)?.accountId;
      accountChangeHook(signedAccount);
    });
*/

  };



/*
  // Sign-in method
  signIn() {
    const description = 'Please select a wallet to sign in.';
    const modal = setupModal(this.walletSelector, { contractId: this.createAccessKeyFor, description });
    modal.show();
  }*//*
  signIn = () => {
    console.log("SIGN IN------");
    const description = 'Please select a wallet to sign in.';
    const modal = setupModal(this.walletSelector, { contractId: this.createAccessKeyFor, description });
    modal.show();
  }
*/
  signIn = async () => {
    console.log("SIGN IN------" + this.createAccessKeyFor);
    const modal = setupModal(await this.walletSelector, { contractId: this.createAccessKeyFor });
    modal.show();
  };

  // Sign-out method
  signOut = async () => {
    const selectedWallet = await (await this.walletSelector).wallet();
    selectedWallet.signOut();
  };


  // Make a read-only call to retrieve information from the network
  async viewMethod({ contractId, method, args = {} }) {

    const url = `https://rpc.${this.network}.near.org`;
    const provider = new providers.JsonRpcProvider({ url });

    //const { network } = this.walletSelector.options;

    //const provider = new providers.JsonRpcProvider({ url: network.nodeUrl });

    let res = await provider.query({
      request_type: 'call_function',
      account_id: contractId,
      method_name: method,
      args_base64: Buffer.from(JSON.stringify(args)).toString('base64'),
      finality: 'optimistic',
    });

    console.log("!this.walletSelector.options ------ 2!  + " + Buffer.from(res.result).toString());

    return JSON.parse(Buffer.from(res.result).toString());
  }


  
  // Call a method that changes the contract's state
  async callMethod({ contractId, meth, args = {}, gas = THIRTY_TGAS, deposit = NO_DEPOSIT }) {

    console.log("DEPOSIT: " + deposit);
    const selectedWallet = await (await this.walletSelector).wallet();
    const outcome = await selectedWallet.signAndSendTransaction({
      receiverId: contractId,
      actions: [
        {
          type: 'FunctionCall',
          params: {
            methodName: meth,
            args,
            gas,
            deposit,
          },
        },
      ],
    });
    
      // Получаем результат транзакции от сети
      return providers.getTransactionLastResult(outcome);

    /*
    // Sign a transaction with the "FunctionCall" action
    const outcome = await this.walletSelector.signAndSendTransaction({
      signerId: this.accountId,
      receiverId: contractId,
      actions: [
        {
          type: 'FunctionCall',
          params: {
            methodName: method,
            args: Buffer.from(JSON.stringify(args)).toString('base64'),
            gas,
            deposit,
          },
        },
      ],
    });

    */
    console.log("!------->call: !  + " + Buffer.from(outcome.result).toString())

    //return providers.getTransactionLastResult(outcome)
  }

  // Get transaction result from the network
  async getTransactionResult(txhash) {
    const { network } = this.walletSelector.options;
    const provider = new providers.JsonRpcProvider({ url: network.nodeUrl });

    // Retrieve transaction result from the network
    const transaction = await provider.txStatus(txhash, 'unnused');
    return providers.getTransactionLastResult(transaction);
  }
}
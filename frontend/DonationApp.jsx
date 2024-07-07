import React, { useState, useEffect } from 'react';
import { Contract } from './near-interface';
import { Wallet } from './near-wallet';
import { utils } from "near-api-js";
import { useParams } from 'react-router-dom';

//const CONTRACT_NAME = "rm_donationcontract.testnet";

const CONTRACT_NAME = "maryiarotar321.testnet";


const DonationApp = (props) => {
  const { author, name } = useParams();


  const [isSignedIn, setIsSignedIn] = useState(false);
  const [beneficiary, setBeneficiary] = useState('');
  const [recipient, setRecipient] = useState(author);
  const [projectName, setProjectName] = useState(name);
  const [donationAmount, setDonationAmount] = useState(0);
  //const [latestDonations, setLatestDonations] = useState([]);
  const [latestDonations, setLatestDonations] = useState([
    // Default donations array
    { account_id: 'default_account_1', total_amount: '100', project_name: "default1" },
    { account_id: 'default_account_2', total_amount: '200', project_name: "default2" }
  ]);

  // Moved wallet declaration here
  const wallet = new Wallet({ createAccessKeyFor: CONTRACT_NAME });
  // Abstract the logic of interacting with the contract to simplify your project
  const contract = new Contract({ contractId: process.env.CONTRACT_NAME, walletToUse: wallet });


  useEffect(() => {
    if (!wallet) { return; }
    //wallet.startUp();
   const initFunction = async () => {
     setLatestDonations((await getDonations()).reverse());
   }
   initFunction();
  }, [])




  const getDonations = async () => {
    console.log("LOG2/1:  -  " + (await wallet.walletSelector).isSignedIn());

    const number_of_donors = await wallet.viewMethod({ contractId: CONTRACT_NAME, method: "number_of_donors" });
    const min = number_of_donors > 10 ? number_of_donors - 9 : 0;

    console.log("LOG2:  number_of_donors -  " + number_of_donors);
    //let donations = await wallet.viewMethod({ contractId: CONTRACT_NAME, method: "get_donations", args: { from_index: min.toString(), limit: number_of_donors.toString() } });
    let donations = await wallet.viewMethod({ contractId: CONTRACT_NAME, method: "get_donations" });


    donations.forEach(elem => {
      elem.total_amount = utils.format.formatNearAmount(elem.total_amount);
    })

    return donations;
  }



  useEffect(() => {
    if (!wallet) return;
    const fetchData = async () => {

      //const isSignedIn = await wallet.startUp();
      //setIsSignedIn(isSignedIn);
      if (isSignedIn) {
        const contract = new Contract({ contractId: CONTRACT_NAME, walletToUse: wallet });
        console.log("***** " + contract.contractId);
        const currentGreeting = await contract.getBeneficiary();
        
        //const currentGreeting = await wallet.viewMethod({ contractId: CONTRACT_NAME, method: "get_beneficiary" });
        setBeneficiary(currentGreeting);
        console.log("currentGreeting: " + currentGreeting);

      /*
      let don_res = await wallet.callMethod({ contractId: CONTRACT_NAME, method: "donate_to",
           args: { recipient: args['recipient'], project_name: args['project_name'] } });
           */
      }
    };
    fetchData();

  }, [wallet]);

/*
  useEffect(() => {

   if (!wallet) {
    setLatestDonations([]);
    return;
   }
   const initFunction = async() => {
      setLatestDonations(await contract.latestDonations());
    }
    //const donations = await contract.latestDonations();
    //setLatestDonations(donations);
    initFunction();

    // Clean up
    //return () => {
      // Perform any cleanup
    //};
  }, [wallet]); // Dependency added to ensure useEffect runs when wallet changes
*/

  const handleSignIn = async () => {
    try {
      await wallet.signIn(); // Вызываем функцию входа из вашего модуля аутентификации
      if (await (await wallet.walletSelector).isSignedIn()){
        setIsSignedIn(true); 
      }
    } catch (error) {
      console.error('Sign in failed:', error);
      // Можно обработать ошибку входа, например, показать сообщение пользователю
    }
  };

  const handleSignOut = () => {
    wallet.signOut();
    setIsSignedIn(false);
    console.log("signed out");
  };

  const handleFormSubmit = (event) => {
    event.preventDefault(); // Предотвращаем стандартное поведение отправки формы (обновление страницы)
    handleDonation(); // Вызываем функцию handleDonation
  };

    const handleDonation = async () => {

      console.log("Donation amount updated 2:", donationAmount);
      console.log("Donation amount updated 2:", recipient);
      console.log("Donation amount updated 2:", projectName);
      /*
      const args = {
        recipient: 'maryiarotar321.testnet',
        project_name: 'Project ABC',
      };
      */
      const args = {
        recipient: recipient,
        project_name: projectName,
      };

      try {
        let deposit = utils.format.parseNearAmount(donationAmount.toString());
        const result = await wallet.callMethod({
          contractId: CONTRACT_NAME,
          meth: 'donate_to',
          args: args,
          deposit: deposit,
        });
        console.log(result);
      } catch (e) {
        alert(
          "Something went wrong! " +
          "Maybe you need to sign out and back in? " +
          "Check your browser console for more info.",
        );
        throw e;
      }

      //const argsBase64 = Buffer.from(JSON.stringify(args)).toString('base64');


      //let don_res = await wallet.callMethod({ contractId: CONTRACT_NAME, method: "donate_to",
      //     args: { recipient: args['recipient'], project_name: args['project_name'] } });

      console.log("LOG5-----> " + 5);
    };

const handleDonationButtonClick = async (amount) => {

    let data = await fetch(
      "https://api.coingecko.com/api/v3/simple/price?ids=near&vs_currencies=usd"
    ).then((response) => response.json());
    const near2usd = data["near"]["usd"];
    const amount_in_near = amount / near2usd;
    const rounded_two_decimals = Math.round(amount_in_near * 100) / 100;
    setDonationAmount(rounded_two_decimals);
    console.log("handle donation  ---> aamount: " + donationAmount );

};


    return (
      <div className="container">
        <div className="row">
          <div className="col-sm-8 pe-2 pe-sm-5">
            <h4> Latest Donations </h4>
            <table className="table table-striped">
              <thead>
                <tr>
                  <th scope="col">User</th>
                  <th scope="col">Total Donated Ⓝ</th>
                  <th scope="col">Projects Ⓝ</th>
                </tr>
              </thead>
              <tbody>
                {latestDonations.map((donation, index) => (
                  <tr key={index}>
                    <td>{donation.account_id}</td>
                    <td>{donation.total_amount}</td>
                    <td>{donation.project_name}</td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
    
          <div className="col-sm-4">
            <div className="donation-box mt-md-4">
              <div className="donation-box-head">
                <h4> ---Make Donation---</h4>
              </div>
    
              <main className={isSignedIn ? 'donation-body signed-in-flow' : 'donation-body signed-out-flow'}>
                {isSignedIn ? (
                  <>
                    <div className="row">
                      <div className="col-3 px-1"><button className="btn btn-outline-primary" onClick={() => handleDonationButtonClick(10)}> $ 10 </button></div>
                      <div className="col-3 px-1"><button className="btn btn-outline-primary" onClick={() => handleDonationButtonClick(20)}> $ 20 </button></div>
                      <div className="col-3 px-1"><button className="btn btn-outline-primary" onClick={() => handleDonationButtonClick(50)}> $ 50 </button></div>
                      <div className="col-3 px-1"><button className="btn btn-outline-primary" onClick={() => handleDonationButtonClick(100)}> $ 100 </button></div>
                    </div>
    
                    <form onSubmit={handleFormSubmit}>
                      <fieldset>
                        <label htmlFor="recipient" className="form-label">Recipient</label>
                        <input
                          id="recipient"
                          className="form-control"
                          value={recipient}
                          onChange={(e) => setRecipient(e.target.value)}
                        />
                        <label htmlFor="projectName" className="form-label">Project Name</label>
                        <input
                          id="projectName"
                          className="form-control"
                          value={projectName}
                          onChange={(e) => setProjectName(e.target.value)}
                        />
                        <label htmlFor="donation" className="form-label">Donation amount (in Ⓝ)</label>
                        <div className="input-group">
                          <input
                            id="donation"
                            className="form-control"
                            value={donationAmount}
                            onChange={(e) => setDonationAmount(e.target.value)}
                          />
                          <span className="input-group-text">Ⓝ</span>
                          <button type="submit" className="btn btn-primary">Donate</button>
                        </div>
                      </fieldset>
                    </form>
                    <button className="link signed-in-flow" onClick={handleSignOut}>
                      Sign out
                    </button>
                  </>
                ) : (
                  <p className={'sign-in-button-style'} onClick={handleSignIn} style={{ cursor: 'pointer' }}>Please sign in with your NEAR wallet to make a donation.</p>
                )}
              </main>
            </div>
          </div>
        </div>
    
        <aside className="bg-success p-2 text-white bg-opacity-75" style={{ display: 'none' }}>
          Thank you! You have donated so far: <label htmlFor="donation-number"></label>Ⓝ
          <footer>
            <div>✔ Succeeded</div>
          </footer>
        </aside>
      </div>
    );
}

export default DonationApp;

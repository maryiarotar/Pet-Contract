use donation::DonationInfo;
// Find all our documentation at https://docs.near.org
use near_sdk::collections::UnorderedMap;
use near_sdk::{near, AccountId, NearToken};

mod donation;

// Define the contract structure
#[near(contract_state)]
pub struct Contract {
    pub beneficiary: AccountId,
    pub donations: UnorderedMap<AccountId, DonationInfo>,
}

// Define the default, which automatically initializes the contract
impl Default for Contract {
    fn default() -> Self {
        Self {
            beneficiary: "marotar.testnet".parse().unwrap(),
            donations: UnorderedMap::new(b"d"),
        }
    }
}

// Implement the contract structure
#[near]
impl Contract {
    // Public Method - but only callable by env::current_account_id()
    // initializes the contract with a beneficiary
    #[init]
    #[private]
    pub fn init(beneficiary: AccountId) -> Self {
        Self {
            beneficiary,
            donations: UnorderedMap::new(b"d"),
        }
    }

    // Public Method - get the current beneficiary
    pub fn get_beneficiary(&self) -> &AccountId {
        &self.beneficiary
    }

    // Public Method - but only callable by env::current_account_id()
    // sets the beneficiary
    #[private]
    pub fn change_beneficiary(&mut self, new_beneficiary: AccountId) {
        self.beneficiary = new_beneficiary;
    }
}

/*
 * The rest of this file holds the inline tests for the code above
 * Learn more about Rust tests: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
 */
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::VMContextBuilder;
    use near_sdk::testing_env;
    use near_sdk::NearToken;

    const BENEFICIARY: &str = "beneficiary";
    const ONE_NEAR: NearToken = NearToken::from_near(1);

    #[test]
    fn initializes() {
        let contract = Contract::init(BENEFICIARY.parse().unwrap());
        assert_eq!(
            contract.beneficiary,
            BENEFICIARY.parse::<AccountId>().unwrap().to_string()
        );
    }

    #[test]
    fn donate_to() {
        let mut contract = Contract::init(BENEFICIARY.parse().unwrap());

        set_context("donor_a", ONE_NEAR);
        contract.donate_to("recipient_1".parse().unwrap(), "Project A".to_string());
        let first_donation = contract.get_donation_for_account("donor_a".parse().unwrap());

        assert_eq!(first_donation.total_amount, ONE_NEAR.saturating_sub(NearToken::from_millinear(1)));
        assert_eq!(first_donation.project_name, Some("Project A".to_string()));

        set_context("donor_b", ONE_NEAR.saturating_mul(2));
        contract.donate_to("recipient_2".parse().unwrap(), "Project B".to_string());
        let second_donation = contract.get_donation_for_account("donor_b".parse().unwrap());

        assert_eq!(second_donation.total_amount, ONE_NEAR.saturating_mul(2).saturating_sub(NearToken::from_millinear(1)));
        assert_eq!(second_donation.project_name, Some("Project B".to_string()));

        // Получаем пожертвования с указанием from_index и limit
        let limited_donations = contract.get_donations(Some(1), Some(2));

        // Проверяем, что количество полученных пожертвований равно 2
        assert_eq!(limited_donations.len(), 2);

        assert_eq!(limited_donations[0].account_id, "donor_a".parse().unwrap());
        assert_eq!(limited_donations[0].total_amount, NearToken::from_near(2));
        assert_eq!(limited_donations[0].project_name, Some("Project A".to_string()));

        // Проверяем содержимое ограниченного пожертвования
        assert_eq!(limited_donations[1].account_id, "donor_b".parse().unwrap());
        assert_eq!(limited_donations[1].total_amount, NearToken::from_near(2));
        assert_eq!(limited_donations[1].project_name, Some("Project B".to_string()));


        // User A makes another donation on top of their original for project A
        set_context("donor_a", ONE_NEAR);
        contract.donate_to("recipient_1".parse().unwrap(), "Project A".to_string());
        let first_donation = contract.get_donation_for_account("donor_a".parse().unwrap());

        // Check the donation was recorded correctly
        assert_eq!(first_donation.total_amount, ONE_NEAR.saturating_mul(2).saturating_sub(NearToken::from_millinear(1)));
        assert_eq!(first_donation.project_name, Some("Project A, Project A".to_string()));

        assert_eq!(contract.number_of_donors(), 2);


    }

    #[test]
    fn test_number_of_donors() {
        let mut contract = Contract::init(BENEFICIARY.parse().unwrap());

        contract.donations.insert(&"donor_a".parse().unwrap(), &DonationInfo {
            total_amount: NearToken::from_near(100),
            project_name: Some("Project A".to_string()),
        });
        contract.donations.insert(&"donor_b".parse().unwrap(), &DonationInfo {
            total_amount: NearToken::from_near(200),
            project_name: Some("Project B".to_string()),
        });

        let num_donors = contract.number_of_donors();

        assert_eq!(num_donors, 2);
    }

    #[test]
    fn test_get_donations() {
        let mut contract = Contract::init("beneficiary".parse().unwrap());

        // Создаем несколько пожертвований
        set_context("donor_a", NearToken::from_near(1));
        contract.donate_to("beneficiary".parse().unwrap(), "Project A".to_string());
        
        set_context("donor_b", NearToken::from_near(2));
        contract.donate_to("beneficiary".parse().unwrap(), "Project B".to_string());

        set_context("donor_c", NearToken::from_near(3));
        contract.donate_to("beneficiary".parse().unwrap(), "Project C".to_string());

        // Получаем все пожертвования без указания from_index и limit
        let donations = contract.get_donations(None, None);

        // Проверяем, что количество полученных пожертвований равно 3
        assert_eq!(donations.len(), 3);

        // Проверяем содержимое первого пожертвования
        assert_eq!(donations[0].account_id, "donor_a".parse().unwrap());
        assert_eq!(donations[0].total_amount, NearToken::from_near(1));
        assert_eq!(donations[0].project_name, Some("Project A".to_string()));

        // Проверяем содержимое второго пожертвования
        assert_eq!(donations[1].account_id, "donor_b".parse().unwrap());
        assert_eq!(donations[1].total_amount, NearToken::from_near(2));
        assert_eq!(donations[1].project_name, Some("Project B".to_string()));

        // Проверяем содержимое третьего пожертвования
        assert_eq!(donations[2].account_id, "donor_c".parse().unwrap());
        assert_eq!(donations[2].total_amount, NearToken::from_near(3));
        assert_eq!(donations[2].project_name, Some("Project C".to_string()));

        // Получаем пожертвования с указанием from_index и limit
        let limited_donations = contract.get_donations(Some(1), Some(1));

        // Проверяем, что количество полученных пожертвований равно 1
        assert_eq!(limited_donations.len(), 1);

        // Проверяем содержимое ограниченного пожертвования
        assert_eq!(limited_donations[0].account_id, "donor_b".parse().unwrap());
        assert_eq!(limited_donations[0].total_amount, NearToken::from_near(2));
        assert_eq!(limited_donations[0].project_name, Some("Project B".to_string()));
    }


    // Auxiliar fn: create a mock context
    fn set_context(predecessor: &str, amount: NearToken) {
        let mut builder = VMContextBuilder::new();
        builder.predecessor_account_id(predecessor.parse().unwrap());
        builder.attached_deposit(amount);

        testing_env!(builder.build());
    }

}

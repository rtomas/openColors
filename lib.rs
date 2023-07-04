#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod open_colors {
    use ink::prelude::vec::Vec;
    use ink::storage::traits::StorageLayout;
    use ink::storage::Mapping;

    #[derive(scale::Encode, scale::Decode, Eq, PartialEq, Debug, Clone)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        // The caller is not the owner of the contract
        NotOwner,
    }

    #[ink(event)]
    pub struct ColorAdded {
        #[ink(topic)]
        account_id: AccountId,
        color: Color,
    }

    #[ink(event)]
    pub struct ColorsClear {
        #[ink(topic)]
        account_id: AccountId,
    }

    #[derive(scale::Encode, scale::Decode, Eq, PartialEq, Debug, Clone)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo, StorageLayout,))]
    pub struct Color {
        r: u8,
        g: u8,
        b: u8,
    }

    #[ink(storage)]
    pub struct OpenColors {
        /// Stores a single `bool` value on the storage.
        colors_list: Vec<Color>,
        last_color: Option<Color>,
        colors_added_per_user: Mapping<AccountId, u32>,
        total_colors_added: u32,
        owner: AccountId,
    }

    impl OpenColors {
        /// Constructor that initializes with the initial colors send in a vector
        #[ink(constructor)]
        pub fn new(initial_colors: Vec<Color>) -> Self {
            let mut instance = Self::default();
            let user = Self::env().caller();
            //set the owner
            instance.owner = user;

            if initial_colors.is_empty() {
                return instance;
            }

            // set the last color
            instance.last_color = initial_colors.last().cloned();

            // set the colors added per user and the total colors added
            let colors_added =
                instance.colors_added_per_user.get(user).unwrap_or(0) + initial_colors.len() as u32;
            instance.colors_added_per_user.insert(user, &colors_added);
            instance.colors_list = initial_colors.clone();
            instance.total_colors_added = initial_colors.len() as u32;

            instance
        }

        /// Constructors with no colors
        pub fn default() -> Self {
            Self {
                owner: Self::env().caller(),
                colors_list: Vec::new(),
                last_color: None,
                colors_added_per_user: Mapping::new(),
                total_colors_added: 0,
            }
        }

        #[ink(message)]
        pub fn clear_colors(&mut self) -> Result<(), Error> {
            let account = self.env().caller();

            // only owners can clean the colors
            self.ensure_owner()?;
            self.colors_list.clear();
            self.total_colors_added = 0;

            self.env().emit_event(ColorsClear {
                account_id: account,
            });

            Ok(())
        }

        #[ink(message)]
        pub fn add_color(&mut self, color: Color) {
            let account = self.env().caller();
            self.colors_list.push(color.clone());
            let amount_of_color = self
                .colors_added_per_user
                .get(self.env().caller())
                .unwrap_or(0)
                + 1;
            self.colors_added_per_user.insert(account, &amount_of_color);
            self.last_color = Some(color.clone());
            self.total_colors_added += 1;

            self.env().emit_event(ColorAdded {
                account_id: account,
                color,
            });
        }

        #[ink(message)]
        pub fn get_last_color(&mut self) -> Option<Color> {
            self.last_color.clone()
        }

        #[ink(message)]
        pub fn get_colors_list(&mut self) -> Vec<Color> {
            self.colors_list.clone()
        }

        // Ensure_owner ensures that the caller is the owner of the contract
        fn ensure_owner(&self) -> Result<(), Error> {
            let account = self.env().caller();
            // Only owners can call this function
            if self.owner != account {
                return Err(Error::NotOwner);
            }
            Ok(())
        }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// We test if the default constructor does its job.
        #[ink::test]
        fn default_values_with_no_colors() {
            let sender = default_accounts().alice;
            set_sender(sender);
            let mut open_colors = OpenColors::default();
            assert_eq!(open_colors.get_last_color(), None);
            assert_eq!(open_colors.get_colors_list(), Vec::new());
            assert_eq!(open_colors.colors_added_per_user.get(sender), None);
            assert_eq!(open_colors.owner, sender);
        }

        #[ink::test]
        fn basic_contract() {
            let mut open_colors = create_contract();
            assert_eq!(
                open_colors.get_last_color(),
                Some(Color {
                    r: 255,
                    g: 255,
                    b: 255
                })
            );
            assert_eq!(open_colors.get_colors_list(), initial_colors());
            assert_eq!(open_colors.colors_added_per_user.get(alice()).unwrap(), 2);
            assert_eq!(open_colors.owner, alice());
        }

        #[ink::test]
        fn add_a_color() {
            let mut open_colors = create_contract();

            set_sender(bob());
            open_colors.add_color(Color { r: 255, g: 0, b: 0 });

            let final_colors = vec![
                Color { r: 0, g: 0, b: 0 },
                Color {
                    r: 255,
                    g: 255,
                    b: 255,
                },
                Color { r: 255, g: 0, b: 0 },
            ];
            assert_eq!(open_colors.get_colors_list(), final_colors);
            assert_eq!(open_colors.colors_added_per_user.get(bob()).unwrap(), 1);
            assert_eq!(open_colors.last_color, Some(Color { r: 255, g: 0, b: 0 }));
        }

        fn create_contract() -> OpenColors {
            let sender = default_accounts().alice;
            set_sender(sender);

            OpenColors::new(initial_colors())
        }

        ///
        /// Some basic test functions
        ///
        fn initial_colors() -> Vec<Color> {
            vec![
                Color { r: 0, g: 0, b: 0 },
                Color {
                    r: 255,
                    g: 255,
                    b: 255,
                },
            ]
        }

        fn alice() -> AccountId {
            default_accounts().alice
        }

        fn bob() -> AccountId {
            default_accounts().bob
        }

        fn set_sender(sender: AccountId) {
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(sender);
        }

        fn default_accounts() -> ink::env::test::DefaultAccounts<ink::env::DefaultEnvironment> {
            ink::env::test::default_accounts::<ink::env::DefaultEnvironment>()
        }
    }
}

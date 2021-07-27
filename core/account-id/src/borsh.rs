use super::AccountId;

use std::io::{Error, Write};

use borsh::{BorshDeserialize, BorshSerialize};

impl BorshSerialize for AccountId {
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<(), Error> {
        self.0.serialize(writer)
    }
}

impl BorshDeserialize for AccountId {
    fn deserialize(buf: &mut &[u8]) -> Result<Self, std::io::Error> {
        let account_id = BorshDeserialize::deserialize(buf)?;
        Self::validate(&account_id)
            .map_err(|err| std::io::Error::new(std::io::ErrorKind::InvalidData, err))?;
        Ok(Self(account_id))
    }
}

#[cfg(test)]
mod tests {
    use super::{
        super::tests::{BAD_ACCOUNT_IDS, OK_ACCOUNT_IDS},
        *,
    };

    #[test]
    fn test_is_valid_account_id() {
        for account_id in OK_ACCOUNT_IDS.iter().cloned() {
            let parsed_account_id = account_id.parse::<AccountId>().unwrap_or_else(|err| {
                panic!("Valid account id {:?} marked invalid: {}", account_id, err)
            });

            let str_serialized_account_id = account_id.try_to_vec().unwrap();

            let deserialized_account_id = AccountId::try_from_slice(&str_serialized_account_id)
                .unwrap_or_else(|err| {
                    panic!("failed to deserialize account ID {:?}: {}", account_id, err)
                });
            assert_eq!(deserialized_account_id, parsed_account_id);

            let serialized_account_id =
                deserialized_account_id.try_to_vec().unwrap_or_else(|err| {
                    panic!("failed to serialize account ID {:?}: {}", account_id, err)
                });
            assert_eq!(serialized_account_id, str_serialized_account_id);
        }

        for account_id in BAD_ACCOUNT_IDS.iter().cloned() {
            let str_serialized_account_id = account_id.try_to_vec().unwrap();

            assert!(
                AccountId::try_from_slice(&str_serialized_account_id).is_err(),
                "successfully deserialized invalid account ID {:?}",
                account_id
            );
        }
    }
}

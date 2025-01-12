use crate::testing::setup::constants::INITIAL_BALANCE;
use crate::testing::setup::constants::{MINT_FEE_FAIR_BURN, MINT_PRICE};
use cosmwasm_std::{coins, Addr, Decimal, Uint128};
use cw721_base::ContractError;
use cw_multi_test::{BankSudo, SudoMsg as CwSudoMsg};
use sg_multi_test::StargazeApp;
use sg_std::NATIVE_DENOM;

pub fn add_funds_for_incremental_fee(
    router: &mut StargazeApp,
    receiver: &Addr,
    fee_amount: u128,
    fee_count: u128,
) -> Result<(), ContractError> {
    let fee_funds = coins(fee_amount * fee_count, NATIVE_DENOM);
    router
        .sudo(CwSudoMsg::Bank({
            BankSudo::Mint {
                to_address: receiver.to_string(),
                amount: fee_funds,
            }
        }))
        .map_err(|err| println!("{:?}", err))
        .ok();
    Ok(())
}

pub fn calculated_creator_balance_after_fairburn() -> Uint128 {
    let fair_burn_percent = Decimal::percent(MINT_FEE_FAIR_BURN / 100);
    let mint_price = Uint128::from(MINT_PRICE);
    Uint128::from(INITIAL_BALANCE) - (mint_price * fair_burn_percent)
}

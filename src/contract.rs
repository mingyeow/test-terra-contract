#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint64, to_binary, from_binary, Addr};

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{PING_COUNT, OWNER};


//Conditionally compiled source code is source code that may or may not be considered a part of the source code depending on certain conditions. Source code can be conditionally compiled using the attributes cfg and cfg_attr and the built-in cfg macro. These conditions are based on the target architecture of the compiled crate, arbitrary values passed to the compiler, and a few other miscellaneous things further described below in detail.
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    // this deps allows querying of blockchain state
    // pub storage: &'a dyn Storage,
    // pub api: &'a dyn Api,
    // pub querier: QuerierWrapper<'a>,
    // Q: where does the deps come from?
    // who sets deps.storage???

    let _ = PING_COUNT.save(deps.storage, &Uint64::zero());
    let _ = OWNER.save(deps.storage , &info.sender.clone());
    let res = Response::new();
    Ok(res)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    
    match msg {
        ExecuteMsg::Increment {} => try_increment(deps, _env),
        ExecuteMsg::Reset{count} => {

            let owner:Addr = OWNER.load(deps.storage)?;
            // if info.sender != owner {
            //     return Err(ContractError::Unauthorized{});
            // }

            let _:Result<Uint64, ContractError> = PING_COUNT.update(deps.storage, |mut state: Uint64|{
                state = Uint64::from(count);
                Ok(state)
            });
            Ok(Response::default())
        }
    }   
}

pub fn try_increment(deps: DepsMut, _env: Env) -> Result<Response, ContractError> {
    let ping = PING_COUNT.load(deps.storage)?;
    let one:Uint64 = Uint64::from(1u64);
    let addition = ping.checked_add(one).unwrap();
    let _ = PING_COUNT.save(deps.storage, &addition);

    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    let ping = PING_COUNT.load(deps.storage)?;
    //encoded and decoded with binary
    to_binary(&ping)
}


#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coins, from_binary};

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies(&[]);
        
        // this one is defined in msg.rs
        let msg = InstantiateMsg {};
        let info = mock_info("creator", &coins(1000, "earth"));

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        // it worked, let's query the state
        let res:Binary = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
        // this looks weird lol
        let value: Uint64 = from_binary(&res).unwrap();

        assert_eq!(Uint64::zero(), value);
    }

    #[test]
    fn increment_and_reset(){
        let mut deps = mock_dependencies(&[]);
        
        // this one is defined in msg.rs
        let msg = InstantiateMsg {};
        let info = mock_info("creator", &coins(1000, "moon"));

        // we can just call .unwrap() to assert this was a success
        instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        let execute_msg = ExecuteMsg::Increment{};
        let execute_info = mock_info("creator", &coins(1000, "moon"));
        execute(deps.as_mut(), mock_env(), execute_info, execute_msg).unwrap();

        let res:Binary = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
        let value: Uint64 = from_binary(&res).unwrap();

        assert_eq!(Uint64::from(1u64), value);

        let execute_msg_2 = ExecuteMsg::Reset{count: 5u64};
        let execute_info_2 = mock_info("creator", &coins(1000, "moon"));
        execute(deps.as_mut(), mock_env(), execute_info_2, execute_msg_2).unwrap();

        let res2:Binary = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
        let value2: Uint64 = from_binary(&res2).unwrap();

        assert_eq!(Uint64::from(5u64), value2);
    }

    #[test]
    fn reset_should_fail(){
        let mut deps = mock_dependencies(&[]);
        
        // this one is defined in msg.rs
        let msg = InstantiateMsg {};
        let info = mock_info("creator", &coins(1000, "moon"));

        // we can just call .unwrap() to assert this was a success
        instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        let execute_msg_2 = ExecuteMsg::Reset{count: 5u64};
        let execute_info_2 = mock_info("not_creator", &coins(1000, "moon"));

        let result = execute(deps.as_mut(), mock_env(), execute_info_2, execute_msg_2);

        println!("{:?}", result);

    }
}
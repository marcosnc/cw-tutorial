use cosmwasm_std::{Addr, Timestamp};
use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {
    pub admins: Vec<String>,
    pub donation_denom: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    AddMembers { admins: Vec<String> },
    Leave {},
    Donate {},
}

#[cw_serde]
pub struct AdminsListResp  {
    pub admins: Vec<Addr>,
}

#[cw_serde]
pub struct JoinTimeResp {
    pub joined: Timestamp,
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(AdminsListResp)]
    AdminsList {},
    #[returns(JoinTimeResp)]
    JoinTime { admin: String },
}

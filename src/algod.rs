use reqwest::header::HeaderMap;

use crate::algod::models::{
    Account, Block, NodeStatus, PendingTransactions, Supply, Transaction, TransactionFee,
    TransactionID, TransactionList, TransactionParams, Version,
};
use crate::transaction::SignedTransaction;
use crate::{Error, Round};

const AUTH_HEADER: &str = "X-Algo-API-Token";

/// Client for interacting with the Algorand protocol daemon
pub struct AlgodClient {
    url: String,
    token: String,
    headers: HeaderMap,
}

impl AlgodClient {
    pub fn new(address: &str, token: &str) -> AlgodClient {
        AlgodClient::new_with_headers(address, token, HeaderMap::new())
    }

    pub fn new_with_headers(address: &str, token: &str, headers: HeaderMap) -> AlgodClient {
        AlgodClient {
            url: address.to_string(),
            token: token.to_string(),
            headers,
        }
    }

    /// Returns Ok if healthy
    pub fn health(&self) -> Result<(), Error> {
        let _ = reqwest::Client::new()
            .get(&format!("{}/health", self.url))
            .headers(self.headers.clone())
            .send()?
            .error_for_status()?;
        Ok(())
    }

    /// Retrieves the current version
    pub fn versions(&self) -> Result<Version, Error> {
        let response = reqwest::Client::new()
            .get(&format!("{}/versions", self.url))
            .headers(self.headers.clone())
            .header(AUTH_HEADER, &self.token)
            .send()?
            .error_for_status()?
            .json()?;
        Ok(response)
    }

    /// Gets the current node status
    pub fn status(&self) -> Result<NodeStatus, Error> {
        let response = reqwest::Client::new()
            .get(&format!("{}/v2/status", self.url))
            .header(AUTH_HEADER, &self.token)
            .headers(self.headers.clone())
            .send()?
            .error_for_status()?
            .json()?;
        Ok(response)
    }

    /// Waits for a block to appear after the specified round and returns the node status at the time
    pub fn status_after_block(&self, round: Round) -> Result<NodeStatus, Error> {
        let response = reqwest::Client::new()
            .get(&format!(
                "{}/v1/status/wait-for-block-after/{}",
                self.url, round.0
            ))
            .header(AUTH_HEADER, &self.token)
            .headers(self.headers.clone())
            .send()?
            .error_for_status()?
            .json()?;
        Ok(response)
    }

    /// Get the block for the given round
    pub fn block(&self, round: Round) -> Result<Block, Error> {
        let response = reqwest::Client::new()
            .get(&format!("{}/v1/block/{}", self.url, round.0))
            .header(AUTH_HEADER, &self.token)
            .headers(self.headers.clone())
            .send()?
            .error_for_status()?
            .json()?;
        Ok(response)
    }

    /// Gets the current supply reported by the ledger
    pub fn ledger_supply(&self) -> Result<Supply, Error> {
        let response = reqwest::Client::new()
            .get(&format!("{}/v1/ledger/supply", self.url))
            .header(AUTH_HEADER, &self.token)
            .headers(self.headers.clone())
            .send()?
            .error_for_status()?
            .json()?;
        Ok(response)
    }

    pub fn account_information(&self, address: &str) -> Result<Account, Error> {
        let response = reqwest::Client::new()
            .get(&format!("{}/v1/account/{}", self.url, address))
            .header(AUTH_HEADER, &self.token)
            .headers(self.headers.clone())
            .send()?
            .error_for_status()?
            .json()?;
        Ok(response)
    }

    /// Gets a list of unconfirmed transactions currently in the transaction pool
    ///
    /// Sorted by priority in decreasing order and truncated at the specified limit, or returns all if specified limit is 0
    pub fn pending_transactions(&self, limit: u64) -> Result<PendingTransactions, Error> {
        let response = reqwest::Client::new()
            .get(&format!("{}/v1/transactions/pending", self.url))
            .header(AUTH_HEADER, &self.token)
            .headers(self.headers.clone())
            .query(&[("max", limit.to_string())])
            .send()?
            .error_for_status()?
            .json()?;
        Ok(response)
    }

    /// Get a specified pending transaction
    pub fn pending_transaction_information(
        &self,
        transaction_id: &str,
    ) -> Result<Transaction, Error> {
        let response = reqwest::Client::new()
            .get(&format!(
                "{}/v1/transactions/pending/{}",
                self.url, transaction_id
            ))
            .header(AUTH_HEADER, &self.token)
            .headers(self.headers.clone())
            .send()?
            .error_for_status()?
            .json()?;
        Ok(response)
    }

    /// Get a list of confirmed transactions, limited to filters if specified
    pub fn transactions(
        &self,
        address: &str,
        first_round: Option<Round>,
        last_round: Option<Round>,
        from_date: Option<String>,
        to_date: Option<String>,
        limit: Option<u64>,
    ) -> Result<TransactionList, Error> {
        let mut query = Vec::new();
        if let Some(first_round) = first_round {
            query.push(("firstRound", first_round.0.to_string()))
        }
        if let Some(last_round) = last_round {
            query.push(("lastRound", last_round.0.to_string()))
        }
        if let Some(from_date) = from_date {
            query.push(("fromDate", from_date))
        }
        if let Some(to_date) = to_date {
            query.push(("toDate", to_date))
        }
        if let Some(limit) = limit {
            query.push(("max", limit.to_string()))
        }
        let response = reqwest::Client::new()
            .get(&format!("{}/v2/account/{}/transactions", self.url, address))
            .header(AUTH_HEADER, &self.token)
            .headers(self.headers.clone())
            .query(&query)
            .send()?
            .error_for_status()?
            .json()?;
        Ok(response)
    }

    /// Broadcasts a transaction to the network
    pub fn send_transaction(
        &self,
        signed_transaction: &SignedTransaction,
    ) -> Result<TransactionID, Error> {
        let bytes = rmp_serde::to_vec_named(signed_transaction)?;
        self.raw_transaction(&bytes)
    }

    /// Broadcasts a raw transaction to the network
    pub fn raw_transaction(&self, raw: &[u8]) -> Result<TransactionID, Error> {
        let response = reqwest::Client::new()
            .post(&format!("{}/v2/transactions", self.url))
            .header(AUTH_HEADER, &self.token)
            .headers(self.headers.clone())
            .body(raw.to_vec())
            .send()?
            .error_for_status()?
            .json()?;
        Ok(response)
    }

    /// Gets the information of a single transaction
    pub fn transaction(&self, transaction_id: &str) -> Result<Transaction, Error> {
        let response = reqwest::Client::new()
            .get(&format!("{}/v2/transaction/{}", self.url, transaction_id))
            .header(AUTH_HEADER, &self.token)
            .headers(self.headers.clone())
            .send()?
            .error_for_status()?
            .json()?;
        Ok(response)
    }

    /// Gets a specific confirmed transaction
    pub fn transaction_information(
        &self,
        address: &str,
        transaction_id: &str,
    ) -> Result<Transaction, Error> {
        let response = reqwest::Client::new()
            .get(&format!(
                "{}/v2/account/{}/transaction/{}",
                self.url, address, transaction_id
            ))
            .header(AUTH_HEADER, &self.token)
            .headers(self.headers.clone())
            .send()?
            .error_for_status()?
            .json()?;
        Ok(response)
    }

    /// Gets suggested fee in units of micro-Algos per byte
    pub fn suggested_fee(&self) -> Result<TransactionFee, Error> {
        let response = reqwest::Client::new()
            .get(&format!("{}/v2/transactions/fee", self.url))
            .header(AUTH_HEADER, &self.token)
            .headers(self.headers.clone())
            .send()?
            .error_for_status()?
            .json()?;
        Ok(response)
    }

    /// Gets parameters for constructing a new transaction
    pub fn transaction_params(&self) -> Result<TransactionParams, Error> {
        let response = reqwest::Client::new()
            .get(&format!("{}/v2/transactions/params", self.url))
            .header(AUTH_HEADER, &self.token)
            .headers(self.headers.clone())
            .send()?
            .error_for_status()?
            .json()?;
        Ok(response)
    }
}

pub mod models {
    use serde::{Deserialize, Serialize};

    use crate::util::deserialize_bytes;
    use crate::util::deserialize_hash;
    use crate::HashDigest;
    use crate::MicroAlgos;
    use crate::Round;

    /// The information about a node status
    #[derive(Debug, Serialize, Deserialize)]
    pub struct NodeStatus {
        /// the last round seen
        #[serde(rename = "last-round")]
        pub last_round: Round,

        /// The last consensus version supported
        #[serde(rename = "last-version")]
        pub last_version: String,

        /// Next version of consensus protocol to use
        #[serde(rename = "next-version")]
        pub next_version: String,

        /// The round at which the next consensus version will apply
        #[serde(rename = "next-version-round")]
        pub next_version_round: Round,

        /// Whether the next consensus version is supported by this node
        #[serde(rename = "next-version-supported")]
        pub next_version_supported: bool,

        /// Time since last round in nanoseconds
        #[serde(rename = "time-since-last-round")]
        pub time_since_last_round: i64,

        // Catchup time in nanoseconds
        #[serde(rename = "catchup-time")]
        pub catchup_time: i64,
    }

    /// TransactionID Description
    #[derive(Debug, Serialize, Deserialize)]
    pub struct TransactionID {
        /// The string encoding of the transaction hash
        #[serde(rename = "txId")]
        pub tx_id: String,
    }

    /// Account Description
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Account {
        /// The round for which this information is relevant
        pub round: Round,

        /// The account public key
        pub address: String,

        /// The total number of MicroAlgos in the account
        pub amount: MicroAlgos,

        /// the amount of MicroAlgos of pending rewards in this account.
        #[serde(rename = "pending-rewards")]
        pub pending_rewards: MicroAlgos,

        /// the amount of MicroAlgos in the account, without the pending rewards.
        #[serde(rename = "amount-without-pending-rewards")]
        pub amount_without_pending_rewards: u64,

        /// Rewards indicates the total rewards of MicroAlgos the account has recieved
        pub rewards: MicroAlgos,

        /// Status indicates the delegation status of the account's MicroAlgos
        /// Offline - indicates that the associated account is delegated.
        /// Online  - indicates that the associated account used as part of the delegation pool.
        /// NotParticipating - indicates that the associated account is neither a delegator nor a delegate.
        pub status: String,
    }

    /// Transaction contains all fields common to all transactions and serves as an envelope to all transactions type
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Transaction {
        /// The transaction type
        #[serde(rename = "type")]
        pub txn_type: String,

        /// The transaction ID
        #[serde(rename = "tx")]
        pub transaction_id: String,

        /// The sender's address
        pub from: String,

        /// Fee is the transaction fee
        pub fee: MicroAlgos,

        /// The first valid round for this transaction
        #[serde(rename = "first-round")]
        pub first_round: Round,

        /// The last valid round for this transaction
        #[serde(rename = "last-round")]
        pub last_round: Round,

        /// Note is a free form data
        #[serde(
        rename = "noteb64",
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "deserialize_bytes"
        )]
        pub note: Vec<u8>,

        /// The block number this transaction appeared in
        #[serde(skip_serializing_if = "Option::is_none")]
        pub round: Option<u64>,

        /// Indicates the transaction was evicted from this node's transaction
        /// pool (if non-empty).  A non-empty pool_error does not guarantee that the
        /// transaction will never be committed; other nodes may not have evicted the
        /// transaction and may attempt to commit it in the future.
        #[serde(
        rename = "poolerror",
        default,
        skip_serializing_if = "String::is_empty"
        )]
        pub pool_error: String,

        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub payment: Option<PaymentTransactionType>,

        /// the amount of pending rewards applied to the from
        /// account as part of this transaction.
        #[serde(rename = "fromrewards", skip_serializing_if = "Option::is_none")]
        pub from_rewards: Option<u64>,

        #[serde(rename = "genesis-id")]
        pub genesis_id: String,

        #[serde(rename = "genesis-hash", deserialize_with = "deserialize_hash")]
        pub genesis_hash: HashDigest,
    }

    /// PaymentTransactionType contains the additional fields for a payment Transaction
    #[derive(Debug, Serialize, Deserialize)]
    pub struct PaymentTransactionType {
        /// To is the receiver's address
        pub to: String,

        /// The address the sender closed to
        #[serde(rename = "close", skip_serializing_if = "Option::is_none")]
        pub close_remainder_to: Option<String>,

        /// The amount sent to close_remainder_to, for committed transaction
        #[serde(rename = "closeamount", skip_serializing_if = "Option::is_none")]
        pub close_amount: Option<MicroAlgos>,

        /// The amount of MicroAlgos intended to be transferred
        pub amount: MicroAlgos,

        /// The amount of pending rewards applied to the To account
        /// as part of this transaction.
        #[serde(rename = "torewards", skip_serializing_if = "Option::is_none")]
        pub to_rewards: Option<u64>,

        /// The amount of pending rewards applied to the CloseRemainderTo
        /// account as part of this transaction.
        #[serde(rename = "closerewards", skip_serializing_if = "Option::is_none")]
        pub close_rewards: Option<u64>,
    }

    /// TransactionList contains a list of transactions
    #[derive(Debug, Serialize, Deserialize)]
    pub struct TransactionList {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub transactions: Vec<Transaction>,
    }

    /// TransactionFee contains the suggested fee
    #[derive(Debug, Serialize, Deserialize)]
    pub struct TransactionFee {
        /// Transaction fee in units of micro-Algos per byte.
        /// Fee may fall to zero but transactions must still have a fee of
        /// at least MinTxnFee for the current network protocol.
        pub fee: MicroAlgos,
    }

    /// TransactionParams contains the parameters that help a client construct a new transaction.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct TransactionParams {
        /// Transaction fee in units of micro-Algos per byte.
        /// Fee may fall to zero but transactions must still have a fee of
        /// at least MinTxnFee for the current network protocol.
        pub fee: MicroAlgos,

        /// Genesis ID
        #[serde(rename = "genesis-id")]
        pub genesis_id: String,

        /// Genesis hash
        #[serde(rename = "genesis-hash", deserialize_with = "deserialize_hash")]
        pub genesis_hash: HashDigest,

        // The last round seen
        #[serde(rename = "last-round")]
        pub last_round: Round,

        // The consensus protocol version as of last_round.
        #[serde(rename = "consensus-version")]
        pub consensus_version: String,
    }

    /// Block contains a block information
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Block {
        /// The current block hash
        pub hash: String,

        /// The previous block hash
        #[serde(rename = "previousBlockHash")]
        pub previous_block_hash: String,

        /// The sortition seed
        pub seed: String,

        /// The address of this block proposer
        pub proposer: String,

        /// The current round on which this block was appended to the chain
        pub round: Round,

        /// Period is the period on which the block was confirmed
        pub period: u64,

        /// TransactionsRoot authenticates the set of transactions appearing in the block.
        /// More specifically, it's the root of a merkle tree whose leaves are the block's Txids, in lexicographic order.
        /// For the empty block, it's 0.
        /// Note that the TxnRoot does not authenticate the signatures on the transactions, only the transactions themselves.
        /// Two blocks with the same transactions but in a different order and with different signatures will have the same TxnRoot.
        #[serde(rename = "txnRoot")]
        pub transactions_root: String,

        /// Specifies how many rewards, in MicroAlgos,
        /// have been distributed to each config.Protocol.RewardUnit
        /// of MicroAlgos since genesis.
        #[serde(rename = "reward", default, skip_serializing_if = "Option::is_none")]
        pub rewards_level: Option<MicroAlgos>,

        /// The number of new MicroAlgos added to the participation stake from rewards at the next round.
        #[serde(rename = "rate", default, skip_serializing_if = "Option::is_none")]
        pub rewards_rate: Option<MicroAlgos>,

        /// The number of leftover MicroAlgos after the distribution of RewardsRate/rewardUnits
        /// MicroAlgos for every reward unit in the next round.
        #[serde(rename = "frac", default, skip_serializing_if = "Option::is_none")]
        pub rewards_residue: Option<MicroAlgos>,

        /// The list of transactions in this block
        #[serde(rename = "txns", default, skip_serializing_if = "Option::is_none")]
        pub transactions: Option<TransactionList>,

        /// TimeStamp in seconds since epoch
        pub timestamp: i64,
        #[serde(flatten)]
        pub upgrade_state: UpgradeState,
        #[serde(flatten)]
        pub upgrade_vote: UpgradeVote,
    }

    /// UpgradeState contains the information about a current state of an upgrade
    #[derive(Debug, Serialize, Deserialize)]
    pub struct UpgradeState {
        /// A string that represents the current protocol
        #[serde(rename = "currentProtocol")]
        current_protocol: String,

        /// A string that represents the next proposed protocol
        #[serde(rename = "nextProtocol")]
        next_protocol: String,

        /// The number of blocks which approved the protocol upgrade
        #[serde(rename = "nextProtocolApprovals")]
        next_protocol_approvals: u64,

        /// The deadline round for this protocol upgrade (No votes will be consider after this round)
        #[serde(rename = "nextProtocolVoteBefore")]
        next_protocol_vote_before: Round,

        /// The round on which the protocol upgrade will take effect
        #[serde(rename = "nextProtocolSwitchOn")]
        next_protocol_switch_on: Round,
    }

    /// UpgradeVote represents the vote of the block proposer with respect to protocol upgrades.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct UpgradeVote {
        /// Indicates a proposed upgrade
        #[serde(rename = "upgradePropose")]
        upgrade_propose: String,

        /// Indicates a yes vote for the current proposal
        #[serde(rename = "upgradeApprove")]
        upgrade_approve: bool,
    }

    /// Supply represents the current supply of MicroAlgos in the system
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Supply {
        round: Round,
        #[serde(rename = "totalMoney")]
        total_money: MicroAlgos,
        #[serde(rename = "onlineMoney")]
        online_money: MicroAlgos,
    }

    /// PendingTransactions represents a potentially truncated list of transactions currently in the
    /// node's transaction pool.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct PendingTransactions {
        #[serde(rename = "truncatedTxns")]
        truncated_txns: TransactionList,
        #[serde(rename = "totalTxns")]
        total_txns: u64,
    }

    /// Version contains the current algod version.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Version {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub versions: Vec<String>,
        pub genesis_id: String,
        #[serde(rename = "genesis_hash_b64", deserialize_with = "deserialize_hash")]
        pub genesis_hash: HashDigest,
    }
}

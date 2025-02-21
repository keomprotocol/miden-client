use core::fmt;
use crypto::{
    dsa::rpo_falcon512::FalconError,
    merkle::MmrError,
    utils::{DeserializationError, HexParseError},
};
use miden_node_proto::errors::ParseError;
use miden_tx::{DataStoreError, TransactionExecutorError, TransactionProverError};
use objects::{
    accounts::AccountId, notes::NoteId, AccountError, AssetVaultError, Digest, NoteError,
    TransactionScriptError,
};
use tonic::{transport::Error as TransportError, Status as TonicStatus};

// CLIENT ERROR
// ================================================================================================

#[derive(Debug)]
pub enum ClientError {
    AccountError(AccountError),
    AuthError(FalconError),
    ImportNewAccountWithoutSeed,
    NoteError(NoteError),
    NoConsumableNoteForAccount(AccountId),
    RpcApiError(RpcApiError),
    StoreError(StoreError),
    TransactionExecutionError(TransactionExecutorError),
    TransactionProvingError(TransactionProverError),
}

impl fmt::Display for ClientError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ClientError::AccountError(err) => write!(f, "account error: {err}"),
            ClientError::AuthError(err) => write!(f, "account auth error: {err}"),
            ClientError::ImportNewAccountWithoutSeed => write!(
                f,
                "import account error: can't import a new account without its initial seed"
            ),
            ClientError::NoConsumableNoteForAccount(account_id) => {
                write!(f, "No consumable note for account ID {}", account_id)
            }
            ClientError::NoteError(err) => write!(f, "note error: {err}"),
            ClientError::RpcApiError(err) => write!(f, "rpc api error: {err}"),
            ClientError::StoreError(err) => write!(f, "store error: {err}"),
            ClientError::TransactionExecutionError(err) => {
                write!(f, "transaction executor error: {err}")
            }
            ClientError::TransactionProvingError(err) => {
                write!(f, "transaction prover error: {err}")
            }
        }
    }
}

// CONVERSIONS
// ================================================================================================

impl From<AccountError> for ClientError {
    fn from(err: AccountError) -> Self {
        Self::AccountError(err)
    }
}

impl From<FalconError> for ClientError {
    fn from(err: FalconError) -> Self {
        Self::AuthError(err)
    }
}

impl From<NoteError> for ClientError {
    fn from(err: NoteError) -> Self {
        Self::NoteError(err)
    }
}

impl From<RpcApiError> for ClientError {
    fn from(err: RpcApiError) -> Self {
        Self::RpcApiError(err)
    }
}

impl From<StoreError> for ClientError {
    fn from(err: StoreError) -> Self {
        Self::StoreError(err)
    }
}

impl From<TransactionExecutorError> for ClientError {
    fn from(err: TransactionExecutorError) -> Self {
        Self::TransactionExecutionError(err)
    }
}

impl From<TransactionProverError> for ClientError {
    fn from(err: TransactionProverError) -> Self {
        Self::TransactionProvingError(err)
    }
}

impl From<rusqlite::Error> for ClientError {
    fn from(err: rusqlite::Error) -> Self {
        Self::StoreError(StoreError::from(err))
    }
}

impl From<ClientError> for String {
    fn from(err: ClientError) -> String {
        err.to_string()
    }
}

#[cfg(feature = "std")]
impl std::error::Error for ClientError {}

// STORE ERROR
// ================================================================================================

#[derive(Debug)]
pub enum StoreError {
    AssetVaultError(AssetVaultError),
    AccountCodeDataNotFound(Digest),
    AccountDataNotFound(AccountId),
    AccountError(AccountError),
    AccountHashMismatch(AccountId),
    AccountStorageNotFound(Digest),
    BlockHeaderNotFound(u32),
    ChainMmrNodeNotFound(u64),
    DatabaseError(String),
    DataDeserializationError(DeserializationError),
    HexParseError(HexParseError),
    InputNoteNotFound(NoteId),
    InputSerializationError(serde_json::Error),
    JsonDataDeserializationError(serde_json::Error),
    MmrError(MmrError),
    NoteTagAlreadyTracked(u64),
    ParsingError(String),
    QueryError(String),
    RpcTypeConversionFailure(ParseError),
    TransactionScriptError(TransactionScriptError),
    VaultDataNotFound(Digest),
}

impl From<AssetVaultError> for StoreError {
    fn from(value: AssetVaultError) -> Self {
        StoreError::AssetVaultError(value)
    }
}

impl From<AccountError> for StoreError {
    fn from(value: AccountError) -> Self {
        StoreError::AccountError(value)
    }
}

impl From<rusqlite_migration::Error> for StoreError {
    fn from(value: rusqlite_migration::Error) -> Self {
        StoreError::DatabaseError(value.to_string())
    }
}
impl From<rusqlite::Error> for StoreError {
    fn from(value: rusqlite::Error) -> Self {
        match value {
            rusqlite::Error::FromSqlConversionFailure(_, _, _)
            | rusqlite::Error::IntegralValueOutOfRange(_, _)
            | rusqlite::Error::InvalidColumnIndex(_)
            | rusqlite::Error::InvalidColumnType(_, _, _) => {
                StoreError::ParsingError(value.to_string())
            }
            rusqlite::Error::InvalidParameterName(_)
            | rusqlite::Error::InvalidColumnName(_)
            | rusqlite::Error::StatementChangedRows(_)
            | rusqlite::Error::ExecuteReturnedResults
            | rusqlite::Error::InvalidQuery
            | rusqlite::Error::MultipleStatement
            | rusqlite::Error::InvalidParameterCount(_, _)
            | rusqlite::Error::QueryReturnedNoRows => StoreError::QueryError(value.to_string()),
            _ => StoreError::DatabaseError(value.to_string()),
        }
    }
}

impl From<DeserializationError> for StoreError {
    fn from(value: DeserializationError) -> Self {
        StoreError::DataDeserializationError(value)
    }
}

impl From<ParseError> for StoreError {
    fn from(value: ParseError) -> Self {
        StoreError::RpcTypeConversionFailure(value)
    }
}

impl From<HexParseError> for StoreError {
    fn from(value: HexParseError) -> Self {
        StoreError::HexParseError(value)
    }
}

impl From<MmrError> for StoreError {
    fn from(value: MmrError) -> Self {
        StoreError::MmrError(value)
    }
}

impl From<TransactionScriptError> for StoreError {
    fn from(value: TransactionScriptError) -> Self {
        StoreError::TransactionScriptError(value)
    }
}

impl fmt::Display for StoreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use StoreError::*;
        match self {
            AssetVaultError(err) => {
                write!(f, "asset vault with root {} not found", err)
            }
            AccountCodeDataNotFound(root) => {
                write!(f, "account code data with root {} not found", root)
            }
            AccountDataNotFound(account_id) => {
                write!(f, "Account data was not found for Account Id {account_id}")
            }
            AccountError(err) => write!(f, "error instantiating Account: {err}"),
            AccountHashMismatch(account_id) => {
                write!(f, "account hash mismatch for account {account_id}")
            }
            AccountStorageNotFound(root) => {
                write!(f, "account storage data with root {} not found", root)
            }
            BlockHeaderNotFound(block_number) => {
                write!(f, "block header for block {} not found", block_number)
            }
            ChainMmrNodeNotFound(node_index) => {
                write!(f, "chain mmr node at index {} not found", node_index)
            }
            DatabaseError(err) => write!(f, "database-related non-query error: {err}"),
            DataDeserializationError(err) => {
                write!(f, "error deserializing data from the store: {err}")
            }
            HexParseError(err) => {
                write!(f, "error parsing hex: {err}")
            }
            InputNoteNotFound(note_id) => {
                write!(f, "input note with note id {} not found", note_id.inner())
            }
            InputSerializationError(err) => {
                write!(f, "error trying to serialize inputs for the store: {err}")
            }
            JsonDataDeserializationError(err) => {
                write!(
                    f,
                    "error deserializing data from JSON from the store: {err}"
                )
            }
            MmrError(err) => write!(f, "error constructing mmr: {err}"),
            NoteTagAlreadyTracked(tag) => write!(f, "note tag {} is already being tracked", tag),
            ParsingError(err) => {
                write!(f, "failed to parse data retrieved from the database: {err}")
            }
            QueryError(err) => write!(f, "failed to retrieve data from the database: {err}"),
            TransactionScriptError(err) => {
                write!(f, "error instantiating transaction script: {err}")
            }
            VaultDataNotFound(root) => write!(f, "account vault data for root {} not found", root),
            RpcTypeConversionFailure(err) => write!(f, "failed to convert data: {err}"),
        }
    }
}

impl From<StoreError> for DataStoreError {
    fn from(value: StoreError) -> Self {
        match value {
            StoreError::AccountDataNotFound(account_id) => {
                DataStoreError::AccountNotFound(account_id)
            }
            StoreError::BlockHeaderNotFound(block_num) => DataStoreError::BlockNotFound(block_num),
            StoreError::InputNoteNotFound(note_id) => DataStoreError::NoteNotFound(note_id),
            err => DataStoreError::InternalError(err.to_string()),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for StoreError {}

// API CLIENT ERROR
// ================================================================================================

use crate::client::RpcApiEndpoint;

#[derive(Debug)]
pub enum RpcApiError {
    ConnectionError(TransportError),
    ConversionFailure(ParseError),
    ExpectedFieldMissing(String),
    InvalidAccountReceived(AccountError),
    RequestError(RpcApiEndpoint, TonicStatus),
}

impl fmt::Display for RpcApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RpcApiError::ConnectionError(err) => {
                write!(f, "failed to connect to the API server: {err}")
            }
            RpcApiError::ConversionFailure(err) => {
                write!(f, "failed to convert RPC data: {err}")
            }
            RpcApiError::ExpectedFieldMissing(err) => {
                write!(f, "rpc API reponse missing an expected field: {err}")
            }
            RpcApiError::InvalidAccountReceived(account_error) => {
                write!(
                    f,
                    "rpc API reponse contained an invalid account: {account_error}"
                )
            }
            RpcApiError::RequestError(endpoint, err) => {
                write!(f, "rpc request failed for {endpoint}: {err}")
            }
        }
    }
}

impl From<ParseError> for RpcApiError {
    fn from(err: ParseError) -> Self {
        Self::ConversionFailure(err)
    }
}

impl From<AccountError> for RpcApiError {
    fn from(err: AccountError) -> Self {
        Self::InvalidAccountReceived(err)
    }
}

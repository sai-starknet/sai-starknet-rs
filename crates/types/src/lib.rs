#![cfg_attr(not(test), no_std)]

#[cfg(feature = "std")]
extern crate std;

extern crate alloc;

pub use sai_felt::Felt;

mod conversions;

mod serde_impls;

/// SNIP-12 typed data.
// pub mod typed_data;
// pub use typed_data::TypedData;

// TODO: better namespacing of exports?
mod codegen;
pub use codegen::{
    BinaryNode, BlockHeader, BlockStatus, BlockTag, BlockWithReceipts, BlockWithTxHashes,
    BlockWithTxs, BroadcastedDeclareTransaction, BroadcastedDeclareTransactionV3,
    BroadcastedDeployAccountTransaction, BroadcastedDeployAccountTransactionV3,
    BroadcastedInvokeTransaction, BroadcastedInvokeTransactionV3, CallType,
    CompressedLegacyContractClass, ContractErrorData, ContractLeafData, ContractStorageDiffItem,
    ContractStorageKeys, ContractsProof, DataAvailabilityMode, DeclareTransactionReceipt,
    DeclareTransactionTrace, DeclareTransactionV0, DeclareTransactionV0Content,
    DeclareTransactionV1, DeclareTransactionV1Content, DeclareTransactionV2,
    DeclareTransactionV2Content, DeclareTransactionV3, DeclareTransactionV3Content,
    DeclaredClassItem, DeployAccountTransactionReceipt, DeployAccountTransactionTrace,
    DeployAccountTransactionV1, DeployAccountTransactionV1Content, DeployAccountTransactionV3,
    DeployAccountTransactionV3Content, DeployTransaction, DeployTransactionContent,
    DeployTransactionReceipt, DeployedContractItem, EdgeNode, EmittedEvent,
    EmittedEventWithFinality, EntryPointType, EntryPointsByType, Event, EventFilter,
    EventFilterWithPage, EventsChunk, ExecutionResources, FeeEstimate, FeePayment,
    FlattenedSierraClass, FunctionCall, FunctionInvocation, FunctionStateMutability, GlobalRoots,
    InnerCallExecutionResources, InnerContractExecutionError, InvokeTransactionReceipt,
    InvokeTransactionTrace, InvokeTransactionV0, InvokeTransactionV0Content, InvokeTransactionV1,
    InvokeTransactionV1Content, InvokeTransactionV3, InvokeTransactionV3Content,
    L1DataAvailabilityMode, L1HandlerTransaction, L1HandlerTransactionContent,
    L1HandlerTransactionReceipt, L1HandlerTransactionTrace, L2TransactionFinalityStatus,
    L2TransactionStatus, LegacyContractEntryPoint, LegacyEntryPointsByType, LegacyEventAbiEntry,
    LegacyEventAbiType, LegacyFunctionAbiEntry, LegacyFunctionAbiType, LegacyStructAbiEntry,
    LegacyStructAbiType, LegacyStructMember, LegacyTypedParameter, MessageFeeEstimate, MsgFromL1,
    MsgToL1, NewTransactionStatus, NoTraceAvailableErrorData, NonceUpdate, OrderedEvent,
    OrderedMessage, PreConfirmedBlockWithReceipts, PreConfirmedBlockWithTxHashes,
    PreConfirmedBlockWithTxs, PreConfirmedStateUpdate, PriceUnit, ReorgData, ReplacedClassItem,
    ResourceBounds, ResourceBoundsMapping, ResourcePrice, ResultPageRequest, RevertedInvocation,
    SequencerTransactionStatus, SierraEntryPoint, SimulatedTransaction, SimulationFlag,
    SimulationFlagForEstimateFee, StarknetError, StateDiff, StateUpdate, StorageEntry,
    StorageProof, SubscriptionId, SyncStatus, TransactionExecutionErrorData,
    TransactionExecutionStatus, TransactionFinalityStatus, TransactionReceiptWithBlockInfo,
    TransactionTraceWithHash, TransactionWithL2Status, TransactionWithReceipt,
};

/// Module containing the [`U256`] type.
// pub mod u256;
// pub use u256::U256;

/// Module containing the [`EthAddress`] type.
pub mod eth_address;
pub use eth_address::EthAddress;

/// Module containing the [`Hash256`] type.
pub mod hash_256;
pub use hash_256::Hash256;

mod execution_result;
pub use execution_result::ExecutionResult;

mod message_status;
pub use message_status::MessageStatus;

mod receipt_block;
pub use receipt_block::ReceiptBlock;

mod msg;
pub use msg::MsgToL2;

mod call;
pub use call::Call;

// mod byte_array;
// pub use byte_array::ByteArray;

// TODO: move generated request code to `starknet-providers`
/// Module containing JSON-RPC request types.
pub mod requests;

// /// Module containing types related to Starknet contracts/classes.
// pub mod contract;
// pub use contract::ContractArtifact;

mod types;
pub use types::{
    BlockHashAndNumber, BlockId, BroadcastedTransaction, ConfirmedBlockId, ContractClass,
    ContractExecutionError, DeclareTransaction, DeclareTransactionContent,
    DeclareTransactionResult, DeployAccountTransaction, DeployAccountTransactionContent,
    DeployAccountTransactionResult, DeployTransactionResult, EventsPage, ExecuteInvocation,
    InvokeTransaction, InvokeTransactionContent, InvokeTransactionResult, LegacyContractAbiEntry,
    MaybePreConfirmedBlockWithReceipts, MaybePreConfirmedBlockWithTxHashes,
    MaybePreConfirmedBlockWithTxs, MaybePreConfirmedStateUpdate, MerkleNode, ParseMsgToL2Error,
    SyncStatusType, Transaction, TransactionContent, TransactionOrHash, TransactionReceipt,
    TransactionStatus, TransactionTrace,
};

pub mod serde;

// pub mod codec;

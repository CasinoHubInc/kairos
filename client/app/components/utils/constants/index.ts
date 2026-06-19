// TODO: install @stellar/stellar-sdk and replace this file with Stellar/Soroban equivalents

// Stellar network passphrase constants
// import { Networks, SorobanRpc } from "@stellar/stellar-sdk";

export const STELLAR_MAINNET_PASSPHRASE = "Public Global Stellar Network ; September 2015";
export const STELLAR_TESTNET_PASSPHRASE = "Test SDF Network ; September 2015";

// TODO: replace contract addresses with deployed Soroban contract IDs (C... format)
export const KAIROS_CONTRACT_ADDRESS = "TODO: deployed Soroban contract ID on testnet";

// TODO: XLM is the native Stellar asset — use the Stellar Asset Contract (SAC) address for wrapped XLM on Soroban
export const XLMTokenAddress = "TODO: XLM SAC contract ID";

// Stellar Soroban RPC endpoints
const IS_MAINNET = process.env.NEXT_PUBLIC_NETWORK === "mainnet";

export const SOROBAN_RPC_URL = IS_MAINNET
  ? "https://mainnet.stellar.validation.cloud/api/v1/soroban/rpc"
  : "https://soroban-testnet.stellar.org";

export const HORIZON_URL = IS_MAINNET
  ? "https://horizon.stellar.org"
  : "https://horizon-testnet.stellar.org";

export const NETWORK_PASSPHRASE = IS_MAINNET
  ? STELLAR_MAINNET_PASSPHRASE
  : STELLAR_TESTNET_PASSPHRASE;

// TODO: initialise SorobanRpc.Server for contract calls:
// export const server = new SorobanRpc.Server(SOROBAN_RPC_URL, { allowHttp: false });

import {
  canonicalAddress,
  isNativeDenom,
  isNativeTerra,
  TerraChainId,
} from "@certusone/wormhole-sdk";
import { formatUnits } from "@ethersproject/units";
import { LCDClient, isTxError } from "@terra-money/terra.js";
import { ConnectedWallet, TxResult } from "@terra-money/wallet-provider";
import axios from "axios";
// import { TerraTokenMetadata } from "../hooks/useTerraTokenMap";
import { TERRA_GAS_PRICES_URL, getTerraConfig } from "./consts";

export const NATIVE_TERRA_DECIMALS = 6;
export const LUNA_CLASSIC_SYMBOL = "LUNC";

// TODO: terra2 support
export const getNativeTerraIcon = (symbol = "") =>
  `https://assets.terra.money/icon/60/${
    symbol === LUNA_CLASSIC_SYMBOL ? "Luna" : symbol.slice(0, symbol.length - 1)
  }.png`;

// inspired by https://github.com/terra-money/station/blob/dca7de43958ce075c6e46605622203b9859b0e14/src/lib/utils/format.ts#L38
export const formatNativeDenom = (denom = ""): string => {
  const unit = denom.slice(1).toUpperCase();
  const isValidTerra = isNativeTerra(denom);
  return denom === "uluna"
    ? LUNA_CLASSIC_SYMBOL
    : isValidTerra
    ? unit.slice(0, 2) + "TC"
    : "";
};

export const formatTerraNativeBalance = (balance = ""): string =>
  formatUnits(balance, 6);

export async function waitForTerraExecution(transaction: TxResult, chainId: TerraChainId) {
  const lcd = new LCDClient(getTerraConfig(chainId));
  let info;
  while (!info) {
    await new Promise((resolve) => setTimeout(resolve, 1000));
    try {
      info = await lcd.tx.txInfo(transaction.result.txhash);
    } catch (e) {
      console.error(e);
    }
  }
  if (isTxError(info)) {
    throw new Error(
      `Tx ${transaction.result.txhash}: error code ${info.code}: ${info.raw_log}`
    );
  }
  return info;
}

// TODO: terra2 support
export const isValidTerraAddress = (address: string) => {
  if (isNativeDenom(address)) {
    return true;
  }
  try {
    const startsWithTerra = address && address.startsWith("terra");
    const isParseable = canonicalAddress(address);
    const isLength20 = isParseable.length === 20;
    return !!(startsWithTerra && isParseable && isLength20);
  } catch (error) {
    return false;
  }
};

export async function postWithFees(
  wallet: ConnectedWallet,
  msgs: any[],
  memo: string,
  feeDenoms: string[],
  chainId: TerraChainId,
) {
  // don't try/catch, let errors propagate
  const lcd = new LCDClient(getTerraConfig(chainId));
  //Thus, we are going to pull it directly from the current FCD.
  const gasPrices = await axios
    .get(TERRA_GAS_PRICES_URL)
    .then((result) => result.data);

  const account = await lcd.auth.accountInfo(wallet.walletAddress);

  const feeEstimate = await lcd.tx.estimateFee(
    [
      {
        sequenceNumber: account.getSequenceNumber(),
        publicKey: account.getPublicKey(),
      },
    ],
    {
      msgs: [...msgs],
      memo,
      feeDenoms,
      gasPrices,
    }
  );

  const result = await wallet.post({
    msgs: [...msgs],
    memo,
    feeDenoms,
    gasPrices,
    fee: feeEstimate,
    // @ts-ignore, https://github.com/terra-money/terra.js/pull/295 (adding isClassic property)
    isClassic: lcd.config.isClassic,
  });

  return result;
}

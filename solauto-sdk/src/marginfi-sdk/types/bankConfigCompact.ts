/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */

import { PublicKey } from '@metaplex-foundation/umi';
import {
  Serializer,
  array,
  publicKey as publicKeySerializer,
  struct,
  u64,
  u8,
} from '@metaplex-foundation/umi/serializers';
import {
  BankOperationalState,
  BankOperationalStateArgs,
  InterestRateConfigCompact,
  InterestRateConfigCompactArgs,
  OracleSetup,
  OracleSetupArgs,
  RiskTier,
  RiskTierArgs,
  WrappedI80F48,
  WrappedI80F48Args,
  getBankOperationalStateSerializer,
  getInterestRateConfigCompactSerializer,
  getOracleSetupSerializer,
  getRiskTierSerializer,
  getWrappedI80F48Serializer,
} from '.';

/** TODO: Convert weights to (u64, u64) to avoid precision loss (maybe?) */
export type BankConfigCompact = {
  assetWeightInit: WrappedI80F48;
  assetWeightMaint: WrappedI80F48;
  liabilityWeightInit: WrappedI80F48;
  liabilityWeightMaint: WrappedI80F48;
  depositLimit: bigint;
  interestRateConfig: InterestRateConfigCompact;
  operationalState: BankOperationalState;
  oracleSetup: OracleSetup;
  oracleKey: PublicKey;
  autoPadding0: Array<number>;
  borrowLimit: bigint;
  riskTier: RiskTier;
  autoPadding1: Array<number>;
  /**
   * USD denominated limit for calculating asset value for initialization margin requirements.
   * Example, if total SOL deposits are equal to $1M and the limit it set to $500K,
   * then SOL assets will be discounted by 50%.
   *
   * In other words the max value of liabilities that can be backed by the asset is $500K.
   * This is useful for limiting the damage of orcale attacks.
   *
   * Value is UI USD value, for example value 100 -> $100
   */
  totalAssetValueInitLimit: bigint;
};

export type BankConfigCompactArgs = {
  assetWeightInit: WrappedI80F48Args;
  assetWeightMaint: WrappedI80F48Args;
  liabilityWeightInit: WrappedI80F48Args;
  liabilityWeightMaint: WrappedI80F48Args;
  depositLimit: number | bigint;
  interestRateConfig: InterestRateConfigCompactArgs;
  operationalState: BankOperationalStateArgs;
  oracleSetup: OracleSetupArgs;
  oracleKey: PublicKey;
  autoPadding0: Array<number>;
  borrowLimit: number | bigint;
  riskTier: RiskTierArgs;
  autoPadding1: Array<number>;
  /**
   * USD denominated limit for calculating asset value for initialization margin requirements.
   * Example, if total SOL deposits are equal to $1M and the limit it set to $500K,
   * then SOL assets will be discounted by 50%.
   *
   * In other words the max value of liabilities that can be backed by the asset is $500K.
   * This is useful for limiting the damage of orcale attacks.
   *
   * Value is UI USD value, for example value 100 -> $100
   */
  totalAssetValueInitLimit: number | bigint;
};

export function getBankConfigCompactSerializer(): Serializer<
  BankConfigCompactArgs,
  BankConfigCompact
> {
  return struct<BankConfigCompact>(
    [
      ['assetWeightInit', getWrappedI80F48Serializer()],
      ['assetWeightMaint', getWrappedI80F48Serializer()],
      ['liabilityWeightInit', getWrappedI80F48Serializer()],
      ['liabilityWeightMaint', getWrappedI80F48Serializer()],
      ['depositLimit', u64()],
      ['interestRateConfig', getInterestRateConfigCompactSerializer()],
      ['operationalState', getBankOperationalStateSerializer()],
      ['oracleSetup', getOracleSetupSerializer()],
      ['oracleKey', publicKeySerializer()],
      ['autoPadding0', array(u8(), { size: 6 })],
      ['borrowLimit', u64()],
      ['riskTier', getRiskTierSerializer()],
      ['autoPadding1', array(u8(), { size: 7 })],
      ['totalAssetValueInitLimit', u64()],
    ],
    { description: 'BankConfigCompact' }
  ) as Serializer<BankConfigCompactArgs, BankConfigCompact>;
}
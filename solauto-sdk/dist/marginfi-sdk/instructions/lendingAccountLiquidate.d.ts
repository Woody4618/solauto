/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */
import { Context, Pda, PublicKey, Signer, TransactionBuilder } from '@metaplex-foundation/umi';
import { Serializer } from '@metaplex-foundation/umi/serializers';
export type LendingAccountLiquidateInstructionAccounts = {
    marginfiGroup: PublicKey | Pda;
    assetBank: PublicKey | Pda;
    liabBank: PublicKey | Pda;
    liquidatorMarginfiAccount: PublicKey | Pda;
    signer: Signer;
    liquidateeMarginfiAccount: PublicKey | Pda;
    bankLiquidityVaultAuthority: PublicKey | Pda;
    bankLiquidityVault: PublicKey | Pda;
    bankInsuranceVault: PublicKey | Pda;
    tokenProgram?: PublicKey | Pda;
};
export type LendingAccountLiquidateInstructionData = {
    discriminator: Array<number>;
    assetAmount: bigint;
};
export type LendingAccountLiquidateInstructionDataArgs = {
    assetAmount: number | bigint;
};
export declare function getLendingAccountLiquidateInstructionDataSerializer(): Serializer<LendingAccountLiquidateInstructionDataArgs, LendingAccountLiquidateInstructionData>;
export type LendingAccountLiquidateInstructionArgs = LendingAccountLiquidateInstructionDataArgs;
export declare function lendingAccountLiquidate(context: Pick<Context, 'programs'>, input: LendingAccountLiquidateInstructionAccounts & LendingAccountLiquidateInstructionArgs): TransactionBuilder;
//# sourceMappingURL=lendingAccountLiquidate.d.ts.map
/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */
import { Context, Option, OptionOrNullable, Pda, PublicKey, Signer, TransactionBuilder } from '@metaplex-foundation/umi';
import { Serializer } from '@metaplex-foundation/umi/serializers';
import { UpdatePositionData, UpdatePositionDataArgs } from '../types';
export type MarginfiOpenPositionInstructionAccounts = {
    signer: Signer;
    marginfiProgram: PublicKey | Pda;
    systemProgram?: PublicKey | Pda;
    tokenProgram?: PublicKey | Pda;
    ataProgram?: PublicKey | Pda;
    rent?: PublicKey | Pda;
    solautoManager: PublicKey | Pda;
    solautoFeesWallet: PublicKey | Pda;
    solautoFeesSupplyTa: PublicKey | Pda;
    signerReferralState: PublicKey | Pda;
    referredByState?: PublicKey | Pda;
    referredBySupplyTa?: PublicKey | Pda;
    solautoPosition: PublicKey | Pda;
    marginfiGroup: PublicKey | Pda;
    marginfiAccount: PublicKey | Pda | Signer;
    supplyMint: PublicKey | Pda;
    supplyBank: PublicKey | Pda;
    positionSupplyTa: PublicKey | Pda;
    debtMint: PublicKey | Pda;
    debtBank: PublicKey | Pda;
    positionDebtTa: PublicKey | Pda;
    signerDebtTa?: PublicKey | Pda;
};
export type MarginfiOpenPositionInstructionData = {
    discriminator: number;
    positionData: UpdatePositionData;
    marginfiAccountSeedIdx: Option<bigint>;
};
export type MarginfiOpenPositionInstructionDataArgs = {
    positionData: UpdatePositionDataArgs;
    marginfiAccountSeedIdx: OptionOrNullable<number | bigint>;
};
export declare function getMarginfiOpenPositionInstructionDataSerializer(): Serializer<MarginfiOpenPositionInstructionDataArgs, MarginfiOpenPositionInstructionData>;
export type MarginfiOpenPositionInstructionArgs = MarginfiOpenPositionInstructionDataArgs;
export declare function marginfiOpenPosition(context: Pick<Context, 'programs'>, input: MarginfiOpenPositionInstructionAccounts & MarginfiOpenPositionInstructionArgs): TransactionBuilder;
//# sourceMappingURL=marginfiOpenPosition.d.ts.map
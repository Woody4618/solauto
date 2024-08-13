/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */
import { Context, Pda, PublicKey, Signer, TransactionBuilder } from '@metaplex-foundation/umi';
import { Serializer } from '@metaplex-foundation/umi/serializers';
export type LendingAccountWithdrawEmissionsInstructionAccounts = {
    marginfiGroup: PublicKey | Pda;
    marginfiAccount: PublicKey | Pda;
    signer: Signer;
    bank: PublicKey | Pda;
    emissionsMint: PublicKey | Pda;
    emissionsAuth: PublicKey | Pda;
    emissionsVault: PublicKey | Pda;
    destinationAccount: PublicKey | Pda;
    tokenProgram?: PublicKey | Pda;
};
export type LendingAccountWithdrawEmissionsInstructionData = {
    discriminator: Array<number>;
};
export type LendingAccountWithdrawEmissionsInstructionDataArgs = {};
export declare function getLendingAccountWithdrawEmissionsInstructionDataSerializer(): Serializer<LendingAccountWithdrawEmissionsInstructionDataArgs, LendingAccountWithdrawEmissionsInstructionData>;
export declare function lendingAccountWithdrawEmissions(context: Pick<Context, 'programs'>, input: LendingAccountWithdrawEmissionsInstructionAccounts): TransactionBuilder;
//# sourceMappingURL=lendingAccountWithdrawEmissions.d.ts.map
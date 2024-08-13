/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */

import {
  Context,
  Option,
  OptionOrNullable,
  Pda,
  PublicKey,
  Signer,
  TransactionBuilder,
  transactionBuilder,
} from '@metaplex-foundation/umi';
import {
  Serializer,
  array,
  bool,
  mapSerializer,
  option,
  struct,
  u64,
  u8,
} from '@metaplex-foundation/umi/serializers';
import {
  ResolvedAccount,
  ResolvedAccountsWithIndices,
  getAccountMetasAndSigners,
} from '../shared';

// Accounts.
export type LendingAccountWithdrawInstructionAccounts = {
  marginfiGroup: PublicKey | Pda;
  marginfiAccount: PublicKey | Pda;
  signer: Signer;
  bank: PublicKey | Pda;
  destinationTokenAccount: PublicKey | Pda;
  bankLiquidityVaultAuthority: PublicKey | Pda;
  bankLiquidityVault: PublicKey | Pda;
  tokenProgram?: PublicKey | Pda;
};

// Data.
export type LendingAccountWithdrawInstructionData = {
  discriminator: Array<number>;
  amount: bigint;
  withdrawAll: Option<boolean>;
};

export type LendingAccountWithdrawInstructionDataArgs = {
  amount: number | bigint;
  withdrawAll: OptionOrNullable<boolean>;
};

export function getLendingAccountWithdrawInstructionDataSerializer(): Serializer<
  LendingAccountWithdrawInstructionDataArgs,
  LendingAccountWithdrawInstructionData
> {
  return mapSerializer<
    LendingAccountWithdrawInstructionDataArgs,
    any,
    LendingAccountWithdrawInstructionData
  >(
    struct<LendingAccountWithdrawInstructionData>(
      [
        ['discriminator', array(u8(), { size: 8 })],
        ['amount', u64()],
        ['withdrawAll', option(bool())],
      ],
      { description: 'LendingAccountWithdrawInstructionData' }
    ),
    (value) => ({
      ...value,
      discriminator: [36, 72, 74, 19, 210, 210, 192, 192],
    })
  ) as Serializer<
    LendingAccountWithdrawInstructionDataArgs,
    LendingAccountWithdrawInstructionData
  >;
}

// Args.
export type LendingAccountWithdrawInstructionArgs =
  LendingAccountWithdrawInstructionDataArgs;

// Instruction.
export function lendingAccountWithdraw(
  context: Pick<Context, 'programs'>,
  input: LendingAccountWithdrawInstructionAccounts &
    LendingAccountWithdrawInstructionArgs
): TransactionBuilder {
  // Program ID.
  const programId = context.programs.getPublicKey(
    'marginfi',
    'MFv2hWf31Z9kbCa1snEPYctwafyhdvnV7FZnsebVacA'
  );

  // Accounts.
  const resolvedAccounts = {
    marginfiGroup: {
      index: 0,
      isWritable: false as boolean,
      value: input.marginfiGroup ?? null,
    },
    marginfiAccount: {
      index: 1,
      isWritable: true as boolean,
      value: input.marginfiAccount ?? null,
    },
    signer: {
      index: 2,
      isWritable: false as boolean,
      value: input.signer ?? null,
    },
    bank: { index: 3, isWritable: true as boolean, value: input.bank ?? null },
    destinationTokenAccount: {
      index: 4,
      isWritable: true as boolean,
      value: input.destinationTokenAccount ?? null,
    },
    bankLiquidityVaultAuthority: {
      index: 5,
      isWritable: true as boolean,
      value: input.bankLiquidityVaultAuthority ?? null,
    },
    bankLiquidityVault: {
      index: 6,
      isWritable: true as boolean,
      value: input.bankLiquidityVault ?? null,
    },
    tokenProgram: {
      index: 7,
      isWritable: false as boolean,
      value: input.tokenProgram ?? null,
    },
  } satisfies ResolvedAccountsWithIndices;

  // Arguments.
  const resolvedArgs: LendingAccountWithdrawInstructionArgs = { ...input };

  // Default values.
  if (!resolvedAccounts.tokenProgram.value) {
    resolvedAccounts.tokenProgram.value = context.programs.getPublicKey(
      'splToken',
      'TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA'
    );
    resolvedAccounts.tokenProgram.isWritable = false;
  }

  // Accounts in order.
  const orderedAccounts: ResolvedAccount[] = Object.values(
    resolvedAccounts
  ).sort((a, b) => a.index - b.index);

  // Keys and Signers.
  const [keys, signers] = getAccountMetasAndSigners(
    orderedAccounts,
    'programId',
    programId
  );

  // Data.
  const data = getLendingAccountWithdrawInstructionDataSerializer().serialize(
    resolvedArgs as LendingAccountWithdrawInstructionDataArgs
  );

  // Bytes Created On Chain.
  const bytesCreatedOnChain = 0;

  return transactionBuilder([
    { instruction: { keys, programId, data }, signers, bytesCreatedOnChain },
  ]);
}
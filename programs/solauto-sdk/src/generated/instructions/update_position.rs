//! This code was AUTOGENERATED using the kinobi library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun kinobi to update it.
//!
//! [https://github.com/metaplex-foundation/kinobi]
//!

use crate::generated::types::UpdatePositionData;
use borsh::BorshDeserialize;
use borsh::BorshSerialize;

/// Accounts.
pub struct UpdatePosition {
    pub signer: solana_program::pubkey::Pubkey,

    pub system_program: solana_program::pubkey::Pubkey,

    pub token_program: solana_program::pubkey::Pubkey,

    pub solauto_position: solana_program::pubkey::Pubkey,

    pub debt_mint: Option<solana_program::pubkey::Pubkey>,

    pub position_debt_ta: Option<solana_program::pubkey::Pubkey>,

    pub signer_debt_ta: Option<solana_program::pubkey::Pubkey>,
}

impl UpdatePosition {
    pub fn instruction(
        &self,
        args: UpdatePositionInstructionArgs,
    ) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(args, &[])
    }
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        args: UpdatePositionInstructionArgs,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(7 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.signer,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.system_program,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.token_program,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.solauto_position,
            false,
        ));
        if let Some(debt_mint) = self.debt_mint {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                debt_mint, false,
            ));
        } else {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                crate::SOLAUTO_ID,
                false,
            ));
        }
        if let Some(position_debt_ta) = self.position_debt_ta {
            accounts.push(solana_program::instruction::AccountMeta::new(
                position_debt_ta,
                false,
            ));
        } else {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                crate::SOLAUTO_ID,
                false,
            ));
        }
        if let Some(signer_debt_ta) = self.signer_debt_ta {
            accounts.push(solana_program::instruction::AccountMeta::new(
                signer_debt_ta,
                false,
            ));
        } else {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                crate::SOLAUTO_ID,
                false,
            ));
        }
        accounts.extend_from_slice(remaining_accounts);
        let mut data = UpdatePositionInstructionData::new().try_to_vec().unwrap();
        let mut args = args.try_to_vec().unwrap();
        data.append(&mut args);

        solana_program::instruction::Instruction {
            program_id: crate::SOLAUTO_ID,
            accounts,
            data,
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct UpdatePositionInstructionData {
    discriminator: u8,
}

impl UpdatePositionInstructionData {
    pub fn new() -> Self {
        Self { discriminator: 3 }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UpdatePositionInstructionArgs {
    pub update_position_data: UpdatePositionData,
}

/// Instruction builder for `UpdatePosition`.
///
/// ### Accounts:
///
///   0. `[signer]` signer
///   1. `[optional]` system_program (default to `11111111111111111111111111111111`)
///   2. `[optional]` token_program (default to `TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA`)
///   3. `[writable]` solauto_position
///   4. `[optional]` debt_mint
///   5. `[writable, optional]` position_debt_ta
///   6. `[writable, optional]` signer_debt_ta
#[derive(Default)]
pub struct UpdatePositionBuilder {
    signer: Option<solana_program::pubkey::Pubkey>,
    system_program: Option<solana_program::pubkey::Pubkey>,
    token_program: Option<solana_program::pubkey::Pubkey>,
    solauto_position: Option<solana_program::pubkey::Pubkey>,
    debt_mint: Option<solana_program::pubkey::Pubkey>,
    position_debt_ta: Option<solana_program::pubkey::Pubkey>,
    signer_debt_ta: Option<solana_program::pubkey::Pubkey>,
    update_position_data: Option<UpdatePositionData>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl UpdatePositionBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    #[inline(always)]
    pub fn signer(&mut self, signer: solana_program::pubkey::Pubkey) -> &mut Self {
        self.signer = Some(signer);
        self
    }
    /// `[optional account, default to '11111111111111111111111111111111']`
    #[inline(always)]
    pub fn system_program(&mut self, system_program: solana_program::pubkey::Pubkey) -> &mut Self {
        self.system_program = Some(system_program);
        self
    }
    /// `[optional account, default to 'TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA']`
    #[inline(always)]
    pub fn token_program(&mut self, token_program: solana_program::pubkey::Pubkey) -> &mut Self {
        self.token_program = Some(token_program);
        self
    }
    #[inline(always)]
    pub fn solauto_position(
        &mut self,
        solauto_position: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.solauto_position = Some(solauto_position);
        self
    }
    /// `[optional account]`
    #[inline(always)]
    pub fn debt_mint(&mut self, debt_mint: Option<solana_program::pubkey::Pubkey>) -> &mut Self {
        self.debt_mint = debt_mint;
        self
    }
    /// `[optional account]`
    #[inline(always)]
    pub fn position_debt_ta(
        &mut self,
        position_debt_ta: Option<solana_program::pubkey::Pubkey>,
    ) -> &mut Self {
        self.position_debt_ta = position_debt_ta;
        self
    }
    /// `[optional account]`
    #[inline(always)]
    pub fn signer_debt_ta(
        &mut self,
        signer_debt_ta: Option<solana_program::pubkey::Pubkey>,
    ) -> &mut Self {
        self.signer_debt_ta = signer_debt_ta;
        self
    }
    #[inline(always)]
    pub fn update_position_data(&mut self, update_position_data: UpdatePositionData) -> &mut Self {
        self.update_position_data = Some(update_position_data);
        self
    }
    /// Add an aditional account to the instruction.
    #[inline(always)]
    pub fn add_remaining_account(
        &mut self,
        account: solana_program::instruction::AccountMeta,
    ) -> &mut Self {
        self.__remaining_accounts.push(account);
        self
    }
    /// Add additional accounts to the instruction.
    #[inline(always)]
    pub fn add_remaining_accounts(
        &mut self,
        accounts: &[solana_program::instruction::AccountMeta],
    ) -> &mut Self {
        self.__remaining_accounts.extend_from_slice(accounts);
        self
    }
    #[allow(clippy::clone_on_copy)]
    pub fn instruction(&self) -> solana_program::instruction::Instruction {
        let accounts = UpdatePosition {
            signer: self.signer.expect("signer is not set"),
            system_program: self
                .system_program
                .unwrap_or(solana_program::pubkey!("11111111111111111111111111111111")),
            token_program: self.token_program.unwrap_or(solana_program::pubkey!(
                "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
            )),
            solauto_position: self.solauto_position.expect("solauto_position is not set"),
            debt_mint: self.debt_mint,
            position_debt_ta: self.position_debt_ta,
            signer_debt_ta: self.signer_debt_ta,
        };
        let args = UpdatePositionInstructionArgs {
            update_position_data: self
                .update_position_data
                .clone()
                .expect("update_position_data is not set"),
        };

        accounts.instruction_with_remaining_accounts(args, &self.__remaining_accounts)
    }
}

/// `update_position` CPI accounts.
pub struct UpdatePositionCpiAccounts<'a, 'b> {
    pub signer: &'b solana_program::account_info::AccountInfo<'a>,

    pub system_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub solauto_position: &'b solana_program::account_info::AccountInfo<'a>,

    pub debt_mint: Option<&'b solana_program::account_info::AccountInfo<'a>>,

    pub position_debt_ta: Option<&'b solana_program::account_info::AccountInfo<'a>>,

    pub signer_debt_ta: Option<&'b solana_program::account_info::AccountInfo<'a>>,
}

/// `update_position` CPI instruction.
pub struct UpdatePositionCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,

    pub signer: &'b solana_program::account_info::AccountInfo<'a>,

    pub system_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub solauto_position: &'b solana_program::account_info::AccountInfo<'a>,

    pub debt_mint: Option<&'b solana_program::account_info::AccountInfo<'a>>,

    pub position_debt_ta: Option<&'b solana_program::account_info::AccountInfo<'a>>,

    pub signer_debt_ta: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    /// The arguments for the instruction.
    pub __args: UpdatePositionInstructionArgs,
}

impl<'a, 'b> UpdatePositionCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: UpdatePositionCpiAccounts<'a, 'b>,
        args: UpdatePositionInstructionArgs,
    ) -> Self {
        Self {
            __program: program,
            signer: accounts.signer,
            system_program: accounts.system_program,
            token_program: accounts.token_program,
            solauto_position: accounts.solauto_position,
            debt_mint: accounts.debt_mint,
            position_debt_ta: accounts.position_debt_ta,
            signer_debt_ta: accounts.signer_debt_ta,
            __args: args,
        }
    }
    #[inline(always)]
    pub fn invoke(&self) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(&[], &[])
    }
    #[inline(always)]
    pub fn invoke_with_remaining_accounts(
        &self,
        remaining_accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(&[], remaining_accounts)
    }
    #[inline(always)]
    pub fn invoke_signed(
        &self,
        signers_seeds: &[&[&[u8]]],
    ) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(signers_seeds, &[])
    }
    #[allow(clippy::clone_on_copy)]
    #[allow(clippy::vec_init_then_push)]
    pub fn invoke_signed_with_remaining_accounts(
        &self,
        signers_seeds: &[&[&[u8]]],
        remaining_accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> solana_program::entrypoint::ProgramResult {
        let mut accounts = Vec::with_capacity(7 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.signer.key,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.system_program.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.token_program.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.solauto_position.key,
            false,
        ));
        if let Some(debt_mint) = self.debt_mint {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                *debt_mint.key,
                false,
            ));
        } else {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                crate::SOLAUTO_ID,
                false,
            ));
        }
        if let Some(position_debt_ta) = self.position_debt_ta {
            accounts.push(solana_program::instruction::AccountMeta::new(
                *position_debt_ta.key,
                false,
            ));
        } else {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                crate::SOLAUTO_ID,
                false,
            ));
        }
        if let Some(signer_debt_ta) = self.signer_debt_ta {
            accounts.push(solana_program::instruction::AccountMeta::new(
                *signer_debt_ta.key,
                false,
            ));
        } else {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                crate::SOLAUTO_ID,
                false,
            ));
        }
        remaining_accounts.iter().for_each(|remaining_account| {
            accounts.push(solana_program::instruction::AccountMeta {
                pubkey: *remaining_account.0.key,
                is_signer: remaining_account.1,
                is_writable: remaining_account.2,
            })
        });
        let mut data = UpdatePositionInstructionData::new().try_to_vec().unwrap();
        let mut args = self.__args.try_to_vec().unwrap();
        data.append(&mut args);

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::SOLAUTO_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(7 + 1 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.signer.clone());
        account_infos.push(self.system_program.clone());
        account_infos.push(self.token_program.clone());
        account_infos.push(self.solauto_position.clone());
        if let Some(debt_mint) = self.debt_mint {
            account_infos.push(debt_mint.clone());
        }
        if let Some(position_debt_ta) = self.position_debt_ta {
            account_infos.push(position_debt_ta.clone());
        }
        if let Some(signer_debt_ta) = self.signer_debt_ta {
            account_infos.push(signer_debt_ta.clone());
        }
        remaining_accounts
            .iter()
            .for_each(|remaining_account| account_infos.push(remaining_account.0.clone()));

        if signers_seeds.is_empty() {
            solana_program::program::invoke(&instruction, &account_infos)
        } else {
            solana_program::program::invoke_signed(&instruction, &account_infos, signers_seeds)
        }
    }
}

/// Instruction builder for `UpdatePosition` via CPI.
///
/// ### Accounts:
///
///   0. `[signer]` signer
///   1. `[]` system_program
///   2. `[]` token_program
///   3. `[writable]` solauto_position
///   4. `[optional]` debt_mint
///   5. `[writable, optional]` position_debt_ta
///   6. `[writable, optional]` signer_debt_ta
pub struct UpdatePositionCpiBuilder<'a, 'b> {
    instruction: Box<UpdatePositionCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> UpdatePositionCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(UpdatePositionCpiBuilderInstruction {
            __program: program,
            signer: None,
            system_program: None,
            token_program: None,
            solauto_position: None,
            debt_mint: None,
            position_debt_ta: None,
            signer_debt_ta: None,
            update_position_data: None,
            __remaining_accounts: Vec::new(),
        });
        Self { instruction }
    }
    #[inline(always)]
    pub fn signer(
        &mut self,
        signer: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.signer = Some(signer);
        self
    }
    #[inline(always)]
    pub fn system_program(
        &mut self,
        system_program: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.system_program = Some(system_program);
        self
    }
    #[inline(always)]
    pub fn token_program(
        &mut self,
        token_program: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.token_program = Some(token_program);
        self
    }
    #[inline(always)]
    pub fn solauto_position(
        &mut self,
        solauto_position: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.solauto_position = Some(solauto_position);
        self
    }
    /// `[optional account]`
    #[inline(always)]
    pub fn debt_mint(
        &mut self,
        debt_mint: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    ) -> &mut Self {
        self.instruction.debt_mint = debt_mint;
        self
    }
    /// `[optional account]`
    #[inline(always)]
    pub fn position_debt_ta(
        &mut self,
        position_debt_ta: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    ) -> &mut Self {
        self.instruction.position_debt_ta = position_debt_ta;
        self
    }
    /// `[optional account]`
    #[inline(always)]
    pub fn signer_debt_ta(
        &mut self,
        signer_debt_ta: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    ) -> &mut Self {
        self.instruction.signer_debt_ta = signer_debt_ta;
        self
    }
    #[inline(always)]
    pub fn update_position_data(&mut self, update_position_data: UpdatePositionData) -> &mut Self {
        self.instruction.update_position_data = Some(update_position_data);
        self
    }
    /// Add an additional account to the instruction.
    #[inline(always)]
    pub fn add_remaining_account(
        &mut self,
        account: &'b solana_program::account_info::AccountInfo<'a>,
        is_writable: bool,
        is_signer: bool,
    ) -> &mut Self {
        self.instruction
            .__remaining_accounts
            .push((account, is_writable, is_signer));
        self
    }
    /// Add additional accounts to the instruction.
    ///
    /// Each account is represented by a tuple of the `AccountInfo`, a `bool` indicating whether the account is writable or not,
    /// and a `bool` indicating whether the account is a signer or not.
    #[inline(always)]
    pub fn add_remaining_accounts(
        &mut self,
        accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> &mut Self {
        self.instruction
            .__remaining_accounts
            .extend_from_slice(accounts);
        self
    }
    #[inline(always)]
    pub fn invoke(&self) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed(&[])
    }
    #[allow(clippy::clone_on_copy)]
    #[allow(clippy::vec_init_then_push)]
    pub fn invoke_signed(
        &self,
        signers_seeds: &[&[&[u8]]],
    ) -> solana_program::entrypoint::ProgramResult {
        let args = UpdatePositionInstructionArgs {
            update_position_data: self
                .instruction
                .update_position_data
                .clone()
                .expect("update_position_data is not set"),
        };
        let instruction = UpdatePositionCpi {
            __program: self.instruction.__program,

            signer: self.instruction.signer.expect("signer is not set"),

            system_program: self
                .instruction
                .system_program
                .expect("system_program is not set"),

            token_program: self
                .instruction
                .token_program
                .expect("token_program is not set"),

            solauto_position: self
                .instruction
                .solauto_position
                .expect("solauto_position is not set"),

            debt_mint: self.instruction.debt_mint,

            position_debt_ta: self.instruction.position_debt_ta,

            signer_debt_ta: self.instruction.signer_debt_ta,
            __args: args,
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

struct UpdatePositionCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    signer: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    system_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    token_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    solauto_position: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    debt_mint: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    position_debt_ta: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    signer_debt_ta: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    update_position_data: Option<UpdatePositionData>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}
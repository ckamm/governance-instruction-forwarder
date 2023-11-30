use solana_program::{
    account_info::AccountInfo,
    entrypoint,
    instruction::{AccountMeta, Instruction},
    msg,
    program::invoke,
    program_error::ProgramError,
    pubkey::Pubkey,
    sysvar::{clock::Clock, Sysvar},
};

#[cfg(not(feature = "no-entrypoint"))]
entrypoint!(process_instruction);

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> entrypoint::ProgramResult {
    let timeout = u64::from_le_bytes(instruction_data[0..8].try_into().unwrap());
    let now_ts: u64 = Clock::get().unwrap().unix_timestamp.try_into().unwrap();
    if timeout < now_ts {
        msg!("instruction timeout expired");
        return Err(ProgramError::InvalidArgument);
    }
    let fwd_instruction_data = &instruction_data[8..];

    let signer = &accounts[0];
    if !signer.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let instruction = Instruction {
        program_id: *accounts[1].key,
        accounts: accounts[2..]
            .iter()
            .map(|ai| AccountMeta {
                pubkey: *ai.key,
                is_signer: ai.is_signer,
                is_writable: ai.is_writable,
            })
            .collect::<Vec<_>>(),
        data: fwd_instruction_data.to_vec(),
    };

    invoke(&instruction, &accounts[1..])?;

    Ok(())
}

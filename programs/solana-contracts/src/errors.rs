use anchor_lang::error_code;

#[error_code]
pub enum ProgramErrorCode {
    #[msg("Invalid Mint account space")]
    InvalidMintAccountSpace,
    #[msg("Cant initialize metadata_pointer")]
    CantInitializeMetadataPointer,
    #[msg("Invalid metadata account")]
    InvalidMetadataAccount,
    #[msg("Metadata account not found")]
    MetadataAccountNotFound,
}

use soroban_sdk::contracterror;

#[contracterror]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum StreamError {
    NotFound          = 1,
    Unauthorized      = 2,
    InvalidTimeRange  = 3,
    InvalidRate       = 4,
    AlreadyCancelled  = 5,
    NothingToWithdraw = 6,
    ZeroDeposit       = 7,
}

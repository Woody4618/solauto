pub mod clients;
pub mod constants;
pub mod entrypoint;
pub mod instructions;
pub mod processors;
pub mod state;
pub mod types;
pub mod utils;

// TODO:
// use solana_security_txt::security_txt;
// security_txt! {
//     name: "solauto",
//     project_url: "https://havenfi.io/",
//     contacts: "contacthavenfi@gmail.com",
//     policy: "https://github.com/haven-fi/solauto/blob/master/SECURITY.md",
//     preferred_languages: "en",
//     source_code: "https://github.com/haven-fi/solauto"
// }

use solana_program::declare_id;
declare_id!("AutoyKBRaHSBHy9RsmXCZMy6nNFAg5FYijrvZyQcNLV");
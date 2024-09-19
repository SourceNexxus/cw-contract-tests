use cosmwasm_std::Empty;
use cw721_base::MintMsg;

let mint_msg = MintMsg::<Empty> {
    token_id: "source-logo-1".to_string(),
    owner: "<owner-address>".to_string(),
    token_uri: Some("https://2352959449-files.gitbook.io/~/files/v0/b/gitbook-x-prod.appspot.com/o/spaces%2F2ufmW7exCclo14ABKCcm%2Fuploads%2FTn4GWYaW1ZchQDYghUe0%2Fsource-logo.svg?alt=media&token=c53d45f9-a1a3-44f6-9730-c245e206e5ec".to_string()),
    extension: Empty {},
};

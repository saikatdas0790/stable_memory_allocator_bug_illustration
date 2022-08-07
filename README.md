## Steps to Reproduce error

- dfx start --clean
- dfx deploy --no-wallet
- Open the Candid UI explorer
- Run one of the query methods. No issues
- Run the update method to insert SPrincipal into stable collections
- Run the query method again. Error

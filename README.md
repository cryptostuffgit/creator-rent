# Creator Rent
## A Solana Program that allows NFT creators to collect rent on their NFTs

## About
The creator-rent smart contract allows NFT collection owners to start a rent collection plan for their NFT collection by setting the rent collection period (ex. once a month) and a rent price. Any wallet can pay rent for as many periods as they'd like for any NFT in the collection. Rent can only be collected at the begging of the cycle and any payments made for the future are put in escrow in the rent program. The creator can update the rent price however any NFT that hasn't missed a payment will pay at its grandfathered price. This program allows NFT collection owners to make additional revenues by token gating functionality to NFT owners who have paid their rent (ex access to discord server).

## Demo
https://youtu.be/OKWH4vwMLC4

### Use the Program
1. Install Rust 
2. Install Solana
3. Install Yarn
4. Install Anchor
5. In `Anchor.toml` set the provider to localnet and pointing to your local keypair
6. Open a terminal and run `./start-validator` this will start a local validator and pull the accounts for the nfts we created for this porject on devnet
7. In another terminal run `anchor test --skip-local-validator`. The test will create a collection, pay rent for an nft, collect the rent, update the collection and then delete it.

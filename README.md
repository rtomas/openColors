# openColors

WIP

The project is designed to create a colour palette for the web. The colours are stored in the smart contract secuencially, so the first colour is the first colour in the blockchain, the second colour is the second colour in the blockchain, and so on. People can add colours to the pallet.

It will be used in Rococo for educational purposes. So people can learn how to interact with the blockchain, have a polkadot wallet and ask for ROCs from the tap. Also, I would like to have a simple user interface to interact with the contract usign useink library.

I'll make 3 tutorials:

1. The smart contract in ink!
2. The UI with next.js & useink library
3. Using it with a wallet and some ROCs

## Usage

test:
`cargo test`

compile:
`cargo contract build --release`

To deployed it go to : https://contracts-ui.substrate.io/

Contracts(Rococo) -> Upload Wasm -> openColors.contract -> Deploy
v0.1 -> 5EiMDgeApcbGXMEDof4nmAj9VSnbomy67pBZKfWVbsoguMuk

Transfer NEAR contract
======================

This is a NEAR contract that allows you to transfer tokens to other accounts.
Usually, you don't need such a contract, but currently, [NEAR BOS](https://docs.near.org/bos) does [not support native NEAR transfers](https://t.me/neardev/29391), so this is a quick proxy to do that.

See the BOS component demo [here](https://near.org/near/widget/ComponentDetailsPage?src=frol.near/widget/TransferNEAR&tab=source).

This contract is deployed to `transfer-near.near` on NEAR and thanks to [nesdie SDK](https://github.com/austinabell/nesdie), it requires only 1TGas attached to the transaction to transfer NEAR tokens to any account, and it is also just 916 bytes compiled to Wasm.

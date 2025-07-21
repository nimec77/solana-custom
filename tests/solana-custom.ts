import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SolanaCustom } from "../target/types/solana_custom";
import { assert } from "chai";

describe("solana-custom", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.solanaCustom as Program<SolanaCustom>;

  it('can send a new message', async () => {
    // Generate a new key pair for the message account (message).
    const message = anchor.web3.Keypair.generate();
    await program.methods
      .sendMessage('space exploration', 'Discovering new worlds!')
      .accounts({
        message: message.publicKey, // Define the message account.
        author: program.provider.wallet.publicKey, // Author account (payer)
      })
      .signers([message]) // Add the message key pair as a signer.
      .rpc();
    const messageAccount = await program.account.message.fetch(message.publicKey);

    assert.equal(messageAccount.author.toBase58(), program.provider.publicKey.toBase58());
    assert.equal(messageAccount.topic, 'space exploration');
    assert.equal(messageAccount.content, 'Discovering new worlds!');
    assert.ok(messageAccount.timestamp);
  });
});

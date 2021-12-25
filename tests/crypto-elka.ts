import * as assert from "assert";
import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { CryptoElka } from '../target/types/crypto_elka';

describe('crypto-elka', () => {

  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.CryptoElka as Program<CryptoElka>;

  it('can place a ball', async () => {
    const ballAccountKeys = anchor.web3.Keypair.generate();
    const ballNftAccountKeys = anchor.web3.Keypair.generate();

    const tx = await program.rpc.placeBall(
        42,
        "message",
        {
            accounts: {
                ball: ballAccountKeys.publicKey,
                ballNft: ballNftAccountKeys.publicKey,
                creator: program.provider.wallet.publicKey,
                systemProgram: anchor.web3.SystemProgram.programId,
            },
            signers: [ballAccountKeys, ballNftAccountKeys],
        }
    );

    console.log("Your transaction signature", tx);

    // Fetch the account details of the created tweet.
    const ballAccount = await program.account.ball.fetch(ballAccountKeys.publicKey);
    console.log(ballAccount);

    assert.equal(ballAccount.creator.toBase58(), program.provider.wallet.publicKey.toBase58());
    assert.equal(ballAccount.place, 42);
    assert.equal(ballAccount.message, "message");

    assert.ok(ballAccount.timestamp);
  });

  it('can fetch all tweets', async () => {
      const ballAccounts = await program.account.ball.all();
      assert.equal(ballAccounts.length, 1);
      console.log(ballAccounts);
  });
});

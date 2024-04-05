import * as anchor from "@coral-xyz/anchor";
import { AnchorError, Program } from "@coral-xyz/anchor";
import { assert } from "chai";
import { Solanasc } from "../target/types/solanasc";

describe("solanasc", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Solanasc as Program<Solanasc>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize(new anchor.BN(777), "Hello :DDD").rpc();
    console.log("Your transaction signature", tx);
  });

  it("Array test", async () => {
    const tx = await program.methods.array([new anchor.BN(777), new anchor.BN(888)]).rpc();
    console.log("Your transaction signature is", tx)
  })

  it("Error test", async () => {
    // Add your test here.

    try {
      const tx = await program.methods.limitRange(new anchor.BN(9)).rpc();
      console.log("Your transaction signature", tx);
    } catch (_err) {
      assert.isTrue(_err instanceof AnchorError);
      const err: AnchorError = _err;

      const errMsg = "a is too small";

      assert.strictEqual(err.error.errorMessage, errMsg);
      console.log("Error number:", err.error.errorCode.number);
    }

    try {
      const tx = await program.methods.limitRange(new anchor.BN(101)).rpc();
      console.log("Your transaction signature", tx);
    } catch (_err) {
      assert.isTrue(_err instanceof AnchorError);
      const err: AnchorError = _err;

      const errMsg = "a is too big";

      assert.strictEqual(err.error.errorMessage, errMsg);
      console.log("Error number:", err.error.errorCode.number);
    }
    
  })
});

import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { CleverTodo } from "../target/types/clever_todo";

describe("to-do-d-app", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.ToDoDApp as Program<CleverTodo>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initializeUser().rpc();
    console.log("Your transaction signature", tx);
  });
});

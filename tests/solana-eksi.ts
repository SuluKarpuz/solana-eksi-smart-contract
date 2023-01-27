import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { SolanaEksi } from "../target/types/solana_eksi";

describe("solana-eksi", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.SolanaEksi as Program<SolanaEksi>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});

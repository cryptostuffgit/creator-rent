import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { CreatorRent } from "../target/types/creator_rent";
import { BN } from "@project-serum/anchor";
import { PublicKey } from "@solana/web3.js";
import { expect } from 'chai';
import { Metadata } from "@metaplex-foundation/mpl-token-metadata";
import fs from 'mz/fs';

const TOKEN_METADATA_PROGRAM_ID = new anchor.web3.PublicKey(
  "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"
);

const LAMPORT_PER_SOL = 1000000000;
const provider = anchor.AnchorProvider.env();

describe("creator-rent", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(provider);
  const program = anchor.workspace.CreatorRent as Program<CreatorRent>;

  it("Does Stuff", async () => {
    const systemProgram = new PublicKey(
      '11111111111111111111111111111111',
    );

    const creator = await getKeypair("./creator-key.json");
    await airdrop(creator.publicKey);

    const collectionMint = new PublicKey("8Awb3gsF6hYdQTvRm6oHhzQJQkxeB4AUVsAAhoB9K6wi");
    const collectionMetadata = await getMetadata(collectionMint);

    console.log(collectionMetadata.toBase58());

    const [nftCollectionKey, _] = (
      await anchor.web3.PublicKey.findProgramAddress(
        [Buffer.from('collection'), collectionMint.toBuffer()],
        program.programId,
      )
    );

    console.log("nftCollectionKey", nftCollectionKey.toBase58());

    console.log("sending");
    const createCollectionTx = await program.methods
      .createCollection({
        periodInSeconds: 604800,
        rentPrice: new BN(0.5 * LAMPORT_PER_SOL)
      })
      .accounts({
        payer: creator.publicKey,
        nftCollection: nftCollectionKey,
        collectionMint: collectionMint,
        collectionMetadata: collectionMetadata,
        systemProgram: systemProgram
      })
      .signers([creator])
      .rpc()
      .catch(e => console.log(e));
    
    console.log("tx", createCollectionTx);
    let nftCollectionData = await program.account.nftCollection.fetch(nftCollectionKey);
    expect(nftCollectionData.creator.toString()).to.equal(creator.publicKey.toString());
    
    const payer1 = anchor.web3.Keypair.generate();
    await airdrop(payer1.publicKey);

    const nftMint = new PublicKey("7BxEkXY4BWD6kCXJnDb3qa6JM9DrKGbtjuHTsMTX1kvh");
    const nftMetadata = await getMetadata(nftMint);
    
    console.log("metadata", nftMetadata.toBase58());

    const [nftRentKey, _nftRentbump] = (
      await anchor.web3.PublicKey.findProgramAddress(
        [Buffer.from('nft'), nftMint.toBuffer()],
        program.programId,
      )
    );

    console.log("rent key", nftRentKey.toBase58())
      
    const payRentTx = await program.methods
      .payRent({
        payment: new BN(1 * LAMPORT_PER_SOL)
      })
      .accounts({
        payer: payer1.publicKey,
        nftCollection: nftCollectionKey,
        nftMint: nftMint,
        nftMetadata: nftMetadata,
        nftRent: nftRentKey,
        systemProgram: systemProgram
      })
      .signers([payer1])
      .rpc()
      .catch(e => console.log(e));
      
    console.log("tx", payRentTx)

    nftCollectionData = await program.account.nftCollection.fetch(nftCollectionKey);
    console.log(nftCollectionData.rentCollection);
    console.log(JSON.stringify(nftCollectionData.rentCollection));

    let nftRentData = await program.account.nftRent.fetch(nftRentKey);
    console.log(nftRentData);
    console.log(JSON.stringify(nftRentData));

    const collectRentTx = await program.methods
      .collectRent()
      .accounts({
        payer: creator.publicKey,
        nftCollection: nftCollectionKey,
        collectionMint: collectionMint
      })
      .signers([creator])
      .rpc()
      .catch(e => console.log(e));

    console.log("tx", collectRentTx);

    nftCollectionData = await program.account.nftCollection.fetch(nftCollectionKey);
    console.log(nftCollectionData.rentCollection);
    console.log(JSON.stringify(nftCollectionData.rentCollection));

    // const updateRentTx = await program.methods
    //   .updateRent({
    //     newRentPrice: new BN(0.25 * LAMPORT_PER_SOL)
    //   })
    //   .accounts({
    //     payer: creator.publicKey,
    //     nftCollection: nftCollectionKey,
    //     systemProgram: systemProgram
    //   })
    //   .signers([creator])
    //   .rpc()
    //   .catch(e => console.log(e));

    // console.log("tx", updateRentTx)

    // nftCollectionData = await program.account.nftCollection.fetch(nftCollectionKey);
    // console.log(nftCollectionData.rentPrice.toString());

    // const deleteCollection = await program.methods
    //   .deleteCollection()
    //   .accounts({
    //     payer: creator.publicKey,
    //     nftCollection: nftCollectionKey,
    //     systemProgram: systemProgram
    //   })
    //   .signers([creator])
    //   .rpc()
    //   .catch(e => console.log(e));

    // console.log("tx", deleteCollection)

    // nftCollectionData = await program.account.nftCollection.fetch(nftCollectionKey);
    // console.log(nftCollectionData.rentPrice.toString());
  });
});

const getMetadata = async (
  mint: anchor.web3.PublicKey
): Promise<anchor.web3.PublicKey> => {
  return (
    await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from("metadata"),
        TOKEN_METADATA_PROGRAM_ID.toBuffer(),
        mint.toBuffer(),
      ],
      TOKEN_METADATA_PROGRAM_ID
    )
  )[0];
};

const airdrop = async (publicKey: PublicKey) => {
  const signature = await provider.connection.requestAirdrop(publicKey, 2 * LAMPORT_PER_SOL);
  await provider.connection.confirmTransaction(signature);
}

export const getKeypair = async (filepath: string): Promise<anchor.web3.Keypair>  => {
  const secretKeyString = await fs.readFile(filepath, { encoding: 'utf8' });
  const secretKey = Uint8Array.from(JSON.parse(secretKeyString));
  return anchor.web3.Keypair.fromSecretKey(secretKey)
}

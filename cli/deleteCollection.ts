import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { CreatorRent } from '../target/types/creator_rent';
import { PublicKey } from "@solana/web3.js";
import fs from 'mz/fs';

const TOKEN_METADATA_PROGRAM_ID = new anchor.web3.PublicKey(
    "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"
);

export const getKeypair = async (filepath: string): Promise<anchor.web3.Keypair>  => {
  const secretKeyString = await fs.readFile(filepath, { encoding: 'utf8' });
  const secretKey = Uint8Array.from(JSON.parse(secretKeyString));
  return anchor.web3.Keypair.fromSecretKey(secretKey)
}

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

var provider = anchor.AnchorProvider.env();

const deleteCollection = async () => {
  anchor.setProvider(provider);
  const program = anchor.workspace.CreatorRent as Program<CreatorRent>;
  
  const systemProgram = new PublicKey(
    '11111111111111111111111111111111',
  );

  const payer = await getKeypair("./creator-key.json");
  const collectionMint = new PublicKey("8Awb3gsF6hYdQTvRm6oHhzQJQkxeB4AUVsAAhoB9K6wi");
    const collectionMetadata = await getMetadata(collectionMint);

  const [nftCollectionKey, _] = (
    await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from('collection'), collectionMint.toBuffer()],
      program.programId,
    )
  );

  
  console.log(`Deleting Collection: ${nftCollectionKey}`);
  console.log(payer.publicKey.toBase58());
  console.log(nftCollectionKey.toBase58());
  const tx = await program.methods
    .deleteCollection()
    .accounts({
      payer: payer.publicKey,
      nftCollection: nftCollectionKey,
      systemProgram: systemProgram
    })
    .signers([payer])
    .rpc({skipPreflight: true});

    console.log(tx);
};

deleteCollection();
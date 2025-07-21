import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { NftMarketPlace } from "../target/types/nft_market_place";
import { ASSOCIATED_TOKEN_PROGRAM_ID, getAssociatedTokenAddress, getOrCreateAssociatedTokenAccount, TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import { createSignerFromKeypair, generateSigner, keypairIdentity, percentAmount, publicKey } from "@metaplex-foundation/umi";
import { Keypair, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js";
import { closeEscrowAccount, createNft, findMasterEditionPda, findMetadataPda, MPL_TOKEN_METADATA_PROGRAM_ID, mplTokenMetadata, setCollectionSize, verifySizedCollectionItem } from "@metaplex-foundation/mpl-token-metadata";


describe("nft-market-place", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(anchor.AnchorProvider.env());
  const admin = provider.wallet.publicKey;
  const program = anchor.workspace.nft_market_place as Program<NftMarketPlace>;
  console.log("program id is: ", program.programId);


  const umi = createUmi("https://api.devnet.solana.com");
  const creatorWallet = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(provider.wallet.payer.secretKey));
  const creator = createSignerFromKeypair(umi, creatorWallet);
  const nft_mint = generateSigner(umi);
  const collecion_mint = generateSigner(umi);
  console.log("nft mint: " , nft_mint.publicKey);

  const maker = Keypair.generate();
  const taker = Keypair.generate();
  let makerAta: anchor.web3.PublicKey;
  let takerAta: anchor.web3.PublicKey;
  let vault: anchor.web3.PublicKey;
  let nftMetadata;
  let collectionMetadata;
  let collectionMasterEdition;

  const connection = provider.connection;
  const payer = provider.wallet;

  const name = "random";
  const market_place = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("marketplace"), Buffer.from(name)], program.programId)[0];
  const treasury = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("treasury"), market_place.toBytes()], program.programId)[0];
  const reward_mint = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("rewards"), market_place.toBytes()], program.programId)[0];
  const listing = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("listing"),
      new anchor.web3.PublicKey(nft_mint.publicKey).toBytes(),
      Buffer.from("v2")
    ],
    program.programId
  )[0];

  it.skip("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods
      .initialize(name)
      .accountsPartial({
        admin: admin,
        marketPlace: market_place,
        treasury,
        rewardMint: reward_mint,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();
    console.log("Your transaction signature", tx);
  });

  it("seding sol to maker and taker!", async () => {
    console.log("sending sol to maker and taker");
    //transfering sol to maker taker
    let sendSol = async (to: PublicKey, amount: number) => {
      let tx = new anchor.web3.Transaction().add(
        anchor.web3.SystemProgram.transfer({
          fromPubkey: provider.publicKey,
          toPubkey: to,
          lamports: amount,
        })
      )
      await provider.sendAndConfirm(tx, [provider.wallet.payer]);
      console.log(`sent sol to ${to}`);
    }

    await sendSol(maker.publicKey, 0.1 * LAMPORTS_PER_SOL);
    await sendSol(taker.publicKey, 0.1 * LAMPORTS_PER_SOL);

    await sleep(1000);
  })

  it("Mints NFTs and verifies collections", async () => {
    // Mint Collection NFT
    const result = await createNft(umi, {
      mint: collecion_mint,
      name: "GM",
      symbol: "GM",
      uri: "https://arweave.net/123",
      sellerFeeBasisPoints: percentAmount(5.5, 2),
      collectionDetails: {
        __kind: 'V1',
        size: 10
      }
    }).sendAndConfirm(umi);

    console.log(`Created Collection NFT: ${collecion_mint.publicKey.toString()}`);


    // Mint NFT into maker's ATA
    await createNft(umi, {
      mint: nft_mint,
      name: "GM",
      symbol: "GM",
      uri: "https://arweave.net/123",
      sellerFeeBasisPoints: percentAmount(5.5),
      collection: { verified: false, key: collecion_mint.publicKey },
      tokenOwner: publicKey(maker.publicKey) // Corrected to use maker's public key
    }).sendAndConfirm(umi);
    console.log(`Created NFT: ${nft_mint.publicKey.toString()}`);

    // Verify Collection
    collectionMetadata = findMetadataPda(umi, { mint: collecion_mint.publicKey });
    collectionMasterEdition = findMasterEditionPda(umi, { mint: collecion_mint.publicKey });
    nftMetadata = findMetadataPda(umi, { mint: nft_mint.publicKey });
    await verifySizedCollectionItem(umi, {
      metadata: nftMetadata,
      collectionAuthority: creator,
      collectionMint: collecion_mint.publicKey,
      collection: collectionMetadata,
      collectionMasterEditionAccount: collectionMasterEdition,
    }).sendAndConfirm(umi);
    console.log("Collection NFT Verified!");

    //getting maker ata
    makerAta = (await getOrCreateAssociatedTokenAccount(
      connection,
      maker,
      new anchor.web3.PublicKey(nft_mint.publicKey),
      maker.publicKey
    )).address;
    //getting taker ata
    takerAta = (await getOrCreateAssociatedTokenAccount(
      connection,
      taker,
      new anchor.web3.PublicKey(nft_mint.publicKey),
      taker.publicKey
    )).address;

    vault = await getAssociatedTokenAddress(
      new anchor.web3.PublicKey(nft_mint.publicKey),
      listing,
      true
    );

  });

  before(() => {
    umi.use(keypairIdentity(creator));
    umi.use(mplTokenMetadata());
  });


  it("list", async () => {
    // Add your test here.
    let price = 1 * anchor.web3.LAMPORTS_PER_SOL;
    const nftMetadata = findMetadataPda(umi, { mint: nft_mint.publicKey });
    const nftEdition = findMasterEditionPda(umi, { mint: nft_mint.publicKey });
    const info = await provider.connection.getAccountInfo(listing);
      if (info) {
        throw new Error("Listing PDA already exists and will cause deserialization error.");
      }

    const tx = await program.methods
      .listNft(new anchor.BN(price))
      .accountsPartial({
        maker: maker.publicKey,
        makerMint: nft_mint.publicKey,
        makerMintAta: makerAta,
        collectionMint: collecion_mint.publicKey,
        listing: listing,
        marketPlace: market_place,
        vault: vault,
        metadata: new anchor.web3.PublicKey(nftMetadata[0]),
        masterEdition: new anchor.web3.PublicKey(nftEdition[0]),
        metadataProgram: MPL_TOKEN_METADATA_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        systemProgram: anchor.web3.SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([maker])
      .rpc();
    console.log("\nListing Initialized!");
    console.log("Your transaction signature", tx);

  });

});


function sleep(ms: number) {
  return new Promise(resolve => setTimeout(resolve, ms));
}
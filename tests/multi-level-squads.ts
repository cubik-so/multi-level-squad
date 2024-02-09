import * as anchor from '@coral-xyz/anchor';
import {
  KP1,
  KP2,
  KP3,
  SEED_MULTISIG,
  SEED_PREFIX,
  SQUADS_PROGRAM_ID,
  adminPair,
  createKey,
  createLocalhostConnection,
  createProgram,
  generateFundedKeypair,
  generateKeypair,
} from './utils';
import { adminPDA, memberPDA } from './pda';
import { config } from 'dotenv';
config();
describe('multi-level-squads', () => {
  const connection = createLocalhostConnection();
  it.skip('Add Pairs', async () => {
    const kp1 = await generateFundedKeypair(connection);
    const kp2 = await generateFundedKeypair(connection);
    const kp3 = await generateFundedKeypair(connection);
    console.log('kp-1  ', anchor.utils.bytes.bs58.encode(kp1.secretKey));
    console.log(kp1.publicKey.toBase58());
    console.log('----------');
    console.log('kp-2  ', anchor.utils.bytes.bs58.encode(kp2.secretKey));
    console.log(kp2.publicKey.toBase58());
    console.log('----------');
    console.log('kp-3  ', anchor.utils.bytes.bs58.encode(kp3.secretKey));
    console.log(kp3.publicKey.toBase58());
  });
  it.skip('Create Admin', async () => {
    const admin = await adminPair(connection);
    const wallet = new anchor.Wallet(admin);
    const program = createProgram(wallet);

    const createKey = generateKeypair();
    const [multisigPDA] = anchor.web3.PublicKey.findProgramAddressSync(
      [SEED_PREFIX, SEED_MULTISIG, createKey.publicKey.toBytes()],
      SQUADS_PROGRAM_ID
    );
    const tx = await program.methods
      .createAdminSquad(
        [KP1().publicKey, KP2().publicKey, KP3().publicKey],
        2,
        null,
        0,
        ''
      )
      .accounts({
        authority: admin.publicKey,
        createKey: createKey.publicKey,
        adminSquad: adminPDA()[0],
        multisig: multisigPDA,
        squadsProgram: SQUADS_PROGRAM_ID,
        systemProgram: anchor.web3.SystemProgram.programId,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      })
      .signers([createKey])
      .rpc();

    console.log(tx);
  });
  it.skip('Fetch', async () => {
    const wallet = new anchor.Wallet(KP3());
    const program = createProgram(wallet);
    const a = await program.account.vault.fetch(adminPDA()[0]);
    console.log(a);
    // const data = await connection.getParsedAccountInfo(adminPDA()[0]);
  });
  it.skip('Member', async () => {
    // const admin = await adminPair(connection);
    const wallet = new anchor.Wallet(KP3());
    const program = createProgram(wallet);
    const createKey = generateKeypair();
    console.log('createKey', adminPDA()[0]);

    const pda = await program.account.vault.fetch(adminPDA()[0]);

    const [multisigPDA] = anchor.web3.PublicKey.findProgramAddressSync(
      [SEED_PREFIX, SEED_MULTISIG, createKey.publicKey.toBytes()],
      SQUADS_PROGRAM_ID
    );
    const tx = await program.methods
      .createMemberSquad(
        [pda.vaultKey, KP3().publicKey],
        2,
        pda.vaultKey,
        0,
        ''
      )
      .accounts({
        authority: KP3().publicKey,
        memberSquad: memberPDA(createKey.publicKey)[0],
        createKey: createKey.publicKey,
        multisig: multisigPDA,
        squadsProgram: SQUADS_PROGRAM_ID,
        systemProgram: anchor.web3.SystemProgram.programId,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      })
      .signers([createKey, KP3()])
      .rpc();
    console.log(tx);
  });

  it('Member', async () => {
    const wallet = new anchor.Wallet(KP3());
    const program = createProgram(wallet);

    const mem = memberPDA(createKey);
    const data = await program.account.vault.fetch(mem[0]);

    console.log(data);
  });
});

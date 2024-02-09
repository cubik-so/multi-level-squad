import {
  AnchorProvider,
  Program,
  Wallet,
  utils,
  web3,
} from '@coral-xyz/anchor';
import { MultiLevelSquads, IDL } from '../target/types/multi_level_squads';

export const PROGRAM_ID = new web3.PublicKey(
  '9LS4pBp29znJwJABdzpsMwoef9rYLcJnzKsGaANh4zEQ'
);

export function createLocalhostConnection() {
  return new web3.Connection('http://127.0.0.1:8899', 'confirmed');
}

export const SQUADS_PROGRAM_ID = new web3.PublicKey(
  'SQDS4ep65T869zMMBKyuUq6aD6EgTu8psMjkvj52pCf'
);

function toUtfBytes(str: string): Uint8Array {
  return new TextEncoder().encode(str);
}
export const SEED_PREFIX = toUtfBytes('multisig');
export const SEED_MULTISIG = toUtfBytes('multisig');

export const createProgram = (wallet: Wallet): Program<MultiLevelSquads> => {
  return new Program(
    IDL,
    PROGRAM_ID,
    new AnchorProvider(createLocalhostConnection(), wallet, {
      commitment: 'confirmed',
    })
  ) as unknown as Program<MultiLevelSquads>;
};

export function generateKeypair() {
  return web3.Keypair.generate();
}

export async function generateFundedKeypair(connection: web3.Connection) {
  const keypair = web3.Keypair.generate();

  const tx = await connection.requestAirdrop(
    keypair.publicKey,
    1 * web3.LAMPORTS_PER_SOL
  );
  const latestBlockHash = await connection.getLatestBlockhash();
  await connection.confirmTransaction({
    blockhash: latestBlockHash.blockhash,
    lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
    signature: tx,
  });

  return keypair;
}

export const adminPair = async (connection: web3.Connection) => {
  const keypair = web3.Keypair.fromSecretKey(
    utils.bytes.bs58.decode(process.env.KP || '')
  );

  const tx = await connection.requestAirdrop(
    keypair.publicKey,
    1 * web3.LAMPORTS_PER_SOL
  );
  const latestBlockHash = await connection.getLatestBlockhash();
  await connection.confirmTransaction({
    blockhash: latestBlockHash.blockhash,
    lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
    signature: tx,
  });
  return keypair;
  // return generateFundedKeypair(createLocalhostConnection());
};

export const KP1 = () => {
  return web3.Keypair.fromSecretKey(
    utils.bytes.bs58.decode(process.env.KP1 || '')
  );
};
export const KP2 = () => {
  return web3.Keypair.fromSecretKey(
    utils.bytes.bs58.decode(process.env.KP2 || '')
  );
};
export const KP3 = () => {
  return web3.Keypair.fromSecretKey(
    utils.bytes.bs58.decode(process.env.KP3 || '')
  );
};

export const createKey = new web3.PublicKey(
  '2d7ocDGkpFpYhq6aNP774YapUB65i5o7otBJ7arrJhnc'
);

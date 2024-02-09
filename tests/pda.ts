import { web3 } from '@coral-xyz/anchor';
import { PROGRAM_ID } from './utils';

export const adminPDA = () => {
  return web3.PublicKey.findProgramAddressSync(
    [Buffer.from('admin')],
    PROGRAM_ID
  );
};
export const memberPDA = (createKey: web3.PublicKey) => {
  return web3.PublicKey.findProgramAddressSync(
    [Buffer.from('member'), createKey.toBuffer()],
    PROGRAM_ID
  );
};

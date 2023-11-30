import { Worker, NearAccount } from 'near-workspaces';
import anyTest, { TestFn } from 'ava';

const test = anyTest as TestFn<{
  worker: Worker;
  accounts: Record<string, NearAccount>;
}>;

test.beforeEach(async (t) => {
  // Init the worker and start a Sandbox server
  const worker = await Worker.init();

  // Deploy contract
  const root = worker.rootAccount;
  const contract = await root.createSubAccount('test-account');
  // Get wasm file path from package.json test script in folder above
  await contract.deploy(
    process.argv[2],
  );

  // Save state for test runs, it is unique for each test 
  t.context.worker = worker;
  t.context.accounts = { root, contract };
});

test.afterEach.always(async (t) => {
  // Stop Sandbox server
  await t.context.worker.tearDown().catch((error) => {
    console.log('Failed to stop the Sandbox:', error);
  });
});

test('new initializes contract states', async (t) => {
  const { contract } = t.context.accounts;
  await contract.call(contract, 'new', { owner_id: contract.accountId, metadata: { spec: 'nft-1.0.0', name: 'test certificate', symbol: 'NCD' } });

  t.is(true, true);
});


test('new fails to re-initializes contract state', async (t) => {
  const { contract } = t.context.accounts;
  await contract.call(contract, 'new', { owner_id: contract.accountId, metadata: { spec: 'nft-1.0.0', name: 'test certificate', symbol: 'NCD' } });

  const result = await contract.callRaw(contract, 'new', { owner_id: contract.accountId, metadata: { spec: 'nft-1.0.0', name: 'test certificate', symbol: 'NCD' } });

  t.is(result.failed, true);
});



test('nft_mint does mint nft for the user', async (t) => {
  const { contract } = t.context.accounts;
  await contract.call(contract, 'new', { owner_id: contract.accountId, metadata: { spec: 'nft-1.0.0', name: 'test certificate', symbol: 'NCD' } });

  const minted_token: any = await contract.call(contract, 'nft_mint', {
    token_id: '1',
    token_owner_id: 'someone.testnet',
    token_metadata: {
      title: 'Certificate',
      description: 'Successfully completed the course'
    }
  }, { gas: '300000000000000', attachedDeposit: '2490000000000000000000' });

  t.is(minted_token.owner_id, 'someone.testnet');
});

test('nft_transfer fails so transfer the nft - making it account-bound', async (t) => {
  const { contract, root } = t.context.accounts;
  await contract.call(contract, 'new', { owner_id: contract.accountId, metadata: { spec: 'nft-1.0.0', name: 'test certificate', symbol: 'NCD' } });
  await contract.call(contract, 'nft_mint', {
    token_id: '1',
    token_owner_id: root.accountId,
    token_metadata: {
      title: 'Certificate',
      description: 'Successfully completed the course'
    }
  }, { gas: '300000000000000', attachedDeposit: '2490000000000000000000' });

  const result: any = await root.callRaw(contract, 'nft_transfer', { receiver_id: 'someone_else.testnet', token_id: '1' });

  t.is(result.failed, true);
});

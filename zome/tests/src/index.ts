import { Config, Orchestrator, InstallAgentsHapps } from '@holochain/tryorama';
import path from 'path'


const delay = (ms) => new Promise((r) => setTimeout(r, ms))

const config = Config.gen();

const installable: InstallAgentsHapps = [
  // agent 0...
  [
    // happ 0...
    [
      // dna 0...
      path.join('../username.dna.gz')
    ]
  ]
] 

const orchestrator = new Orchestrator()

function setUsername(username) {
  return (conductor) =>
    conductor.call('username', 'set_username', username);
};

function getUsername(agent_pubkey) {
  return (conductor) => 
    conductor.call('username', 'get_username', agent_pubkey)
}
function getAllUsernames() {
  return (conductor) =>
    conductor.call('username', 'get_all_usernames', {})
};

function getAgentPubkeyFromUsername(username) {
  return (conductor) =>
    conductor.call('username', 'get_agent_pubkey_from_username', username)
};

function getMyUsername() {
  return (conductor) =>
    conductor.call('username', 'get_my_username', {})
};


orchestrator.registerScenario('create username', async (s, t) => {
  const [alice, bobby] = await s.players([config, config]);
  const [alice_lobby_happ] = await alice.installAgentsHapps(installable);
  const [bobby_lobby_happ] = await bobby.installAgentsHapps(installable);
  const alice_conductor = alice_lobby_happ[0].cells[0];
  const bobby_conductor = bobby_lobby_happ[0].cells[0];

  const [dna_hash_1, agent_pubkey_alice] = alice_conductor.cellId('alice');
  const [dna_hash_2, agent_pubkey_bobby] = bobby_conductor.cellId('bobby');

  // alice sets her username
  const set_username_alice = await setUsername('alice')(alice_conductor);
  t.deepEqual(set_username_alice.username, 'alice');
  t.deepEqual(set_username_alice.agent_id, agent_pubkey_alice);

  // bob sets his username
  const set_username_bobbo = await setUsername('bobbo')(bobby_conductor);
  await delay(1000);
  t.deepEqual(set_username_bobbo.username, 'bobbo');
  t.deepEqual(set_username_bobbo.agent_id, agent_pubkey_bobby);

  // // error: bob sets a new username for himself
  // const set_username_bobbo_2 = await setUsername('bobbo')(bobby_conductor, 'bobbo');
  // await delay(1000);

  // // error: carly sets an already taken username
  // const set_username_carly = await setUsername('bobbo')(conductor, 'carly');
  // await delay(1000);
  });

  orchestrator.registerScenario('get usernames', async (s, t) => {
  const [alice, bobby, clark] = await s.players([config, config, config]);
  const [alice_lobby_happ] = await alice.installAgentsHapps(installable);
  const [bobby_lobby_happ] = await bobby.installAgentsHapps(installable);
  const [clark_lobby_happ] = await clark.installAgentsHapps(installable);
  const alice_conductor = alice_lobby_happ[0].cells[0];
  const bobby_conductor = bobby_lobby_happ[0].cells[0];
  const clark_conductor = clark_lobby_happ[0].cells[0];

  const [dna_hash_1, agent_pubkey_alice] = alice_conductor.cellId('alice');
  const [dna_hash_2, agent_pubkey_bobby] = bobby_conductor.cellId('bobby');
  const [dna_hash_3, agent_pubkey_clark] = clark_conductor.cellId('clark');

  // // error: alice gets own nonexistent 
  // const profile_alice_none = await getMyUsername()(conductor, 'alice');
  // t.deepEqual(profile_alice_none.username, 'alice');
  // t.deepEqual(profile_alice_none.agent_id, pubkey_alice);
  // await delay(1000);

  const set_username_alice = await setUsername('alice')(alice_conductor,);
  const set_username_bobbo = await setUsername('bobbo')(bobby_conductor);
  await delay(1000);

  // alice gets own profile
  const profile_alice = await getMyUsername()(alice_conductor,);
  t.deepEqual(profile_alice.username, 'alice');
  await delay(1000);

  // bobbo gets own profile
  const profile_bobbo = await getMyUsername()(bobby_conductor,);
  t.deepEqual(profile_bobbo.username, 'bobbo');

  // alice gets bobbo's profile using his agent pubkey
  const profile_bobbo_alice_2 = await getUsername(agent_pubkey_bobby)(alice_conductor,);
  t.deepEqual(profile_bobbo_alice_2.username, 'bobbo');

  // bobbo gets alice's username using her agent pubkey
  const profile_alice_bobbo_2 = await getUsername(agent_pubkey_alice)(bobby_conductor,);
  t.deepEqual(profile_alice_bobbo_2.username, 'alice');

  // alice gets all usernames
  const profile_all_alice = await getAllUsernames()(alice_conductor,);
  t.deepEqual(profile_all_alice.length, 2);

  // bobbo gets all usernames
  const profile_all_bobbo = await getAllUsernames()(bobby_conductor,);
  t.deepEqual(profile_all_bobbo.length, 2);

  // alice gets her address from her username
  const alice_address = await getAgentPubkeyFromUsername('alice')(alice_conductor,);
  t.deepEqual(alice_address, agent_pubkey_alice)

  // bobbo gets his address from his username
  const bobbo_address = await getAgentPubkeyFromUsername('bobbo')(bobby_conductor,);
  t.deepEqual(bobbo_address, agent_pubkey_bobby)

  // alice gets bobbo's address from his username
  const bobbo_address_alice = await getAgentPubkeyFromUsername('bobbo')(alice_conductor,);
  t.deepEqual(bobbo_address_alice, agent_pubkey_bobby)

  // bobbo gets alice's address grom her username
  const alice_address_bobbo = await getAgentPubkeyFromUsername('alice')(bobby_conductor,);
  t.deepEqual(alice_address_bobbo, agent_pubkey_alice)

  // // error: alice gets non-existent carly's profile
  // const profile_carly = await getUsername(agent_pubkey_clark)(alice_conductor, 'alice');
  // t.deepEqual(profile_carly.username, 'carly');
  // t.deepEqual(profile_carly.agent_id, pubkey_carly);
  // await delay(1000);

  // // error: alice gets pubkey from non-existent profile
  // const get_pubkey_carly = await getAgentPubkeyFromUsername('carly')(alice_conductor, 'alice');
  // t.deepEqual(get_pubkey_carly, agent_pubkey_clark);
})


orchestrator.run()


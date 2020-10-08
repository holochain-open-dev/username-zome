import { Config } from '@holochain/tryorama'
import * as _ from 'lodash'

const delay = (ms) => new Promise((r) => setTimeout(r, ms))

const config = Config.gen({
  alice: Config.dna('../example-dna.dna.gz', null),
  bobbo: Config.dna('../example-dna.dna.gz', null),
  carly: Config.dna('../example-dna.dna.gz', null)
})

function setUsername(username) {
  return (conductor, caller) =>
    conductor.call(caller, 'username', 'set_username', username);
};

function getUsername(agent_pubkey) {
  return (conductor, caller) => 
    conductor.call(caller, 'username', 'get_username', agent_pubkey)
}
function getAllUsernames() {
  return (conductor, caller) =>
    conductor.call(caller, 'username', 'get_all_usernames', null)
};

function getAgentPubkeyFromUsername(username) {
  return (conductor, caller) =>
    conductor.call(caller, 'username', 'get_agent_pubkey_from_username', username)
};

// function getMyUsername() {
//   return (conductor, caller) =>
//     conductor.call(caller, 'profiles', 'get_my_username', null)
// };

module.exports = (orchestrator) => {
  orchestrator.registerScenario('create username', async (s, t) => {
    const { conductor } = await s.players({ conductor: config })
    await conductor.spawn()

    const [dna_hash_alice, pubkey_alice] = conductor.cellId('alice');
    const [dna_hash_bobbo, pubkey_bobbo] = conductor.cellId('bobbo');

    // alice sets her username
    const set_username_alice = await setUsername('alice')(conductor, 'alice');
    await delay(1000);
    t.deepEqual(set_username_alice.username, 'alice');
    t.deepEqual(set_username_alice.agent_id, pubkey_alice);

    // bob sets his username
    const set_username_bobbo = await setUsername('bobbo')(conductor, 'bobbo');
    await delay(1000);
    t.deepEqual(set_username_bobbo.username, 'bobbo');
    t.deepEqual(set_username_bobbo.agent_id, pubkey_bobbo);

    // // error: bob sets a new username for himself
    // const set_username_bobbo_2 = await setUsername('bobbo')(conductor, 'bobbo');
    // await delay(1000);
    
    // // error: carly sets an already taken username
    // const set_username_carly = await setUsername('bobbo')(conductor, 'carly');
    // await delay(1000);
  });

  orchestrator.registerScenario('get usernames', async (s, t) => {
    const { conductor } = await s.players({ conductor: config })
    await conductor.spawn()

    const [dna_hash_alice, pubkey_alice] = conductor.cellId('alice');
    const [dna_hash_bobbo, pubkey_bobbo] = conductor.cellId('bobbo');
    const [dna_hash_carly, pubkey_carly] = conductor.cellId('carly');

    const set_username_alice = await setUsername('alice')(conductor, 'alice');
    await delay(1000);
    const set_username_bobbo = await setUsername('bobbo')(conductor, 'bobbo');
    await delay(1000);

    // // alice gets own profile
    // const profile_alice = await getMyUsername()(conductor, 'alice');
    // t.deepEqual(profile_alice.username, 'alice');
    // t.deepEqual(profile_alice.agent_id, pubkey_alice);
    // await delay(1000);

    // // bobbo gets own profile
    // const profile_bobbo = await getMyUsername()(conductor, 'bobbo');
    // t.deepEqual(profile_bobbo.username, 'bobbo');
    // t.deepEqual(profile_bobbo.agent_id, pubkey_bobbo);
    // await delay(1000);

    // alice gets bobbo's profile using his agent pubkey
    const profile_bobbo_alice_2 = await getUsername(pubkey_bobbo)(conductor, 'alice');
    t.deepEqual(profile_bobbo_alice_2.username, 'bobbo');
    t.deepEqual(profile_bobbo_alice_2.agent_id, pubkey_bobbo);
    await delay(1000);

    // bobbo gets alice's username using her agent pubkey
    const profile_alice_bobbo_2 = await getUsername(pubkey_alice)(conductor, 'bobbo');
    t.deepEqual(profile_alice_bobbo_2.username, 'alice');
    t.deepEqual(profile_alice_bobbo_2.agent_id, pubkey_alice);
    await delay(1000);

    // alice gets all usernames
    const profile_all_alice = await getAllUsernames()(conductor, 'alice');
    t.deepEqual(profile_all_alice.length, 2);
    await delay(1000);

    // bobbo gets all usernames
    const profile_all_bobbo = await getAllUsernames()(conductor, 'bobbo');
    t.deepEqual(profile_all_bobbo.length, 2);

    // alice gets her address from her username
    const alice_address = await getAgentPubkeyFromUsername('alice')(conductor, 'alice');
    t.deepEqual(alice_address, pubkey_alice)

    // bobbo gets his address from his username
    const bobbo_address = await getAgentPubkeyFromUsername('bobbo')(conductor, 'bobbo');
    t.deepEqual(bobbo_address, pubkey_bobbo)

    // alice gets bobbo's address from his username
    const bobbo_address_alice = await getAgentPubkeyFromUsername('bobbo')(conductor, 'alice');
    t.deepEqual(bobbo_address_alice, pubkey_bobbo)

    // bobbo gets alice's address grom her username
    const alice_address_bobbo = await getAgentPubkeyFromUsername('alice')(conductor, 'bobbo');
    t.deepEqual(alice_address_bobbo, pubkey_alice)

    // // error: alice gets non-existent carly's profile
    // const profile_carly = await getUsername(pubkey_carly)(conductor, 'alice');
    // t.deepEqual(profile_carly.username, 'carly');
    // t.deepEqual(profile_carly.agent_id, pubkey_carly);
    // await delay(1000);
    
    // // error: alice gets pubkey from non-existent profile
    // const get_pubkey_carly = await getAgentPubkeyFromUsername('carly')(conductor, 'alice');
    // t.deepEqual(get_pubkey_carly, pubkey_carly);
  })
}

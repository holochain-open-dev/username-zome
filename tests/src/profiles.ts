import { Config } from '@holochain/tryorama'
import { profile } from 'console'
import * as _ from 'lodash'

const delay = (ms) => new Promise((r) => setTimeout(r, ms))
const sleep = (ms) => new Promise((resolve) => setTimeout(() => resolve(), ms));

// Configure a conductor with two identical DNAs,
// differentiated by UUID, nicknamed "alice" and "bobbo"
const config = Config.gen({
  alice: Config.dna('../profiles.dna.gz', null),
  bobbo: Config.dna('../profiles.dna.gz', null),
})

module.exports = (orchestrator) => {
  orchestrator.registerScenario('kizuna test', async (s, t) => {
    // spawn the conductor process
    const { conductor } = await s.players({ conductor: config })
    await conductor.spawn()

    const set_bob = await conductor.call('bobbo', 'profiles', 'create_profile', { username: "bob" });
    console.log("set bob");
    console.log(set_bob);

    await sleep(100);

    const set_alice = await conductor.call('alice', 'profiles', 'create_profile', { username: "alice" });
    console.log("set alice");
    console.log(set_alice);

    await sleep(100);

    const profile_bob = await conductor.call('bobbo', 'profiles', 'get_my_profile', null);
    console.log("bob gets profile");
    console.log(profile_bob);

    await sleep(100);

    const profile_alice = await conductor.call('alice', 'profiles', 'get_my_profile', null);
    console.log("alice gets profile");
    console.log(profile_alice);
    
    await sleep(100);

    const all = await conductor.call('bobbo', 'profiles', 'get_all_profiles', null);
    console.log("all profiles");
    console.log(all);

    await sleep(100);

    const bob_alice = await conductor.call('bobbo', 'profiles', 'get_profile_from_username', "alice" );
    console.log("bob gets alice");
    console.log(bob_alice);

    await sleep(100);

    const alice_bob = await conductor.call('alice', 'profiles', 'get_profile_from_username', "bob" );
    console.log("alice gets bob");
    console.log(alice_bob);

    await sleep(100);

    const address_of_alice = await conductor.call('alice', 'profiles', 'get_address_from_username', "alice" );
    console.log("alice's address");
    console.log(address_of_alice);

    const address_of_bob = await conductor.call('alice', 'profiles', 'get_address_from_username', "bob" );
    console.log("bob's address");
    console.log(address_of_bob);

    await sleep(100);

    const test_path = await conductor.call('alice', 'profiles', 'test_path_profile', { username: 'alice' });
    console.log("root path");
    console.log(test_path);

    await sleep(100);
    
    // const all = await conductor.call('bobbo', 'profiles', 'get_all_profiles', { username: "bob" });
    // console.log("nicko get");
    // console.log(all);

    // const all3 = await conductor.call('bobbo', 'profiles', 'get_all_profiles', { username: "bo"} );
    // console.log("path ni bo");
    // console.log(all3);

    // const all4 = await conductor.call('bobbo', 'profiles', 'get_all_profiles', { username: "bo" } );
    // console.log("path ni bo");
    // console.log(all4);

    // address is the HEADER address
    // const set = await conductor.call('bobbo', 'profiles', 'set_username', "Bob" );
    // console.log("nicko");
    // console.log(set);
    
    // s.consistency();

    // const get = await conductor.call('bobbo', 'profiles', 'get_username', set );
    // console.log("nicko 2");
    // console.log(get);

    // const set_2 = await conductor.call('bobbo', 'profiles', 'set_username', "Bob" );
    // console.log("nicko 3");
    // console.log(set_2);
    
    // s.consistency();

    // const all_agents = await conductor.call('bobbo', 'profiles', 'get_all_agents', null);
    // console.log("nicko 4");
    // console.log(all_agents);

    // // get back the post at the address (header) where it was just written
    // // const list = await conductor.call('bobbo', 'profiles', 'get_all_agents',  null)
    // // address is the HEADER address
    // // console.log(list);
    // // t.deepEqual(list[0], "Bob");
  })
}

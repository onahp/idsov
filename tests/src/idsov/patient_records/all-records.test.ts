import { assert, test } from "vitest";

import { runScenario, dhtSync, CallableCell } from "@holochain/tryorama";
import {
  NewEntryAction,
  ActionHash,
  Record,
  AppBundleSource,
  fakeActionHash,
  fakeAgentPubKey,
  fakeEntryHash,
} from "@holochain/client";
import { decode } from "@msgpack/msgpack";

import { createPatientRecord } from "./common.js";

test("create a PatientRecord and get all records", async () => {
  await runScenario(async (scenario) => {
    // Construct proper paths for your app.
    // This assumes app bundle created by the `hc app pack` command.
    const testAppPath = process.cwd() + "/../workdir/idsov.happ";

    // Set up the app to be installed
    const appSource = { appBundleSource: { path: testAppPath } };

    // Add 2 players with the test app to the Scenario. The returned players
    // can be destructured.
    const [alice, bob] = await scenario.addPlayersWithApps([
      appSource,
      appSource,
    ]);

    // Shortcut peer discovery through gossip and register all agents in every
    // conductor of the scenario.
    await scenario.shareAllAgents();

    // Bob gets all records
    let collectionOutput: Link[] = await bob.cells[0].callZome({
      zome_name: "patient_records",
      fn_name: "get_all_records",
      payload: null,
    });
    assert.equal(collectionOutput.length, 0);

    // Alice creates a PatientRecord
    const createRecord: Record = await createPatientRecord(alice.cells[0]);
    assert.ok(createRecord);

    await dhtSync([alice, bob], alice.cells[0].cell_id[0]);

    // Bob gets all records again
    collectionOutput = await bob.cells[0].callZome({
      zome_name: "patient_records",
      fn_name: "get_all_records",
      payload: null,
    });
    assert.equal(collectionOutput.length, 1);
    assert.deepEqual(
      createRecord.signed_action.hashed.hash,
      collectionOutput[0].target,
    );

    // Alice deletes the PatientRecord
    await alice.cells[0].callZome({
      zome_name: "patient_records",
      fn_name: "delete_patient_record",
      payload: createRecord.signed_action.hashed.hash,
    });

    await dhtSync([alice, bob], alice.cells[0].cell_id[0]);

    // Bob gets all records again
    collectionOutput = await bob.cells[0].callZome({
      zome_name: "patient_records",
      fn_name: "get_all_records",
      payload: null,
    });
    assert.equal(collectionOutput.length, 0);
  });
});

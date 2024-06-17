import { assert, test } from "vitest";

import { runScenario, dhtSync, CallableCell } from "@holochain/tryorama";
import {
  NewEntryAction,
  ActionHash,
  Record,
  AppBundleSource,
  fakeDnaHash,
  fakeActionHash,
  fakeAgentPubKey,
  fakeEntryHash,
} from "@holochain/client";
import { decode } from "@msgpack/msgpack";

import { createPatientRecord, samplePatientRecord } from "./common.js";

test("create PatientRecord", async () => {
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

    // Alice creates a PatientRecord
    const record: Record = await createPatientRecord(alice.cells[0]);
    assert.ok(record);
  });
});

test("create and read PatientRecord", async () => {
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

    const sample = await samplePatientRecord(alice.cells[0]);

    // Alice creates a PatientRecord
    const record: Record = await createPatientRecord(alice.cells[0], sample);
    assert.ok(record);

    // Wait for the created entry to be propagated to the other node.
    await dhtSync([alice, bob], alice.cells[0].cell_id[0]);

    // Bob gets the created PatientRecord
    const createReadOutput: Record = await bob.cells[0].callZome({
      zome_name: "patient_records",
      fn_name: "get_original_patient_record",
      payload: record.signed_action.hashed.hash,
    });
    assert.deepEqual(
      sample,
      decode((createReadOutput.entry as any).Present.entry) as any,
    );
  });
});

test("create and update PatientRecord", async () => {
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

    // Alice creates a PatientRecord
    const record: Record = await createPatientRecord(alice.cells[0]);
    assert.ok(record);

    const originalActionHash = record.signed_action.hashed.hash;

    // Alice updates the PatientRecord
    let contentUpdate: any = await samplePatientRecord(alice.cells[0]);
    let updateInput = {
      original_patient_record_hash: originalActionHash,
      previous_patient_record_hash: originalActionHash,
      updated_patient_record: contentUpdate,
    };

    let updatedRecord: Record = await alice.cells[0].callZome({
      zome_name: "patient_records",
      fn_name: "update_patient_record",
      payload: updateInput,
    });
    assert.ok(updatedRecord);

    // Wait for the updated entry to be propagated to the other node.
    await dhtSync([alice, bob], alice.cells[0].cell_id[0]);

    // Bob gets the updated PatientRecord
    const readUpdatedOutput0: Record = await bob.cells[0].callZome({
      zome_name: "patient_records",
      fn_name: "get_latest_patient_record",
      payload: updatedRecord.signed_action.hashed.hash,
    });
    assert.deepEqual(
      contentUpdate,
      decode((readUpdatedOutput0.entry as any).Present.entry) as any,
    );

    // Alice updates the PatientRecord again
    contentUpdate = await samplePatientRecord(alice.cells[0]);
    updateInput = {
      original_patient_record_hash: originalActionHash,
      previous_patient_record_hash: updatedRecord.signed_action.hashed.hash,
      updated_patient_record: contentUpdate,
    };

    updatedRecord = await alice.cells[0].callZome({
      zome_name: "patient_records",
      fn_name: "update_patient_record",
      payload: updateInput,
    });
    assert.ok(updatedRecord);

    // Wait for the updated entry to be propagated to the other node.
    await dhtSync([alice, bob], alice.cells[0].cell_id[0]);

    // Bob gets the updated PatientRecord
    const readUpdatedOutput1: Record = await bob.cells[0].callZome({
      zome_name: "patient_records",
      fn_name: "get_latest_patient_record",
      payload: updatedRecord.signed_action.hashed.hash,
    });
    assert.deepEqual(
      contentUpdate,
      decode((readUpdatedOutput1.entry as any).Present.entry) as any,
    );

    // Bob gets all the revisions for PatientRecord
    const revisions: Record[] = await bob.cells[0].callZome({
      zome_name: "patient_records",
      fn_name: "get_all_revisions_for_patient_record",
      payload: originalActionHash,
    });
    assert.equal(revisions.length, 3);
    assert.deepEqual(
      contentUpdate,
      decode((revisions[2].entry as any).Present.entry) as any,
    );
  });
});

test("create and delete PatientRecord", async () => {
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

    const sample = await samplePatientRecord(alice.cells[0]);

    // Alice creates a PatientRecord
    const record: Record = await createPatientRecord(alice.cells[0], sample);
    assert.ok(record);

    await dhtSync([alice, bob], alice.cells[0].cell_id[0]);

    // Alice deletes the PatientRecord
    const deleteActionHash = await alice.cells[0].callZome({
      zome_name: "patient_records",
      fn_name: "delete_patient_record",
      payload: record.signed_action.hashed.hash,
    });
    assert.ok(deleteActionHash);

    // Wait for the entry deletion to be propagated to the other node.
    await dhtSync([alice, bob], alice.cells[0].cell_id[0]);

    // Bob gets the oldest delete for the PatientRecord
    const oldestDeleteForPatientRecord = await bob.cells[0].callZome({
      zome_name: "patient_records",
      fn_name: "get_oldest_delete_for_patient_record",
      payload: record.signed_action.hashed.hash,
    });
    assert.ok(oldestDeleteForPatientRecord);

    // Bob gets the deletions for the PatientRecord
    const deletesForPatientRecord = await bob.cells[0].callZome({
      zome_name: "patient_records",
      fn_name: "get_all_deletes_for_patient_record",
      payload: record.signed_action.hashed.hash,
    });
    assert.equal(deletesForPatientRecord.length, 1);
  });
});

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

import { createComment, sampleComment } from "./common.js";

test("create Comment", async () => {
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

    // Alice creates a Comment
    const record: Record = await createComment(alice.cells[0]);
    assert.ok(record);
  });
});

test("create and read Comment", async () => {
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

    const sample = await sampleComment(alice.cells[0]);

    // Alice creates a Comment
    const record: Record = await createComment(alice.cells[0], sample);
    assert.ok(record);

    // Wait for the created entry to be propagated to the other node.
    await dhtSync([alice, bob], alice.cells[0].cell_id[0]);

    // Bob gets the created Comment
    const createReadOutput: Record = await bob.cells[0].callZome({
      zome_name: "patient_records",
      fn_name: "get_original_comment",
      payload: record.signed_action.hashed.hash,
    });
    assert.deepEqual(
      sample,
      decode((createReadOutput.entry as any).Present.entry) as any,
    );

    // Bob gets the PatientRecords for the new Comment
    let linksToPatientRecords: Link[] = await bob.cells[0].callZome({
      zome_name: "patient_records",
      fn_name: "get_comments_for_patient_record",
      payload: sample.patient_record_hash,
    });
    assert.equal(linksToPatientRecords.length, 1);
    assert.deepEqual(
      linksToPatientRecords[0].target,
      record.signed_action.hashed.hash,
    );
  });
});

test("create and update Comment", async () => {
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

    // Alice creates a Comment
    const record: Record = await createComment(alice.cells[0]);
    assert.ok(record);

    const originalActionHash = record.signed_action.hashed.hash;

    // Alice updates the Comment
    let contentUpdate: any = await sampleComment(alice.cells[0]);
    let updateInput = {
      original_comment_hash: originalActionHash,
      previous_comment_hash: originalActionHash,
      updated_comment: contentUpdate,
    };

    let updatedRecord: Record = await alice.cells[0].callZome({
      zome_name: "patient_records",
      fn_name: "update_comment",
      payload: updateInput,
    });
    assert.ok(updatedRecord);

    // Wait for the updated entry to be propagated to the other node.
    await dhtSync([alice, bob], alice.cells[0].cell_id[0]);

    // Bob gets the updated Comment
    const readUpdatedOutput0: Record = await bob.cells[0].callZome({
      zome_name: "patient_records",
      fn_name: "get_latest_comment",
      payload: updatedRecord.signed_action.hashed.hash,
    });
    assert.deepEqual(
      contentUpdate,
      decode((readUpdatedOutput0.entry as any).Present.entry) as any,
    );

    // Alice updates the Comment again
    contentUpdate = await sampleComment(alice.cells[0]);
    updateInput = {
      original_comment_hash: originalActionHash,
      previous_comment_hash: updatedRecord.signed_action.hashed.hash,
      updated_comment: contentUpdate,
    };

    updatedRecord = await alice.cells[0].callZome({
      zome_name: "patient_records",
      fn_name: "update_comment",
      payload: updateInput,
    });
    assert.ok(updatedRecord);

    // Wait for the updated entry to be propagated to the other node.
    await dhtSync([alice, bob], alice.cells[0].cell_id[0]);

    // Bob gets the updated Comment
    const readUpdatedOutput1: Record = await bob.cells[0].callZome({
      zome_name: "patient_records",
      fn_name: "get_latest_comment",
      payload: updatedRecord.signed_action.hashed.hash,
    });
    assert.deepEqual(
      contentUpdate,
      decode((readUpdatedOutput1.entry as any).Present.entry) as any,
    );

    // Bob gets all the revisions for Comment
    const revisions: Record[] = await bob.cells[0].callZome({
      zome_name: "patient_records",
      fn_name: "get_all_revisions_for_comment",
      payload: originalActionHash,
    });
    assert.equal(revisions.length, 3);
    assert.deepEqual(
      contentUpdate,
      decode((revisions[2].entry as any).Present.entry) as any,
    );
  });
});

test("create and delete Comment", async () => {
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

    const sample = await sampleComment(alice.cells[0]);

    // Alice creates a Comment
    const record: Record = await createComment(alice.cells[0], sample);
    assert.ok(record);

    await dhtSync([alice, bob], alice.cells[0].cell_id[0]);

    // Bob gets the PatientRecords for the new Comment
    let linksToPatientRecords: Link[] = await bob.cells[0].callZome({
      zome_name: "patient_records",
      fn_name: "get_comments_for_patient_record",
      payload: sample.patient_record_hash,
    });
    assert.equal(linksToPatientRecords.length, 1);
    assert.deepEqual(
      linksToPatientRecords[0].target,
      record.signed_action.hashed.hash,
    );

    // Alice deletes the Comment
    const deleteActionHash = await alice.cells[0].callZome({
      zome_name: "patient_records",
      fn_name: "delete_comment",
      payload: record.signed_action.hashed.hash,
    });
    assert.ok(deleteActionHash);

    // Wait for the entry deletion to be propagated to the other node.
    await dhtSync([alice, bob], alice.cells[0].cell_id[0]);

    // Bob gets the oldest delete for the Comment
    const oldestDeleteForComment = await bob.cells[0].callZome({
      zome_name: "patient_records",
      fn_name: "get_oldest_delete_for_comment",
      payload: record.signed_action.hashed.hash,
    });
    assert.ok(oldestDeleteForComment);

    // Bob gets the deletions for the Comment
    const deletesForComment = await bob.cells[0].callZome({
      zome_name: "patient_records",
      fn_name: "get_all_deletes_for_comment",
      payload: record.signed_action.hashed.hash,
    });
    assert.equal(deletesForComment.length, 1);

    // Bob gets the PatientRecords for the Comment again
    linksToPatientRecords = await bob.cells[0].callZome({
      zome_name: "patient_records",
      fn_name: "get_comments_for_patient_record",
      payload: sample.patient_record_hash,
    });
    assert.equal(linksToPatientRecords.length, 0);

    // Bob gets the deleted PatientRecords for the Comment
    const deletedLinksToPatientRecords = await bob.cells[0].callZome({
      zome_name: "patient_records",
      fn_name: "get_deleted_comments_for_patient_record",
      payload: sample.patient_record_hash,
    });
    assert.equal(deletedLinksToPatientRecords.length, 1);
  });
});

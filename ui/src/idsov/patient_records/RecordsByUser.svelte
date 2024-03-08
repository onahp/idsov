<script lang="ts">
import { onMount, getContext } from 'svelte';
import '@material/mwc-circular-progress';
import type { EntryHash, Record, AgentPubKey, ActionHash, AppAgentClient, NewEntryAction } from '@holochain/client';
import { clientContext } from '../../contexts';
import PatientRecordListItem from './PatientRecordListItem.svelte';
import type { PatientRecordsSignal } from './types';
import "@holochain-open-dev/profiles/dist/elements/agent-avatar.js";
import "@holochain-open-dev/profiles/dist/elements/my-profile.js";
import "@holochain-open-dev/profiles/dist/elements/profiles-context.js";
import type { Profile } from "@holochain-open-dev/profiles";
import { encodeHashToBase64, fakeAgentPubKey } from "@holochain/client";

// PatientRecordListItem imports
import { decode } from '@msgpack/msgpack';
import type { PatientRecord } from './types';
let record: Record | undefined;
let patientRecord: PatientRecord | undefined;
export let patientRecordHash: ActionHash;

let client: AppAgentClient = (getContext(clientContext) as any).getClient();

let hashes: Array<ActionHash> | undefined;
let loading = true;
let error: any = undefined;

export let userRecordHash: AgentPubKey;

$: hashes, loading, error, record, patientRecord;

onMount(async () => {
  if (userRecordHash === undefined) {
    throw new Error(`The userRecordHash input is required for the RecordsByUser element`);
  }
  console.log(`passed: userRecordHash ${userRecordHash}`);

  // await fetchPatientRecords();

  try {
    const links = await client.callZome({
      cap_secret: null,
      role_name: 'idsov',
      zome_name: 'patient_records',
      fn_name: 'get_records_for_recorder',
      payload: userRecordHash,
    });
    hashes = links.map(l => l.signed_action.hashed.hash);
    // hashes = links.map(r => r.signed_hash.hashed.hash);
    console.log(`call_zome:: get_records_for_recorder - ${JSON.stringify(hashes)}`);
    console.log(hashes.length);
    
    for (var i = 0; i < hashes.length; i++) {
      console.log(`call_individual_hash :: ${JSON.stringify(hashes[i])}`);
    }

  } catch (e) {
    error = e;
  }
  loading = false;

  client.on('signal', signal => {
    if (signal.zome_name !== 'patient_records') return;
    const payload = signal.payload as PatientRecordsSignal;
    if (payload.type !== 'EntryCreated') return;
    if (payload.app_entry.type !== 'PatientRecord') return;
    // hashes = [...hashes, payload.action.hashed.hash];
    hashes = [...hashes, payload.action.hashed.content.entry_hash];
    console.log(`hashes post client: ${hashes}`);
  });
});

// async function fetchPatientRecords() {
//   try {
//     const links = await client.callZome({
//       cap_secret: null,
//       role_name: 'idsov',
//       zome_name: 'patient_records',
//       fn_name: 'get_records_for_recorder',
//       payload: userRecordHash,
//     });
//     // hashes = links.map(l => l.target);
//     // hashes = links.map(r => r.signed_hash.hashed.hash);
//     // console.log(JSON.stringify(hashes));
//   } catch (e) {
//     error = e;
//   }
//   loading = false;
// }

async function fetchPatientRecord() {
  loading = true;
  error = undefined;
  record = undefined;
  patientRecord = undefined;
  
  try {
    record = await client.callZome({
      cap_secret: null,
      role_name: 'idsov',
      zome_name: 'patient_records',
      fn_name: 'get_latest_patient_record',
      payload: patientRecordHash,
    });
    if (record) {
      patientRecord = decode((record.entry as any).Present.entry) as PatientRecord;
    }
  } catch (e) {
    error = e;
  }

  loading = false;
}

</script>
<h2>List of Active Users</h2>
<list-profiles on:agent-selected={e => alert(e.detail.agentPubKey)}></list-profiles>
<br>
<h2>Records Created By You</h2>
<br>

{#if loading}
<div style="display: flex; flex: 1; align-items: center; justify-content: center">
  <mwc-circular-progress indeterminate></mwc-circular-progress>
</div>
{:else if error}
<span>Error fetching the patient records: {error.data.data}.</span>
{:else if hashes.length === 0}
<span>No patient records found.</span>
{:else}
  <div style="display: flex; flex-direction: column">
    <!-- row 1 -->
    {#each hashes.reverse() as hash}
      <div style="margin-bottom: 8px;">
        <PatientRecordListItem patientRecordHash={hash}></PatientRecordListItem>
        <!-- <PatientRecordListItem patientRecordHash={hash}  on:patient-record-deleted={() => fetchPatientRecords()}></PatientRecordListItem> -->

      </div>
    {/each}
  </div>
{/if}

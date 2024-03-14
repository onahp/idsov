<script lang="ts">
import { onMount, getContext } from 'svelte';
import '@material/mwc-circular-progress';
import type { EntryHash, Record, AgentPubKey, ActionHash, AppAgentClient, NewEntryAction } from '@holochain/client';
import { clientContext } from '../../contexts';
import PatientRecordListItem from './PatientRecordListItem.svelte';
import EditPatientRecord from './EditPatientRecord.svelte';
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

// let patients: Array<PatientRecord> | undefined;
let patients = [];
let hashes: Array<ActionHash> | undefined;
let loading = true;
let error: any = undefined;
let editing = false;

export let userRecordHash: AgentPubKey;

$: hashes, loading, error, record, patientRecord, patients, editing;

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
    // console.log(`call_zome:: get_records_for_recorder - ${JSON.stringify(hashes)}`);
    console.log(hashes.length);
  } catch (e) {
    error = e;
  }

  client.on('signal', signal => {
    if (signal.zome_name !== 'patient_records') return;
    const payload = signal.payload as PatientRecordsSignal;
    if (payload.type !== 'EntryCreated') return;
    if (payload.app_entry.type !== 'PatientRecord') return;
    // hashes = [...hashes, payload.action.hashed.hash];
    hashes = [...hashes, payload.action.hashed.content.entry_hash];
  });
  
  try {
    for (var i = 0; i < hashes.length; i++) {
      console.log(`[Records By User | call_individual_hash] : ${JSON.stringify(hashes[i])}`);
      let pRecord = await fetchPatientRecord(hashes[i]);
      pRecord.record_hash = hashes[i];
      patients?.push(pRecord);
    }
    console.log(`Records By User | Patient Records] : ${JSON.stringify(patients, null, 2)}`);
  } catch (e) {
    error = e
  }

  loading = false;

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

async function fetchPatientRecord(hash) {
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
      payload: hash,
      // payload: patientRecordHash,
    });
    if (record) {
      patientRecord = decode((record.entry as any).Present.entry) as PatientRecord;
      return patientRecord;
    }
  } catch (e) {
    error = e;
  }

  loading = false;
}
  
</script>


<div class="flex flex-col w-full border-opacity-50">
  <div class="grid card bg-base-100 rounded-box">
    <h1 class="text-1xl font-bold text-center">My Dashboard</h1>
    <!-- <h2>List of Active Users</h2> -->
    <!-- <list-profiles on:agent-selected={e => alert(e.detail.agentPubKey)}></list-profiles> -->
    <!-- <br> -->
    <!-- <h2>Records Created By You</h2> -->
    <br>

    {#if loading}
      <div style="display: flex; flex: 1; align-items: center; justify-content: center">
        <progress class="progress w-56"></progress>
      </div>
    {:else if error}
      <span>Error fetching the patient records: {error.data.data}.</span>
    {:else if patients.length === 0}
      <span>No patient records found.</span>
    {:else if editing}
      <EditPatientRecord
        originalPatientRecordHash={ patientRecordHash}
        currentRecord={record}
        on:patient-record-updated={async () => {
        editing = false;
        await fetchPatientRecord()
        } }
        on:edit-canceled={() => { editing = false; } }
        ></EditPatientRecord>
      {:else}
        <div style="display: flex; flex-direction: column">
          <!-- row 1 -->

          <div class="overflow-x-auto">
            <table class="table">
              <!-- head -->
              <thead>
                <tr>
                  <th>
                    <label>
                      <input type="checkbox" class="checkbox" />
                    </label>
                  </th>
                  <th>Name</th>
                  <th>Iwi</th>
                  <th>Resource Type</th>
                  <th></th>
                </tr>
              </thead>
              <tbody>
                <!-- {#each hashes.reverse() as hash} -->
                  {#each patients.reverse() as patient}
                    <!-- <div style="margin-bottom: 8px;"> -->
                  <!-- row 1 -->
                  <tr >
                    <th>
                      <label>
                        <input type="checkbox" class="checkbox" />
                      </label>
                    </th>
                    <td >
                      <div class="flex items-center gap-3">
                        <div class="avatar">
                          <div class="mask mask-squircle w-12 h-12">
                            <img src="https://icons.getbootstrap.com/assets/icons/person-circle.svg" alt="Avatar" />
                          </div>
                        </div>
                        <div>
                          <div class="font-bold">{patient.whanau}, {patient.ingoa}</div>
                          <div class="text-sm opacity-50">{patient.no_hea_koe}</div>
                        </div>
                      </div>
                    </td>
                    <td>
                      {patient.maunga}
                      <br/>
                      <span class="badge badge-ghost badge-sm">{patient.moana}</span>
                    </td>
                    <td>{patient.resource_type}</td>
                    <th>
                      <button class="btn btn-ghost btn-xs" on:click={() => { patientRecordHash = patient.record_hash; editing = true; }}>Details</button>
                    </th>
                  </tr>


                  <!-- <PatientRecordListItem patientRecordHash={hash}></PatientRecordListItem> -->
                  <!-- <PatientRecordListItem patientRecordHash={hash}  on:patient-record-deleted={() => fetchPatientRecords()}></PatientRecordListItem> -->

                  <!-- </div> -->
                {/each}
              </tbody>
            </table>
          </div>
        </div>
      {/if}
    </div>
</div>

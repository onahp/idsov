<script lang="ts">
import { onMount, getContext } from 'svelte';
import '@material/mwc-circular-progress';
import type { EntryHash, Record, AgentPubKey, ActionHash, AppAgentClient, NewEntryAction } from '@holochain/client';
import { clientContext } from '../../contexts';
import PatientRecordListItem from './PatientRecordListItem.svelte';
import DashboardPatientRecordListItem from './DashboardPatientRecordListItem.svelte';
import type { PatientRecordsSignal } from './types';
import { decode } from '@msgpack/msgpack';
import type { PatientRecord } from './types';

let client: AppAgentClient = (getContext(clientContext) as any).getClient();

let hashes: Array<ActionHash> | undefined;
let loading = true;
let error: any = undefined;
let record: Record | undefined;
let patientRecord: PatientRecord | undefined;
let patients = [];
let result = null;

// $: hashes, loading, error;
$: hashes, loading, error, record, patientRecord, patients, result;

onMount(async () => {
  
  try {
    const links = await client.callZome({
      cap_secret: null,
      role_name: 'idsov',
      zome_name: 'patient_records',
      fn_name: 'get_all_records',
      payload: null,
    });
    hashes = links.map(l => l.target);
    // hashes = links.map(l => l.signed_action.hashed.hash);
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
      console.log(`call_individual_hash :: ${JSON.stringify(hashes[i])}`);
      let pRecord = await fetchPatientRecord(hashes[i]);
      // let pRecord = hashes[i];
      patients?.push(pRecord);
    }
    console.log(`[All Records | Patient Records] : ${JSON.stringify(patients, null, 2)}`);
  } catch (e) {
    error = e
  }

  loading = false;
  
  // dummy fetch data - success
  // const response = await fetch("https://jsonplaceholder.typicode.com/posts");
  // const data = await response.json();
  // console.log(data);
  
  // dummy post data - success
  // doPost(patients);

});

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

async function doPost (data) {
  const res = await fetch('https://httpbin.org/post', {
    method: 'POST',
    body: JSON.stringify({
      data,
    })
  });
    
  const json = await res.json()
  result = JSON.stringify(json)
  // console.log(`POST Success :: ${result}`);
}


// async function fetchPatientRecords() {
//   try {
//    const links = await client.callZome({
//      cap_secret: null,
//      role_name: 'idsov',
//      zome_name: 'patient_records',
//      fn_name: 'get_all_records',
//      payload: null,
//    });
//    // hashes = links.map(l => l.target);
//    hashes = links.map(l => l.signed_action.hashed.hash);
//  } catch (e) {
//    error = e;
//  }
//  loading = false;
// }

</script>

<div class="flex flex-col w-full border-opacity-50">
  <div class="grid card bg-base-100 rounded-box">
    <h1 class="text-1xl font-bold text-center">Shared Dashboard</h1>
    <br>
    {#if loading}
      <div style="display: flex; flex: 1; align-items: center; justify-content: center">
        <progress class="progress w-56"></progress>
      </div>
    {:else if error}
      <span>Error fetching the patient records: {error.data.data}.</span>
    {:else if patients.length === 0}
      <span>No patient records found.</span>
      {:else}
        <div style="display: flex; flex-direction: column">
          <!-- row 1 -->

          <div class="overflow-x-auto">
            <table class="table">
              <!-- head -->
              <thead>
                <tr>
                  <!-- <th> -->
                  <!--   <label> -->
                  <!--     <input type="checkbox" class="checkbox" /> -->
                  <!--   </label> -->
                  <!-- </th> -->
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
                    <!-- <th> -->
                    <!--   <label> -->
                    <!--     <input type="checkbox" class="checkbox" /> -->
                    <!--   </label> -->
                    <!-- </th> -->
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
                      <button class="btn btn-ghost btn-xs" on:click={() => { editing = true; }}>Details</button>
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

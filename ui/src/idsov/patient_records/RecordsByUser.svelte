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
import { encodeHashToBase64 } from "@holochain/client";

let client: AppAgentClient = (getContext(clientContext) as any).getClient();

let hashes: Array<ActionHash> | undefined;
let loading = true;
let error: any = undefined;

export let userRecordHash: AgentPubKey;

$: hashes, loading, error;

onMount(async () => {
  if (userRecordHash === undefined) {
    throw new Error(`The userRecordHash input is required for the RecordsByUser element`);
  }
  await fetchPatientRecords();
  client.on('signal', signal => {
    if (signal.zome_name !== 'patient_records') return;
    const payload = signal.payload as PatientRecordsSignal;
    if (payload.type !== 'EntryCreated') return;
    if (payload.app_entry.type !== 'PatientRecord') return;
    hashes = [...hashes, payload.action.hashed.hash];
  });
});

async function fetchPatientRecords() {
  try {
    const links = await client.callZome({
      cap_secret: null,
      role_name: 'idsov',
      zome_name: 'patient_records',
      fn_name: 'get_records_by_recorder',
      payload: userRecordHash,
    });
    hashes = links.map(l => l.target);
  } catch (e) {
    error = e;
  }
  loading = false;
}

</script>
<h2>List of Active Users</h2>
<list-profiles on:agent-selected={e => alert(e.detail.agentPubKey)}></list-profiles>
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
  <!-- {#each hashes as hash} -->
  <!--   <div style="margin-bottom: 8px;"> -->
  <!--     <PatientRecordListItem patientRecordHash={hash}  on:patient-record-deleted={() => fetchPatientRecords()}></PatientRecordListItem> -->
  <!--   </div> -->
  <!-- {/each} -->

  {#each hashes.reverse() as hash}
    <div style="margin-bottom: 8px;">
      <span on:click={() => navigate('dashboard', hash)}>
        <!-- <DeliberationDetail deliberationHash={hash}></DeliberationDetail> -->
        <!-- <PatientRecordListItem deliberationHash={hash}></DeliberationListItem> -->
        <PatientRecordListItem userRecordHash={hash} on:patient-record-deleted={() => fetchPatientRecords()}></PatientRecordListItem>
      </span>
    </div>
  {/each} 


</div>
{/if}


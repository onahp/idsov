<script lang="ts">
import { createEventDispatcher, onMount, getContext } from 'svelte';
import '@material/mwc-circular-progress';
import { decode } from '@msgpack/msgpack';
import type { Record, ActionHash, AppAgentClient, EntryHash, AgentPubKey, DnaHash } from '@holochain/client';
import { clientContext } from '../../contexts';
import type { PatientRecord } from './types';
import '@material/mwc-circular-progress';
import type { Snackbar } from '@material/mwc-snackbar';
import '@material/mwc-snackbar';
import '@material/mwc-icon-button';
import EditPatientRecord from './EditPatientRecord.svelte'; 
import CreateComment from './CreateComment.svelte';
import CommentsForPatientRecord from './CommentsForPatientRecord.svelte';

const dispatch = createEventDispatcher();

export let patientRecordHash: ActionHash;

let client: AppAgentClient = (getContext(clientContext) as any).getClient();
let loading = true;
let error: any = undefined;
let record: Record | undefined;
let patientRecord: PatientRecord | undefined;
let editing = false;
let errorSnackbar: Snackbar;
  
$: editing,  error, loading, record, patientRecord;

onMount(async () => {
  if (patientRecordHash === undefined) {
    throw new Error(`The patientRecordHash input is required for the PatientRecordDetail element`);
  }
  await fetchPatientRecord();
});

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

async function deletePatientRecord() {
  try {
    await client.callZome({
      cap_secret: null,
      role_name: 'idsov',
      zome_name: 'patient_records',
      fn_name: 'delete_patient_record',
      payload: patientRecordHash,
    });
    dispatch('patient-record-deleted', { patientRecordHash: patientRecordHash });
  } catch (e: any) {
    errorSnackbar.labelText = `Error deleting the patient record: ${e.data.data}`;
    errorSnackbar.show();
  }
}
</script>

<mwc-snackbar bind:this={errorSnackbar} leading>
</mwc-snackbar>

{#if loading}
<div style="display: flex; flex: 1; align-items: center; justify-content: center">
  <mwc-circular-progress indeterminate></mwc-circular-progress>
</div>
{:else if error}
<span>Error fetching the patient record: {error.data.data}</span>
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
  <div style="display: flex; flex-direction: row">
    <span style="flex: 1"></span>
    <mwc-icon-button style="margin-left: 8px" icon="edit" on:click={() => { editing = true; } }></mwc-icon-button>
    <mwc-icon-button style="margin-left: 8px" icon="delete" on:click={() => deletePatientRecord()}></mwc-icon-button>
  </div>

  <div style="display: flex; flex-direction: row; margin-bottom: 16px">
    <span style="margin-right: 4px"><strong>Content:</strong></span>
    <span style="white-space: pre-line">{ patientRecord.content }</span>
  </div>

  <div style="display: flex; flex-direction: row; margin-bottom: 16px">
    <span style="margin-right: 4px"><strong>Resource Type:</strong></span>
    <span style="white-space: pre-line">{ patientRecord.resource_type }</span>
  </div>

  <div style="display: flex; flex-direction: row; margin-bottom: 16px">
    <span style="margin-right: 4px"><strong>Date Visited:</strong></span>
    <span style="white-space: pre-line">{ new Date(patientRecord.date_visited / 1000).toLocaleString() }</span>
  </div>

  <CreateComment patientRecordHash="{patientRecordHash}"></CreateComment>
  <CommentsForPatientRecord patientRecordHash="{patientRecordHash}"></CommentsForPatientRecord>
</div>
{/if}


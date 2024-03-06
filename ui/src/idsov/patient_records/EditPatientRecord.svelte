<script lang="ts">
import { createEventDispatcher, getContext, onMount } from 'svelte';
import type { AppAgentClient, Record, EntryHash, AgentPubKey, DnaHash, ActionHash } from '@holochain/client';
import { decode } from '@msgpack/msgpack';
import { clientContext } from '../../contexts';
import type { PatientRecord } from './types';
import '@material/mwc-button';
import '@material/mwc-snackbar';
import type { Snackbar } from '@material/mwc-snackbar';
import '@vaadin/date-time-picker/theme/material/vaadin-date-time-picker.js';

import '@material/mwc-textfield';
import '@material/mwc-textarea';
let client: AppAgentClient = (getContext(clientContext) as any).getClient();

const dispatch = createEventDispatcher();

export let originalPatientRecordHash!: ActionHash;

export let currentRecord!: Record;
let currentPatientRecord: PatientRecord = decode((currentRecord.entry as any).Present.entry) as PatientRecord;

let content: string | undefined = currentPatientRecord.content;
let resourceType: string | undefined = currentPatientRecord.resource_type;
let dateVisited: number | undefined = currentPatientRecord.date_visited;

let errorSnackbar: Snackbar;

$: content, resourceType, dateVisited;
$: isPatientRecordValid = true && content !== '' && resourceType !== '' && true;

onMount(() => {
  if (currentRecord === undefined) {
    throw new Error(`The currentRecord input is required for the EditPatientRecord element`);
  }
  if (originalPatientRecordHash === undefined) {
    throw new Error(`The originalPatientRecordHash input is required for the EditPatientRecord element`);
  }
});

async function updatePatientRecord() {

  const patientRecord: PatientRecord = {
    content: content!,
    resource_type: resourceType!,
    date_visited: dateVisited!,
  };

  try {
    const updateRecord: Record = await client.callZome({
      cap_secret: null,
      role_name: 'idsov',
      zome_name: 'patient_records',
      fn_name: 'update_patient_record',
      payload: {
        original_patient_record_hash: originalPatientRecordHash,
        previous_patient_record_hash: currentRecord.signed_action.hashed.hash,
        updated_patient_record: patientRecord
      }
    });

    dispatch('patient-record-updated', { actionHash: updateRecord.signed_action.hashed.hash });
  } catch (e) {
    errorSnackbar.labelText = `Error updating the patient record: ${e.data.data}`;
    errorSnackbar.show();
  }
}

</script>
<mwc-snackbar bind:this={errorSnackbar} leading>
</mwc-snackbar>
<div style="display: flex; flex-direction: column">
  <!-- <span style="font-size: 18px">Modify Patient Record</span> -->
  <mwc-textarea outlined label="Content" value={ content } on:input={e => { content = e.target.value;} } required></mwc-textarea>
  <br>
  <mwc-textfield outlined label="Resource Type" value={ resourceType } on:input={e => { resourceType = e.target.value; } } required></mwc-textfield>
  <br>
  <vaadin-date-time-picker label="Date Visited" value={new Date(dateVisited / 1000).toISOString()} on:change={e => { dateVisited = new Date(e.target.value).valueOf() * 1000;} } required></vaadin-date-time-picker>
  <!-- <div style="margin-bottom: 16px"> -->
  <!--   <mwc-textarea outlined label="Content" value={ content } on:input={e => { content = e.target.value;} } required></mwc-textarea> -->
  <!-- </div> -->

  <!-- <div style="margin-bottom: 16px"> -->
  <!--   <mwc-textfield outlined label="Resource Type" value={ resourceType } on:input={e => { resourceType = e.target.value; } } required></mwc-textfield> -->
  <!-- </div> -->

  <!-- <div style="margin-bottom: 16px"> -->
  <!--   <vaadin-date-time-picker label="Date Visited" value={new Date(dateVisited / 1000).toISOString()} on:change={e => { dateVisited = new Date(e.target.value).valueOf() * 1000;} } required></vaadin-date-time-picker> -->
  <!-- </div> -->

  <div style="display: flex; flex-direction: row">
    <mwc-button
      outlined
      label="Cancel"
      on:click={() => dispatch('edit-canceled')}
      style="flex: 1; margin-right: 16px"
    ></mwc-button>
    <mwc-button
      raised
      label="Save"
      disabled={!isPatientRecordValid}
      on:click={() => updatePatientRecord()}
      style="flex: 1;"
    ></mwc-button>
  </div>
</div>

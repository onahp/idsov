<script lang="ts">
import { createEventDispatcher, getContext, onMount } from 'svelte';
import type { AppAgentClient, Record, EntryHash, AgentPubKey, ActionHash, DnaHash } from '@holochain/client';
import { clientContext } from '../../contexts';
import type { PatientRecord } from './types';
import '@material/mwc-button';
import '@material/mwc-snackbar';
import type { Snackbar } from '@material/mwc-snackbar';
import '@material/mwc-textarea';
import '@vaadin/date-time-picker/theme/material/vaadin-date-time-picker.js';
import '@material/mwc-textfield';

let client: AppAgentClient = (getContext(clientContext) as any).getClient();

const dispatch = createEventDispatcher();


let content: string = '';
let resourceType: string = '';
let dateVisited: number = Date.now();

let errorSnackbar: Snackbar;

$: content, resourceType, dateVisited;
$: isPatientRecordValid = true && content !== '' && resourceType !== '' && true;

onMount(() => {
});

async function createPatientRecord() {  
  const patientRecordEntry: PatientRecord = { 
    content: content!,
    resource_type: resourceType!,
    date_visited: dateVisited!,
  };
  
  try {
    const record: Record = await client.callZome({
      cap_secret: null,
      role_name: 'idsov',
      zome_name: 'patient_records',
      fn_name: 'create_patient_record',
      payload: patientRecordEntry,
    });
    dispatch('patient-record-created', { patientRecordHash: record.signed_action.hashed.hash });
  } catch (e) {
    errorSnackbar.labelText = `Error creating the patient record: ${e.data.data}`;
    errorSnackbar.show();
  }
}

</script>
<mwc-snackbar bind:this={errorSnackbar} leading>
</mwc-snackbar>
<div style="display: flex; flex-direction: column">
  <span style="font-size: 18px">Create Patient Record</span>
  

  <div style="margin-bottom: 16px">
    <mwc-textarea outlined label="Content" value={ content } on:input={e => { content = e.target.value;} } required></mwc-textarea>          
  </div>
            
  <div style="margin-bottom: 16px">
    <mwc-textfield outlined label="Resource Type" value={ resourceType } on:input={e => { resourceType = e.target.value; } } required></mwc-textfield>          
  </div>
            
  <div style="margin-bottom: 16px">
    <vaadin-date-time-picker label="Date Visited" value={new Date(dateVisited / 1000).toISOString()} on:change={e => { dateVisited = new Date(e.target.value).valueOf() * 1000;} } required></vaadin-date-time-picker>          
  </div>
            

  <mwc-button 
    raised
    label="Create Patient Record"
    disabled={!isPatientRecordValid}
    on:click={() => createPatientRecord()}
  ></mwc-button>
</div>

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
  let whanau: string | undefined = currentPatientRecord.whanau;
  let ingoa: string | undefined = currentPatientRecord.ingoa;
  let noHeaKoe: string | undefined = currentPatientRecord.no_hea_koe;
  let maunga: string | undefined = currentPatientRecord.maunga;
  let moana: string | undefined = currentPatientRecord.moana;

  let errorSnackbar: Snackbar;

  $: content, resourceType, dateVisited, whanau, ingoa, noHeaKoe, maunga, moana;
  // $: isPatientRecordValid = true && content !== '' && resourceType !== '' && true;
  $: isPatientRecordValid = true && 
  content !== '' && 
  resourceType !== '' && 
  whanau !== '' && 
  ingoa !== '' &&
  noHeaKoe !== '' && 
  maunga !== '' &&
  moana !== '' &&
  true;

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
      whanau: whanau!,
      ingoa: ingoa!,
      no_hea_koe: noHeaKoe!,
      maunga: maunga!,
      moana: moana!,
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
      errorSnackbar.labelText = `Error updating the patient record: ${e?.data?.data}`;
      errorSnackbar.show();
    }
  }

</script>

<mwc-snackbar bind:this={errorSnackbar} leading>
</mwc-snackbar>

<div class="flex flex-col w-full border-opacity-50">
  <div class="grid card bg-base-100 rounded-box place-items-center">

    <label class="input input-bordered flex items-center gap-2 w-full bg-base-200">
      <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-journal-text" viewBox="0 0 16 16"><path d="M5 10.5a.5.5 0 0 1 .5-.5h2a.5.5 0 0 1 0 1h-2a.5.5 0 0 1-.5-.5m0-2a.5.5 0 0 1 .5-.5h5a.5.5 0 0 1 0 1h-5a.5.5 0 0 1-.5-.5m0-2a.5.5 0 0 1 .5-.5h5a.5.5 0 0 1 0 1h-5a.5.5 0 0 1-.5-.5m0-2a.5.5 0 0 1 .5-.5h5a.5.5 0 0 1 0 1h-5a.5.5 0 0 1-.5-.5"/><path d="M3 0h10a2 2 0 0 1 2 2v12a2 2 0 0 1-2 2H3a2 2 0 0 1-2-2v-1h1v1a1 1 0 0 0 1 1h10a1 1 0 0 0 1-1V2a1 1 0 0 0-1-1H3a1 1 0 0 0-1 1v1H1V2a2 2 0 0 1 2-2"/><path d="M1 5v-.5a.5.5 0 0 1 1 0V5h.5a.5.5 0 0 1 0 1h-2a.5.5 0 0 1 0-1zm0 3v-.5a.5.5 0 0 1 1 0V8h.5a.5.5 0 0 1 0 1h-2a.5.5 0 0 1 0-1zm0 3v-.5a.5.5 0 0 1 1 0v.5h.5a.5.5 0 0 1 0 1h-2a.5.5 0 0 1 0-1z"/>
      </svg>
      <input type="text" class="grow" placeholder="Medical Note" value={ content } on:input={e => { content = e?.target?.value;} } required/>
      <span class="badge badge-warning">Required</span>
    </label>
    <br>
    <label class="input input-bordered flex items-center gap-2 w-full bg-base-200">
      <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-calendar2-event" viewBox="0 0 16 16"><path d="M11 7.5a.5.5 0 0 1 .5-.5h1a.5.5 0 0 1 .5.5v1a.5.5 0 0 1-.5.5h-1a.5.5 0 0 1-.5-.5z"/><path d="M3.5 0a.5.5 0 0 1 .5.5V1h8V.5a.5.5 0 0 1 1 0V1h1a2 2 0 0 1 2 2v11a2 2 0 0 1-2 2H2a2 2 0 0 1-2-2V3a2 2 0 0 1 2-2h1V.5a.5.5 0 0 1 .5-.5M2 2a1 1 0 0 0-1 1v11a1 1 0 0 0 1 1h12a1 1 0 0 0 1-1V3a1 1 0 0 0-1-1z"/><path d="M2.5 4a.5.5 0 0 1 .5-.5h10a.5.5 0 0 1 .5.5v1a.5.5 0 0 1-.5.5H3a.5.5 0 0 1-.5-.5z"/>
      </svg>
      <input type="text" class="grow" placeholder="Type of Visit" value={ resourceType } on:input={e => { resourceType = e?.target?.value; } } required/>
      <span class="badge badge-warning">Required</span>
    </label>
    <br>
    <!-- <vaadin-date-time-picker label="Date Visited" value={new Date(dateVisited / 1000).toISOString()} on:change={e => { dateVisited = new Date(e.target.value).valueOf() * 1000;} } required></vaadin-date-time-picker> -->
  </div>
  <div style="display: flex; flex-direction: row">
    <button
      class="btn btn-error btn-outline"
      on:click={() => dispatch('edit-canceled')}
      style="flex: 1; margin-right: 16px"
      >Cancel</button>
    <button
      class="btn btn-success btn-outline"
      disabled={!isPatientRecordValid}
      on:click={() => updatePatientRecord()}
      style="flex: 1;"
      >Save</button>
  </div>
</div>

<!-- <div style="display: flex; flex-direction: column"> -->
  <!--   <\!-- <span style="font-size: 18px">Modify Patient Record</span> -\-> -->
    <!--   <mwc-textarea outlined label="Content" value={ content } on:input={e => { content = e.target.value;} } required></mwc-textarea> -->
    <!--   <br> -->
    <!--   <mwc-textfield outlined label="Resource Type" value={ resourceType } on:input={e => { resourceType = e.target.value; } } required></mwc-textfield> -->
    <!--   <br> -->
    <!--   <vaadin-date-time-picker label="Date Visited" value={new Date(dateVisited / 1000).toISOString()} on:change={e => { dateVisited = new Date(e.target.value).valueOf() * 1000;} } required></vaadin-date-time-picker> -->
    <!--   <\!-- <div style="margin-bottom: 16px"> -\-> -->
      <!--   <\!--   <mwc-textarea outlined label="Content" value={ content } on:input={e => { content = e.target.value;} } required></mwc-textarea> -\-> -->
<!--   <\!-- </div> -\-> -->

  <!--   <\!-- <div style="margin-bottom: 16px"> -\-> -->
    <!--   <\!--   <mwc-textfield outlined label="Resource Type" value={ resourceType } on:input={e => { resourceType = e.target.value; } } required></mwc-textfield> -\-> -->
<!--   <\!-- </div> -\-> -->

  <!--   <\!-- <div style="margin-bottom: 16px"> -\-> -->
    <!--   <\!--   <vaadin-date-time-picker label="Date Visited" value={new Date(dateVisited / 1000).toISOString()} on:change={e => { dateVisited = new Date(e.target.value).valueOf() * 1000;} } required></vaadin-date-time-picker> -\-> -->
<!--   <\!-- </div> -\-> -->

  <!--   <div style="display: flex; flex-direction: row"> -->
    <!--     <button -->
    <!--       class="btn btn-error btn-outline" -->
    <!--       outlined -->
    <!--       label="Cancel" -->
    <!--       on:click={() => dispatch('edit-canceled')} -->
      <!--       style="flex: 1; margin-right: 16px" -->
      <!--     >Cancel</button> -->
    <!--     <button -->
    <!--       class="btn btn-success btn-outline" -->
    <!--       raised -->
    <!--       label="Save" -->
    <!--       disabled={!isPatientRecordValid} -->
    <!--       on:click={() => updatePatientRecord()} -->
      <!--       style="flex: 1;" -->
      <!--     >Save</button> -->
    <!--   </div> -->
  <!-- </div> -->

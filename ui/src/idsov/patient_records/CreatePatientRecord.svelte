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
  
// specific imports
  import Svelecte from 'svelecte';
  import maungaList from './data/maungaList';
  import moanaList from './data/moanaList';
  import noHeaKoeList from './data/noHeaKoeList';
  import { navigate } from '../../store';

  let client: AppAgentClient = (getContext(clientContext) as any).getClient();

  const dispatch = createEventDispatcher();

  let content: string = '';
  let resourceType: string = '';
  let dateVisited: number = Date.now();
  let whanau: string = '';
  let ingoa: string = '';
  let noHeaKoe: string = '';
  let maunga: string = '';
  let moana: string = '';

  let errorSnackbar: Snackbar;

  $: content, resourceType, dateVisited, whanau, ingoa, noHeaKoe, maunga, moana;
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
  });

  async function createPatientRecord() {  
    const patientRecordEntry: PatientRecord = { 
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
      const record: Record = await client.callZome({
        cap_secret: null,
        role_name: 'idsov',
        zome_name: 'patient_records',
        fn_name: 'create_patient_record',
        payload: patientRecordEntry,
      });
      dispatch('patient-record-created', { patientRecordHash: record.signed_action.hashed.hash });

      console.log(`base_recorder: ${client.myPubKey}`);
      console.log(`target_record_hash: ${record.signed_action.hashed.hash}`);
      
      // join deliberation
      await client.callZome({
        cap_secret: null,
        role_name: 'idsov',
        zome_name: 'patient_records',
        fn_name: 'add_records_for_recorder',
        payload: {
          base_recorder: client.myPubKey,
          target_record_hash: record.signed_action.hashed.hash
        },
      });

    } catch (e) {
      console.log(`Error: add_records_for_recorder :: ${JSON.stringify(e)}`);
      errorSnackbar.labelText = `Error creating the patient record: ${e.data.data}`;
      errorSnackbar.show();
    }
  }

</script>
<mwc-snackbar bind:this={errorSnackbar} leading>
</mwc-snackbar>


<div class="flex flex-col w-full border-opacity-50">
  <div class="grid card bg-base-100 rounded-box place-items-center">
    <h1 class="text-1xl font-bold">New Patient Record</h1>
    <br>
    <div class="flex w-full">
      
      <!-- Whanau (family name) -->
      <label class="input input-bordered flex items-center gap-2 w-full bg-base-200">
        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-people" viewBox="0 0 16 16"><path d="M15 14s1 0 1-1-1-4-5-4-5 3-5 4 1 1 1 1zm-7.978-1L7 12.996c.001-.264.167-1.03.76-1.72C8.312 10.629 9.282 10 11 10c1.717 0 2.687.63 3.24 1.276.593.69.758 1.457.76 1.72l-.008.002-.014.002zM11 7a2 2 0 1 0 0-4 2 2 0 0 0 0 4m3-2a3 3 0 1 1-6 0 3 3 0 0 1 6 0M6.936 9.28a6 6 0 0 0-1.23-.247A7 7 0 0 0 5 9c-4 0-5 3-5 4q0 1 1 1h4.216A2.24 2.24 0 0 1 5 13c0-1.01.377-2.042 1.09-2.904.243-.294.526-.569.846-.816M4.92 10A5.5 5.5 0 0 0 4 13H1c0-.26.164-1.03.76-1.724.545-.636 1.492-1.256 3.16-1.275ZM1.5 5.5a3 3 0 1 1 6 0 3 3 0 0 1-6 0m3-2a2 2 0 1 0 0 4 2 2 0 0 0 0-4"/></svg>
        <input type="text" class="grow" placeholder="Whānau" value={ whanau } on:input={e => { whanau = e.target.value; } } required/>
        <span class="badge badge-warning">Required</span>
      </label>
      <div class="divider divider-horizontal"></div>
      
      <!-- Ingoa (first name) -->
      <label class="input input-bordered flex items-center gap-2 w-full bg-base-200">
        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-person" viewBox="0 0 16 16"><path d="M8 8a3 3 0 1 0 0-6 3 3 0 0 0 0 6m2-3a2 2 0 1 1-4 0 2 2 0 0 1 4 0m4 8c0 1-1 1-1 1H3s-1 0-1-1 1-4 6-4 6 3 6 4m-1-.004c-.001-.246-.154-.986-.832-1.664C11.516 10.68 10.289 10 8 10s-3.516.68-4.168 1.332c-.678.678-.83 1.418-.832 1.664z"/></svg>
        <input type="text" class="grow" placeholder="Ingoa" value={ ingoa } on:input={e => { ingoa = e.target.value; } } required/>
        <span class="badge badge-warning">Required</span>
      </label>
    </div>

    <!-- Medical Notes -->
    <br>
    <label class="textarea textarea-bordered flex items-center gap-2 w-full bg-base-200">
      <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-journal-text" viewBox="0 0 16 16"><path d="M5 10.5a.5.5 0 0 1 .5-.5h2a.5.5 0 0 1 0 1h-2a.5.5 0 0 1-.5-.5m0-2a.5.5 0 0 1 .5-.5h5a.5.5 0 0 1 0 1h-5a.5.5 0 0 1-.5-.5m0-2a.5.5 0 0 1 .5-.5h5a.5.5 0 0 1 0 1h-5a.5.5 0 0 1-.5-.5m0-2a.5.5 0 0 1 .5-.5h5a.5.5 0 0 1 0 1h-5a.5.5 0 0 1-.5-.5"/><path d="M3 0h10a2 2 0 0 1 2 2v12a2 2 0 0 1-2 2H3a2 2 0 0 1-2-2v-1h1v1a1 1 0 0 0 1 1h10a1 1 0 0 0 1-1V2a1 1 0 0 0-1-1H3a1 1 0 0 0-1 1v1H1V2a2 2 0 0 1 2-2"/><path d="M1 5v-.5a.5.5 0 0 1 1 0V5h.5a.5.5 0 0 1 0 1h-2a.5.5 0 0 1 0-1zm0 3v-.5a.5.5 0 0 1 1 0V8h.5a.5.5 0 0 1 0 1h-2a.5.5 0 0 1 0-1zm0 3v-.5a.5.5 0 0 1 1 0v.5h.5a.5.5 0 0 1 0 1h-2a.5.5 0 0 1 0-1z"/>
      </svg>
      <textarea class="grow h-40 bg-base-200" placeholder="Medical Note" value={ content } on:input={e => { content = e.target.value;} } required></textarea>
      <!-- <input type="text" class="grow" placeholder="Medical Note" value={ content } on:input={e => { content = e.target.value;} } required/> -->
      <span class="badge badge-warning">Required</span>
    </label>

    <!-- Type of Visit -->
    <br>
    <label class="input input-bordered flex items-center gap-2 w-full bg-base-200">
      <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-calendar2-event" viewBox="0 0 16 16"><path d="M11 7.5a.5.5 0 0 1 .5-.5h1a.5.5 0 0 1 .5.5v1a.5.5 0 0 1-.5.5h-1a.5.5 0 0 1-.5-.5z"/><path d="M3.5 0a.5.5 0 0 1 .5.5V1h8V.5a.5.5 0 0 1 1 0V1h1a2 2 0 0 1 2 2v11a2 2 0 0 1-2 2H2a2 2 0 0 1-2-2V3a2 2 0 0 1 2-2h1V.5a.5.5 0 0 1 .5-.5M2 2a1 1 0 0 0-1 1v11a1 1 0 0 0 1 1h12a1 1 0 0 0 1-1V3a1 1 0 0 0-1-1z"/><path d="M2.5 4a.5.5 0 0 1 .5-.5h10a.5.5 0 0 1 .5.5v1a.5.5 0 0 1-.5.5H3a.5.5 0 0 1-.5-.5z"/>
      </svg>
      <input type="text" class="grow" placeholder="Type of Visit" value={ resourceType } on:input={e => { resourceType = e.target.value; } } required/>
      <span class="badge badge-warning">Required</span>
    </label>
    
    <!-- Maunga (Mountain) -->
    <br>
    <div class="flex w-full">

      <!-- <Svelecte {maungaList} value = { maunga } bind:value={ value } placeholder="Maunga" on:input={e => { maunga = e.target.value; } }></Svelecte> -->

      <label class="input input-bordered flex items-center gap-2 w-full bg-base-200">
        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-geo-alt" viewBox="0 0 16 16"><path d="M12.166 8.94c-.524 1.062-1.234 2.12-1.96 3.07A32 32 0 0 1 8 14.58a32 32 0 0 1-2.206-2.57c-.726-.95-1.436-2.008-1.96-3.07C3.304 7.867 3 6.862 3 6a5 5 0 0 1 10 0c0 .862-.305 1.867-.834 2.94M8 16s6-5.686 6-10A6 6 0 0 0 2 6c0 4.314 6 10 6 10"/><path d="M8 8a2 2 0 1 1 0-4 2 2 0 0 1 0 4m0 1a3 3 0 1 0 0-6 3 3 0 0 0 0 6"/></svg>       
        <!-- <input type="text" class="grow" placeholder="Maunga" value={ maunga } on:input={e => { maunga = e.target.value; } } required/> -->
        <Svelecte options={maungaList} bind:value={maunga} placeholder="Maunga" ></Svelecte>
        <span class="badge badge-warning">Required</span>
      </label>
      <div class="divider divider-horizontal"></div>
      
      <!-- Moana (Water) -->
      <label class="input input-bordered flex items-center gap-2 w-full bg-base-200">
        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-droplet" viewBox="0 0 16 16"><path fill-rule="evenodd" d="M7.21.8C7.69.295 8 0 8 0q.164.544.371 1.038c.812 1.946 2.073 3.35 3.197 4.6C12.878 7.096 14 8.345 14 10a6 6 0 0 1-12 0C2 6.668 5.58 2.517 7.21.8m.413 1.021A31 31 0 0 0 5.794 3.99c-.726.95-1.436 2.008-1.96 3.07C3.304 8.133 3 9.138 3 10a5 5 0 0 0 10 0c0-1.201-.796-2.157-2.181-3.7l-.03-.032C9.75 5.11 8.5 3.72 7.623 1.82z"/><path fill-rule="evenodd" d="M4.553 7.776c.82-1.641 1.717-2.753 2.093-3.13l.708.708c-.29.29-1.128 1.311-1.907 2.87z"/></svg>       
        <!-- <input type="text" class="grow" placeholder="Moana" value={ moana } on:input={e => { moana = e.target.value; } } required/> -->
        <Svelecte options={moanaList} bind:value={moana} placeholder="Moana" ></Svelecte>
        <span class="badge badge-warning">Required</span>
      </label>
      <div class="divider divider-horizontal"></div>
      
      <!-- No Hea Koe (Where are you from) -->
      <label class="input input-bordered flex items-center gap-2 w-full bg-base-200">
        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-signpost-split" viewBox="0 0 16 16"><path d="M7 7V1.414a1 1 0 0 1 2 0V2h5a1 1 0 0 1 .8.4l.975 1.3a.5.5 0 0 1 0 .6L14.8 5.6a1 1 0 0 1-.8.4H9v10H7v-5H2a1 1 0 0 1-.8-.4L.225 9.3a.5.5 0 0 1 0-.6L1.2 7.4A1 1 0 0 1 2 7zm1 3V8H2l-.75 1L2 10zm0-5h6l.75-1L14 3H8z"/> </svg>
        <!-- <input type="text" class="grow" placeholder="Nō Hea Koe" value={ noHeaKoe } on:input={e => { noHeaKoe = e.target.value; } } required/> -->
        <Svelecte options={noHeaKoeList} bind:value={noHeaKoe} placeholder="Nō Hea Koe" ></Svelecte>
        <span class="badge badge-warning">Required</span>
      </label>
    </div>
    <vaadin-date-time-picker label="Date Visited" value={new Date(dateVisited / 1000).toISOString()} on:change={e => { dateVisited = new Date(e.target.value).valueOf() * 1000;} } required></vaadin-date-time-picker>
  </div>
  <div style="display: flex; flex-direction: row">
    <button
      class="btn btn-accent btn-outline"
      label="Create Patient Record"
      disabled={!isPatientRecordValid}
      on:click={() => createPatientRecord()}
      style="flex: 1;"
      >Create Patient Record</button>
  </div>
</div>

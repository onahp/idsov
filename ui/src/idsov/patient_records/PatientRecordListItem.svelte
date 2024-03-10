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
  import SvgIcon from '../../SvgIcon.svelte';
  import AllRecords from './AllRecords.svelte';

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


<div class="flex flex-col w-full border-opacity-50">
  <div class="grid card bg-base-100 rounded-box place-items-center">
    <h1 class="text-1xl font-bold">My Dashboard</h1>
    {#if loading}
      <div style="display: flex; flex: 1; align-items: center; justify-content: center">
        <progress class="progress w-56"></progress>
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
        <!-- row 1 -->
        <tr>
          <th>
            <label>
              <input type="checkbox" class="checkbox" />
            </label>
          </th>
          <td>
            <div class="flex items-center gap-3">
              <div class="avatar">
                <div class="mask mask-squircle w-12 h-12">
                  <img src="/tailwind-css-component-profile-2@56w.png" alt="Avatar Tailwind CSS Component" />
                </div>
              </div>
              <div>
                <div class="font-bold">{patientRecord.whanau}, {patientRecord.ingoa}</div>
                <div class="text-sm opacity-50">{patientRecord.noHeaKoe}</div>
              </div>
            </div>
          </td>
          <td>
            {paitn}
            <br/>
            <span class="badge badge-ghost badge-sm">Maori</span>
          </td>
          <td>{patientRecord.resource_type}</td>
          <th>
            <button class="btn btn-ghost btn-xs">Details</button>
          </th>
        </tr>
      </tbody>
      <!-- foot -->
      <!-- <tfoot> -->
        <!--   <tr> -->
          <!--     <th></th> -->
          <!--     <th>Name</th> -->
          <!--     <th>Iwi</th> -->
          <!--     <th>Resource Type</th> -->
          <!--     <th></th> -->
          <!--   </tr> -->
        <!-- </tfoot> -->
      
    </table>
  </div>

  <!-- <div class="dashboard-section"> -->
    <!--   <div class="dashboard-item" on:click={() => { editing = true; }}> -->

      <!-- ?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80 -->

      <!-- <ul role="list" class="divide-y divide-gray-100"> -->
        <!--   <li class="flex justify-between gap-x-6 py-5"> -->
          <!--     <div class="flex min-w-0 gap-x-4"> -->
            <!--       <img class="h-12 w-12 flex-none rounded-full bg-gray-50" src="https://images.unsplash.com/photo-1519244703995-f4e0f30006d5?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80" alt=""> -->
            <!--       <div class="min-w-0 flex-auto"> -->
              <!--         <p class="text-sm font-semibold leading-6 text-gray-900">Person Name</p> -->
              <!--         <p class="mt-1 truncate text-xs leading-5 text-gray-500">{patientRecord.content}</p> -->
              <!--       </div> -->
            <!--     </div> -->
          <!--     <div class="hidden shrink-0 sm:flex sm:flex-col sm:items-end"> -->
            <!--       <p class="text-sm leading-6 text-gray-900">{patientRecord.resource_type}</p> -->
            <!--       <p class="mt-1 text-xs leading-5 text-gray-500">{new Date(patientRecord.date_visited / 1000).toLocaleString()}</p> -->
            <!--     </div> -->
          <!--   </li> -->
        <!-- </ul> -->
      <!-- <div><strong>Resource Type: </strong>{ patientRecord.resource_type }</div> -->
      <!-- <div><strong>Doctors Notes: </strong>{ patientRecord.content }</div> -->
      <!-- <div class="issue-info"> -->
        <!-- <\!-- <div style="display: flex; flex-direction: row"> -\-> -->
          <!--   <div> -->
            <!--     <\!-- <SvgIcon color="#6fcae8" icon="faBars"></SvgIcon> -\-> -->
              <!--     <strong>Clinical Notes</strong> -->
              <!--     <\!-- <div><i class="fas fa-file"></i> -\-> -->
                <!--   </div> -->
          <!-- </div> -->
      <!-- <div class="dashboard-item-details"> -->
        <!--   {#if patientRecord} -->
          <!--     <\!-- <span>{CommentsForPatientRecord.length} comments</span> -\-> -->
          <!--     <span><CommentsForPatientRecord patientRecordHash="{patientRecordHash}"></CommentsForPatientRecord></span> -->
        <!--   {/if} -->
        <!-- <div style="text-align: right;">{new Date(patientRecord.date_visited / 1000).toLocaleString()}</div> -->
          <!-- </div> -->
      <!-- <\!-- </div> -\-> -->
        <!-- <mwc-icon-button style="margin-left: 8px" icon="edit" on:click={() => { editing = true; } }></mwc-icon-button> -->
        <!-- <mwc-icon-button style="margin-left: 6px" icon="delete" on:click={() => deletePatientRecord()}></mwc-icon-button> -->
        <!--   </div> -->
    <!-- </div> -->

  <!-- <div style="display: flex; flex-direction: column">
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
       <!-- <CommentsForPatientRecord patientRecordHash="{patientRecordHash}"></CommentsForPatientRecord>
            </div> -->
          {/if}
        </div>
</div>

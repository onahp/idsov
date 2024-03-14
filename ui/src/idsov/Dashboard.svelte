<script lang="ts">
  import { onMount, setContext } from 'svelte';
  import type { ActionHash, AppAgentClient } from '@holochain/client';
  import { clientContext, profilesStoreContext } from '../contexts';

  // custom records that are implemented
  import CreatePatientRecord from './patient_records/CreatePatientRecord.svelte';
  import { view, viewHash, navigate } from '../store';

  // this can be placed in the index.js, at the top level of your web-app.
  import "@holochain-open-dev/profiles/dist/elements/profiles-context.js";
  import "@holochain-open-dev/profiles/dist/elements/profile-prompt.js";
  import "@holochain-open-dev/profiles/dist/elements/my-profile.js";
  import "@holochain-open-dev/profiles/dist/elements/list-profiles.js";
  import "@holochain-open-dev/profiles/dist/elements/profile-list-item-skeleton.js";
  import "@holochain-open-dev/profiles/dist/elements/profile-detail.js";
  import { encodeHashToBase64 } from "@holochain/client";

  // files
  import Holochain from "../assets/holochain.png";
  
  import PatientRecordDetail from './patient_records/PatientRecordDetail.svelte';
  import RecordsByUser from './patient_records/RecordsByUser.svelte';
  import { dAppDna, dProfilesStore, dProfilesClient } from "../store";
  import AllRecords from './patient_records/AllRecords.svelte';

  // initialized for client
  // let client: AppAgentClient | undefined;
  let client;
  let loading = true; 
  let profilesStore = undefined;
  let initialized: boolean = false;
  let dna;
  
  export let url = "";

  $: client, loading, profilesStore, initialized, dna;

  // current view context
  let currentView: string | undefined;
  let currentHash: ActionHash | undefined;

  dAppDna.subscribe((value) => {
    dna = value;
  });
  
  dProfilesStore.subscribe((value) => {
    profilesStore = value;
  });
  
  dProfilesClient.subscribe((value) => {
    client = value;
  });

  // set client context
  setContext(clientContext, {
    getClient: () => client,
  });

  // set profiles context
  setContext(profilesStoreContext, {
    getProfileStore: () => profilesStore,
  });

  view.subscribe(value => {
    currentView = value;
  });

  viewHash.subscribe(value => {
    currentHash = value;
  });
  
  console.log(`[Dashboard | Profiles Store] : ${JSON.stringify(profilesStore)}`)
  console.log(`[Dashboard | Profiles Client] : ${JSON.stringify(client)}`)
  console.log(`[Dashboard | Loading] : ${loading}`);

</script>

<div role="tablist" class="tabs tabs-boxed">
  <input type="radio" name="general" role="tab" class="tab" aria-label="Shared Dashboard" checked />
  <div role="tabpanel" class="tab-content bg-base-100 border-base-300 rounded-box p-6">
    <!-- <PatientRecordDetail patientRecordHash={currentHash} /> -->
    <AllRecords />
  </div>

  <input type="radio" name="general" role="tab" class="tab" aria-label="My Dashboard" />
  <div role="tabpanel" class="tab-content bg-base-100 border-base-300 rounded-box p-6">
    <RecordsByUser userRecordHash={client.myPubKey} />
  </div>

  <input type="radio" name="general" role="tab" class="tab" aria-label="Patient Records" />
  <div role="tabpanel" class="tab-content bg-base-100 border-base-300 rounded-box p-6">
    <CreatePatientRecord />
  </div>

  <input type="radio" name="general" role="tab" class="tab" aria-label="Profile" />
  <div role="tabpanel" class="tab-content bg-base-100 border-base-300 rounded-box p-6">
    <div role="tablist" class="tabs tabs-boxed">

      <input type="radio" name="profile" role="tab" class="tab" aria-label="My Agent Profile" checked />
      <div role="tabpanel" class="tab-content bg-base-100 border-base-300 rounded-box p-6">
        <my-profile />
      </div>

      <input type="radio" name="profile" role="tab" class="tab" aria-label="IDSOV Agent List" checked />
      <div role="tabpanel" class="tab-content bg-base-100 border-base-300 rounded-box p-6">
        <list-profiles />
      </div>

    </div>
  </div>

</div>

<!-- <profiles-context store="{profilesStore}"> -->
  <!--   <profile-prompt> -->
    <!--     <main> -->
      <!--       <Header /> -->
      <!--       <div class="flex flex-col w-full border-opacity-50"> -->
        <!--         {#if loading} -->
          <!--           <progress class="progress w-56"></progress> -->
        <!--         {:else if currentView == "patient-record"} -->
          <!--           <PatientRecordDetail patientRecordHash={currentHash} /> -->
        <!--         {:else if currentView == "dashboard"} -->
          <!--           <RecordsByUser userRecordHash={client.myPubKey} /> -->
        <!--         {:else if currentView == "create-patient-record"} -->
          <!--           <CreatePatientRecord /> -->
        <!--         {:else} -->
          <!--           <\!-- <div class="grid h-20 card bg-base-300 rounded-box place-items-center"> -\-> -->
          <!--             <AllRecords /> -->
          <!--             <\!-- </div> -\-> -->
        <!--           {/if} -->
        <!--           <\!-- <div class="grid h-20 card bg-base-300 rounded-box place-items-center">content</div> -\-> -->
              <!--           <div class="divider"></div> -->
              <!--       </div> -->
      <!--       </main> -->
    <!--   </profile-prompt> -->
  <!-- </profiles-context> -->

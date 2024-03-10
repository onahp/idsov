<script lang="ts">
  import { onMount, setContext } from 'svelte';
  import type { ActionHash, AppAgentClient } from '@holochain/client';
  import { AppAgentWebsocket } from '@holochain/client';
  import '@material/mwc-circular-progress';
  import { clientContext, profilesStoreContext } from '../contexts';

  // custom records that are implemented
  // import AllRecords from '';
  import AllRecords from './patient_records/AllRecords.svelte';
  import CreatePatientRecord from './patient_records/CreatePatientRecord.svelte';
  import { view, viewHash, navigate, setWeClient } from '../store';
  import { Router, Link, Route } from 'svelte-routing';

  // this can be placed in the index.js, at the top level of your web-app.
  import { ProfilesClient, ProfilesStore } from '@holochain-open-dev/profiles';
  import "@holochain-open-dev/profiles/dist/elements/profiles-context.js";
  import "@holochain-open-dev/profiles/dist/elements/profile-prompt.js";
  import "@holochain-open-dev/profiles/dist/elements/my-profile.js";
  import "@holochain-open-dev/profiles/dist/elements/list-profiles.js";
  import "@holochain-open-dev/profiles/dist/elements/profile-list-item-skeleton.js";
  import "@holochain-open-dev/profiles/dist/elements/profile-detail.js";

  // files
  import Holochain from "../assets/holochain.png";
  
  // header file import
  import Welcome from '../Welcome.svelte';
  import Header from '../idsov/Header.svelte';
  import Timeline from '../idsov/Timeline.svelte';
  import Footer from '../idsov/Footer.svelte';
  import PatientRecordDetail from './patient_records/PatientRecordDetail.svelte';
  import RecordsByUser from './patient_records/RecordsByUser.svelte';
  import Dashboard from '../idsov/Dashboard.svelte';

  // initialized for client
  let client: AppAgentClient | undefined;
  let loading = true; 
  let profilesStore = undefined;
  let initialized: boolean = false;
  let dna;
  
  export let url = "";

  $: client, loading, profilesStore, initialized, dna;

  // current view context
  let currentView: string | undefined;
  let currentHash: ActionHash | undefined;

  onMount(async () => {
    // We pass an unused string as the url because it will dynamically be replaced in launcher environments
    client = await AppAgentWebsocket.connect(new URL('https://UNUSED'), 'idsov');
    profilesStore = new ProfilesStore(new ProfilesClient(client, "idsov"), {
      avatarMode: "identicon",
    });

    // attempt to get DNA hash
    try {
      dna = await client
        .callZome({
            cap_secret: null,
            role_name: 'idsov',
            zome_name: 'patient_records',
            fn_name: 'get_dna_hash',
            payload: null,
        });
      console.log(`dna ${dna}`)
    } catch (e) {
      console.log("no dna")
      console.log(e)
    }

    loading = false;
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
  

  console.log(`Some profilesStore :: ${profilesStore}`)
  console.log(`loading :: ${loading}`);

</script>

<profiles-context store="{profilesStore}">
  <profile-prompt>
    <!-- <main> -->
      <!-- <Header /> -->
      <div class="flex flex-col w-full border-opacity-50">
        {#if loading}
          <progress class="progress w-56"></progress>
        {:else if currentView == "patient-record"}
          <PatientRecordDetail patientRecordHash={currentHash} />
        {:else if currentView == "dashboard"}
          <RecordsByUser userRecordHash={client.myPubKey} />
        {:else if currentView == "create-patient-record"}
          <CreatePatientRecord />
        {:else}
          <!-- <div class="grid h-20 card bg-base-300 rounded-box place-items-center"> -->
            <AllRecords />
            <!-- </div> -->
          {/if}
          <!-- <div class="grid h-20 card bg-base-300 rounded-box place-items-center">content</div> -->
          <div class="divider"></div>
      </div>
      <!-- </main> -->
  </profile-prompt>
</profiles-context>

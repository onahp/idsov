<script lang="ts">
  import { onMount, setContext } from 'svelte';
  import type { ActionHash, AppAgentClient } from '@holochain/client';
  import { AppAgentWebsocket } from '@holochain/client';
  import '@material/mwc-circular-progress';
  import { clientContext, profilesStoreContext } from './contexts';

  // custom records that are implemented
  import AllRecords from './idsov/patient_records/AllRecords.svelte';
  import CreatePatientRecord from './idsov/patient_records/CreatePatientRecord.svelte';
  import { view, viewHash, navigate, setWeClient } from './store';

  // this can be placed in the index.js, at the top level of your web-app.
  import { ProfilesClient, ProfilesStore } from '@holochain-open-dev/profiles';
  import "@holochain-open-dev/profiles/dist/elements/profiles-context.js";
  import "@holochain-open-dev/profiles/dist/elements/profile-prompt.js";
  import "@holochain-open-dev/profiles/dist/elements/my-profile.js";
  import "@holochain-open-dev/profiles/dist/elements/list-profiles.js";
  import "@holochain-open-dev/profiles/dist/elements/profile-list-item-skeleton.js";
  import "@holochain-open-dev/profiles/dist/elements/profile-detail.js";

  // files
  import Holochain from "./assets/holochain.png";
  
  // header file import
  import Header from './idsov/Header.svelte';
  import Order from './idsov/Order.svelte';
  import Footer from './idsov/Footer.svelte';
  import PatientRecordDetail from './idsov/patient_records/PatientRecordDetail.svelte';
  import RecordsByUser from './idsov/patient_records/RecordsByUser.svelte';

  // initialized for client
  let client: AppAgentClient | undefined;
  let loading = true; 
  let profilesStore = undefined;
  let initialized: boolean = false;
  let dna;

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

<!-- <main>
  {#if profilesStore}
    <profiles-context store={profilesStore}>
      <profile-prompt>
        <list-profiles on:agent-selected={e => alert(e.detail.agentPubKey)}></list-profiles>
        <search-agent include-myself></search-agent>
      </profile-prompt>
    </profiles-context>
  {/if}
  {#if loading}
    <div style="display: flex; flex: 1; align-items: center; justify-content: center">
      <mwc-circular-progress indeterminate />
    </div>
  {:else}
    <div id="content" style="display: flex; flex-direction: column; flex: 1;">
      <CreatePatientRecord></CreatePatientRecord>
      <AllRecords></AllRecords>
    </div>
  {/if}
</main> -->

<!-- {#if profilesStore}
  <profiles-context store={profilesStore}>
    <profile-prompt>
      <main class="idsov-container">
        <list-profiles on:agent-selected={e => alert(e.detail.agentPubKey)}></list-profiles>
        <search-agent include-myself></search-agent>
      </main>
    </profile-prompt>
  </profiles-context>
{/if} -->


<div class="idsov-container">
{#if !loading}
  <profiles-context store="{profilesStore}">
    <Header />
    <div class="flex flex-col w-full border-opacity-50">
        <small class="text-center">
          <div class="badge badge-accent badge-outline">Private Holochain Network</div>
          <br><br>
          {dna}
        </small>
      <div class="divider"></div>
    </div>
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
            <div class="grid h-20 card bg-base-300 rounded-box place-items-center">
              <AllRecords />
            </div>
          {/if}
          <!-- <div class="grid h-20 card bg-base-300 rounded-box place-items-center">content</div> -->
          <div class="divider"></div>
          <div class="hero min-h-screen bg-base-200">
            <div class="hero-content text-center">
              <div class="max-w-md">
                <h1 class="text-5xl font-bold loading loading-infinity loading-lg text-accent"></h1>
                <h1 class="text-5xl font-bold">IDSOV Network</h1>
                <p class="py-6">A simple application to create data within a holochain network with the attempt to share data with external servers</p>
                <!-- <button class="btn btn-primary">Get Started</button> -->
              </div>
            </div>
          </div>
        </div>


      <!-- </main> -->
    </profile-prompt>

  <!-- <div style="display: flex; flex: 1; align-items: center; justify-content: center"> -->
  <!--   <mwc-circular-progress indeterminate /> -->
  <!-- </div> -->
  <!-- <progress class="progress w-56"></progress> -->
  <!-- <Header /> -->
  <!-- <Order /> -->

  </profiles-context>
{/if}
</div>

  <!-- <profiles-context store="{profilesStore}"> -->
    <!-- <profile-prompt> -->
      <!-- <main> -->
      <!-- <main class="idsov-container"> -->
        <!-- <Header /> -->
        <!-- {#if dna && !loading && currentView != "" && currentView != ""} -->
        <!--   <footer style="margin: 10px;"> -->
        <!--     <small> -->
        <!--       Private Holochain Network: {dna} -->
        <!--     </small> -->
        <!--   </footer> -->
        <!-- {/if} -->
        <!-- <div> -->
          <!-- <div class="white-container"> -->
            <!-- <div class="flex flex-col w-full border-opacity-50"> -->
            <!-- {#if loading} -->
              <!-- <div style="display: flex; flex: 1; align-items: center; justify-content: center"> -->
              <!--   <mwc-circular-progress indeterminate /> -->
              <!-- </div> -->
              <!-- <progress class="progress w-56"></progress> -->
            <!-- {:else if currentView == "patient-record"} -->
            <!--   <PatientRecordDetail patientRecordHash={currentHash} />   -->
            <!-- {:else if currentView == "dashboard"} -->
            <!--   <RecordsByUser userRecordHash={client.myPubKey} />   -->
            <!-- {:else if currentView == "create-patient-record"} -->
            <!--   <CreatePatientRecord /> -->
            <!-- {:else} -->
              <!-- <div id="content" style="display: flex; flex-direction: column; flex: 1;"> -->
                <!-- </div> -->
              <!-- <AllRecords /> -->
            <!-- {/if} -->
        <!-- </div> -->

        <!-- <list-profiles on:agent-selected={e => alert(e.detail.agentPubKey)}></list-profiles> -->
        <!-- <br> -->
        <!-- <div class="white-container">
          <div id="content" style="display: flex; flex-direction: column; flex: 1;">
            <list-profiles on:agent-selected={e => alert(e.detail.agentPubKey)}></list-profiles>
          </div>
        </div> -->
          <!-- <list-profiles on:agent-selected={e => alert(e.detail.agentPubKey)}></list-profiles> -->
      <!-- </main> -->
    <!-- </profile-prompt> -->
  <!-- </profiles-context> -->


<!-- {#if loading} -->
<!-- {#if dna && !loading && currentView != "" && currentView != ""} -->
<!--   <footer style="margin: 10px;"> -->
<!--     <small> -->
<!--       <\!-- <img class="holochain-logo" src={Holochain} alt="holochain logo"/> -\-> -->
<!--       Private Holochain Network: {dna} -->
<!--     </small> -->
<!--   </footer> -->
<!-- {/if} -->

<!-- <style> -->
<!--   main { -->
<!--   } -->
<!--   @media (min-width: 640px) { -->
<!--     main { -->
<!--       max-width: none; -->
<!--     } -->
<!--   } -->
<!-- </style> -->

<!-- <Footer /> -->

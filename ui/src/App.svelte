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
        console.log("dna")
      console.log(dna)
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

{#if loading}
  <!-- <div style="display: flex; flex: 1; align-items: center; justify-content: center"> -->
  <!--   <mwc-circular-progress indeterminate /> -->
  <!-- </div> -->
  <progress class="progress w-56"></progress>
{:else}

  <profiles-context store="{profilesStore}">
    <profile-prompt>
      <!-- <main> -->
      <!-- <main class="idsov-container"> -->
        <Header />
        <!-- {#if dna && !loading && currentView != "" && currentView != ""} -->
        <!--   <footer style="margin: 10px;"> -->
        <!--     <small> -->
        <!--       Private Holochain Network: {dna} -->
        <!--     </small> -->
        <!--   </footer> -->
        <!-- {/if} -->
        <div>
        <!-- <div class="white-container"> -->
          {#if loading}
            <!-- <div style="display: flex; flex: 1; align-items: center; justify-content: center"> -->
            <!--   <mwc-circular-progress indeterminate /> -->
            <!-- </div> -->
            <progress class="progress w-56"></progress>
          {:else if currentView == "patient-record"}
            <PatientRecordDetail patientRecordHash={currentHash} />  
          {:else if currentView == "dashboard"}
            <RecordsByUser userRecordHash={client.myPubKey} />  
          {:else if currentView == "create-patient-record"}
            <CreatePatientRecord />
          {:else}
          <!-- <div id="content" style="display: flex; flex-direction: column; flex: 1;"> -->
          <!-- </div> -->
            <!-- <AllRecords /> -->
          {/if}
        </div>

        <!-- <list-profiles on:agent-selected={e => alert(e.detail.agentPubKey)}></list-profiles> -->
        <!-- <br> -->
        <!-- <div class="white-container">
          <div id="content" style="display: flex; flex-direction: column; flex: 1;">
            <list-profiles on:agent-selected={e => alert(e.detail.agentPubKey)}></list-profiles>
          </div>
        </div> -->
          <!-- <list-profiles on:agent-selected={e => alert(e.detail.agentPubKey)}></list-profiles> -->
      <!-- </main> -->
    </profile-prompt>
  </profiles-context>
{/if}

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

<div class="hero min-h-screen bg-base-200">
  <div class="hero-content flex-col lg:flex-row-reverse">
    <div class="text-center lg:text-left">
      <h1 class="text-5xl font-bold loading loading-infinity loading-lg text-accent"></h1>
      <h1 class="text-5xl font-bold">IDSOV Network</h1>
      <p class="py-6">A simple application to create data within a holochain network with the attempt to share data with external servers</p>
    </div>
    <div class="card shrink-0 w-full max-w-sm shadow-2xl bg-base-100">
      <form class="card-body">
        <div class="form-control">
          <label class="label">
            <span class="label-text">Email</span>
          </label>
          <input type="email" placeholder="email" class="input input-bordered" required />
        </div>
        <div class="form-control">
          <label class="label">
            <span class="label-text">Password</span>
          </label>
          <input type="password" placeholder="password" class="input input-bordered" required />
          <label class="label">
            <a href="#" class="label-text-alt link link-hover">Forgot password?</a>
          </label>
        </div>
        <div class="form-control mt-6">
          <button class="btn btn-primary">Login</button>
        </div>
      </form>
    </div>
  </div>
</div>

<ul class="timeline timeline-snap-icon max-md:timeline-compact timeline-vertical">
  <li>
    <div class="timeline-middle">
      <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" class="h-5 w-5"><path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.857-9.809a.75.75 0 00-1.214-.882l-3.483 4.79-1.88-1.88a.75.75 0 10-1.06 1.061l2.5 2.5a.75.75 0 001.137-.089l4-5.5z" clip-rule="evenodd" /></svg>
    </div>
    <div class="timeline-start md:text-end mb-10">
      <time class="font-mono italic">Phase 1</time>
      <div class="text-lg font-black">Centralized Infrastructure Build</div>
      Centralized paradigms have long been the best models for data management. To mimic modern platforms, we use SST to manage data using the same model. SST is a modern framework for building and managing AWS infrastructure. With AWS, we create a service set that is commonly used on the cloud.
    </div>
    <hr/>
  </li>
  <li>
    <hr />
    <div class="timeline-middle">
      <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" class="h-5 w-5"><path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.857-9.809a.75.75 0 00-1.214-.882l-3.483 4.79-1.88-1.88a.75.75 0 10-1.06 1.061l2.5 2.5a.75.75 0 001.137-.089l4-5.5z" clip-rule="evenodd" /></svg>
    </div>
    <div class="timeline-end mb-10">
      <time class="font-mono italic">Phase 2; Part 1</time>
      <div class="text-lg font-black">Learning Holochain</div>
      As Holochain is a novel technology, which required some level of learning, some of the underlying concepts underpinning this technology have had to be learned. From the usage of Rust and its syntax, to the understanding of Zomes, creating entries, the importance of the conductor and other concepts related to the formation of Holochain. This phase included the use of the newly built scaffolding tool to generate prototypes as well as the standard ‘Hello World’ application.
    </div>
    <hr />
  </li>
  <li>
    <hr />
    <div class="timeline-middle">
      <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" class="h-5 w-5"><path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.857-9.809a.75.75 0 00-1.214-.882l-3.483 4.79-1.88-1.88a.75.75 0 10-1.06 1.061l2.5 2.5a.75.75 0 001.137-.089l4-5.5z" clip-rule="evenodd" /></svg>
    </div>
    <div class="timeline-start md:text-end mb-10">
      <time class="font-mono italic">Phase 2; Part 2</time>
      <div class="text-lg font-black">Holochain Prototype - IDSOV</div>
      Given the time constraints, we needed to quickly build a prototype that allowed us to create data, delete data and verify that the data was indeed available to us and another agent. The result of this is this application in use.
    </div>
    <hr />
  </li>
  <li>
    <hr />
    <div class="timeline-middle">
      <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" class="h-5 w-5"><path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.857-9.809a.75.75 0 00-1.214-.882l-3.483 4.79-1.88-1.88a.75.75 0 10-1.06 1.061l2.5 2.5a.75.75 0 001.137-.089l4-5.5z" clip-rule="evenodd" /></svg>
    </div>
    <div class="timeline-end mb-10">
      <time class="font-mono italic">Phase 3</time>
      <div class="text-lg font-black">Shared Data Research</div>
      The sharing of data within the Holochain network is not possible due to the heavy constraints of the project using WASM and the restrictions placed on the conductor to allow communication with websockets.
    </div>
    <hr />
  </li>
  <li>
    <hr />
    <div class="timeline-middle">
      <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" class="h-5 w-5"><path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.857-9.809a.75.75 0 00-1.214-.882l-3.483 4.79-1.88-1.88a.75.75 0 10-1.06 1.061l2.5 2.5a.75.75 0 001.137-.089l4-5.5z" clip-rule="evenodd" /></svg>
    </div>
    <div class="timeline-start md:text-end mb-10">
      <time class="font-mono italic">Submission</time>
      <div class="text-lg font-black">Results</div>
      The results of our analysis suggest that at the current stage in the Holochain project, there is no way to share data without redesigning the project towards that artifact. However, another project named ‘Holo’ is built by the same organization, and is looking to connect Holochain and external services outside the network to achieve a shared data management model.
    </div>
  </li>
</ul>

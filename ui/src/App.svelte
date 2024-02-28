<script lang="ts">
  import { onMount, setContext } from 'svelte';
  import type { ActionHash, AppAgentClient } from '@holochain/client';
  import { AppAgentWebsocket } from '@holochain/client';
  import '@material/mwc-circular-progress';
  import { clientContext, profilesStoreContext } from './contexts';

  // custom records that are implemented
  import AllRecords from './idsov/patient_records/AllRecords.svelte';
  import CreatePatientRecord from './idsov/patient_records/CreatePatientRecord.svelte';

  // this can be placed in the index.js, at the top level of your web-app.
  import { ProfilesClient, ProfilesStore } from '@holochain-open-dev/profiles';

  // files
  import Holochain from "./assets/holochain.png";

  // initialized 
  let client: AppAgentClient | undefined;
  let loading = true; 
  let profilesStore = undefined;
  let initialized: boolean = false;
  let dna;
  $: client, loading, profilesStore, initialized, dna;

  onMount(async () => {
    // We pass an unused string as the url because it will dynamically be replaced in launcher environments
    client = await AppAgentWebsocket.connect(new URL('https://UNUSED'), 'idsov');
    profilesStore = new ProfilesStore(new ProfilesClient(client, "idsov"), {
      avatarMode: "identicon",
    });
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
  <div style="display: flex; flex: 1; align-items: center; justify-content: center">
    <mwc-circular-progress indeterminate />
  </div>
{:else}
  <main class="idsov-container">
    <div id="content" style="display: flex; flex-direction: column; flex: 1;">
      <CreatePatientRecord></CreatePatientRecord>
      <AllRecords></AllRecords>
    </div>
  </main>
{/if}

{#if loading}
  <footer style="margin: 10px;">
    <small>
      <img class="holochain-logo" src={Holochain} alt="holochain logo"/>
      Private Holochain network: {dna}
    </small>
  </footer>
{/if}


<style>
  :root {
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen,
      Ubuntu, Cantarell, 'Open Sans', 'Helvetica Neue', sans-serif;
  }

  main {
    text-align: center;
    padding: 1em;
    max-width: 240px;
    margin: 0 auto;
  }

  img {
    height: 16rem;
    width: 16rem;
  }

  h1 {
    color: #ff3e00;
    text-transform: uppercase;
    font-size: 4rem;
    font-weight: 100;
    line-height: 1.1;
    margin: 2rem auto;
    max-width: 14rem;
  }

  p {
    max-width: 14rem;
    margin: 1rem auto;
    line-height: 1.35;
  }

  @media (min-width: 640px) {
    main {
      max-width: none;
    }
  }
</style>
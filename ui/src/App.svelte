<script lang="ts">
  import { onMount, setContext } from 'svelte';
  import type { ActionHash, AppAgentClient } from '@holochain/client';
  import { AppAgentWebsocket } from '@holochain/client';
  import '@material/mwc-circular-progress';
  import { clientContext, profilesStoreContext } from './contexts';

  // custom records that are implemented
  import AllRecords from './idsov/patient_records/AllRecords.svelte';
  import CreatePatientRecord from './idsov/patient_records/CreatePatientRecord.svelte';
  import { view, viewHash, navigate, setWeClient, dAppDna, dProfilesStore, dProfilesClient, dInitialized, dConnected } from './store';
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
  import Holochain from "./assets/holochain.png";
  
  // header file import
  import Welcome from './Welcome.svelte';
  import Header from './idsov/Header.svelte';
  import Timeline from './idsov/Timeline.svelte';
  import Footer from './idsov/Footer.svelte';
  import Dashboard from './idsov/Dashboard.svelte';
  import Register from './idsov/Register.svelte';
  
  // constants
  const appId = 'idsov'
  const appPort = '8888';
  const rurl = `ws://localhost:${appPort}`;
  export let url = "";

  // initialized for client
  let client: AppAgentClient | undefined;
  let loading = true; 
  let store = undefined;
  let profilesStore = undefined;
  let initialized: boolean = false;
  let connected: boolean = false;
  let dna;
  
  $: client, loading, store, profilesStore, initialized, dna;

  // current view context
  let currentView: string | undefined;
  let currentHash: ActionHash | undefined;
  
  async function initialize() : Promise<void> {
    let profilesClient;

    console.log(`initializing... AppAgentWebsocket connection `);
    client = await AppAgentWebsocket.connect(new URL(rurl), appId);
    profilesClient = new ProfilesClient(client, appId);
    profilesStore = new ProfilesStore(profilesClient, {
      avatarMode: "identicon",
      minNicknameLength: 3,
      additionalFields: [
        {
          name: "maunga",
          label: "Maunga",
          required: true,
        },
        {
          name: "moana",
          label: "Moana",
          required: true,
        },
        {
          name: "location",
          label: "Nō Hea Koe",
          required: true,
        },
        {
          name: "whanau",
          label: "Whanāu",
          required: true,
        },
        {
          name: "ingoa",
          label: "Ingoa",
          required: true,
        },
      ],
    });
  }
  
  async function retrieveHappDna() {

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
      dAppDna.set(dna);
      dProfilesStore.set(profilesStore);
      dProfilesClient.set(client);

      console.log(`[DNA Call] : ${dna}`)
      // console.log(`[Profiles Store] : ${JSON.stringify(profilesStore, null, 2)}`)
      // console.log(`[Profiles Client] : ${JSON.stringify(client, null, 2)}`)
      console.log(`[App | Connected] : ${connected}`);
    } catch (e) {
      console.log(`[No DNA] : ${e}`);
    }

    loading = false;
    initialized = true;
    dInitialized.set(initialized);
    dConnected.set(connected);
  }

  onMount(async () => {

    // We pass an unused string as the url because it will dynamically be replaced in launcher environments
    // client = await AppAgentWebsocket.connect(new URL('https://UNUSED'), 'idsov');
    // profilesStore = new ProfilesStore(new ProfilesClient(client, 'idsov'), {
    // });
  
    await initialize();
    await retrieveHappDna();
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

<Router {url}>

  <div class="idsov-container">
    {#if profilesStore}
      <profiles-context store="{profilesStore}">
        <Header />
        <div>
          <Route path="/" component={Welcome} />
          <Route path="/timeline" component={Timeline} />
          <Route path="/dashboard" component={Dashboard} />
          <Route path="/register" component={Register} />
        </div>
      </profiles-context>

    {/if}
  </div>
  <Footer />

</Router>

<script lang="ts">
  import { onMount, setContext } from 'svelte';
  import type { ActionHash, AppAgentClient } from '@holochain/client';
  import { AppAgentWebsocket } from '@holochain/client';
  import '@material/mwc-circular-progress';
  import { clientContext, profilesStoreContext } from './contexts';

  // custom records that are implemented
  import AllRecords from './idsov/patient_records/AllRecords.svelte';
  import CreatePatientRecord from './idsov/patient_records/CreatePatientRecord.svelte';
  import { view, viewHash, navigate, setWeClient, dAppDna } from './store';
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

  // initialized for client
  let client: AppAgentClient | undefined;
  let loading = true; 
  let store = undefined;
  let profilesStore = undefined;
  let initialized: boolean = false;
  let dna;
  
  export let url = "";

  $: client, loading, store, profilesStore, initialized, dna;

  // current view context
  let currentView: string | undefined;
  let currentHash: ActionHash | undefined;

  onMount(async () => {
    // We pass an unused string as the url because it will dynamically be replaced in launcher environments
    // client = await AppAgentWebsocket.connect(new URL('https://UNUSED'), 'idsov');
    // client = await AppAgentWebsocket.connect('idsov', {url: 'https://unused'});
    client = await AppAgentWebsocket.connect('', 'idsov');
    profilesStore = new ProfilesStore(new ProfilesClient(client, 'idsov'), {
      avatarMode: "avatar-required",
      minNicknameLength: 5,
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
      console.log(`DNA Zome Call : ${dna}`)
      dAppDna.set(dna);
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
  

  console.log(`profilesStore :: ${profilesStore}`)
  console.log(`loading :: ${loading}`);
  console.log(`store :: ${store}`);

</script>

<Router {url}>

  <div class="idsov-container">
    {#if !loading}
      <profiles-context store="{profilesStore}">
      <Header />
      <div>
        <Route path="/" component={Welcome} />
        <Route path="/timeline" component={Timeline} />
        <Route path="/dashboard" component={Dashboard} />
      </div>
      </profiles-context>

    {/if}
  </div>
  <Footer />

</Router>

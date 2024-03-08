<script lang="ts">
    import Logo from "../assets/idsov.png";
    import FaPlusCircle from "svelte-icons/fa/FaPlusCircle.svelte";
    import FaBell from "svelte-icons/fa/FaBell.svelte";
    import FaBullhorn from "svelte-icons/fa/FaBullhorn.svelte";
    import FaList from "svelte-icons/fa/FaList.svelte";
    import FaHome from "svelte-icons/fa/FaHome.svelte";
    import FaUsers from "svelte-icons/fa/FaUsers.svelte";
    import { navigate, view } from "../store";
    import { clientContext } from "../contexts";
    import type {
        EntryHash,
        Record,
        AgentPubKey,
        ActionHash,
        AppAgentClient,
        NewEntryAction,
    } from "@holochain/client";
    import { onMount, setContext, getContext } from "svelte";
    import { decode } from "@msgpack/msgpack";
    import "@holochain-open-dev/profiles/dist/elements/agent-avatar.js";
    import "@holochain-open-dev/profiles/dist/elements/my-profile.js";
    import "@holochain-open-dev/profiles/dist/elements/profiles-context.js";
    import type { Profile } from "@holochain-open-dev/profiles";
    import { encodeHashToBase64 } from "@holochain/client";
    import { svgIcons } from "../svgIcons";

    let client: AppAgentClient = (getContext(clientContext) as any).getClient();
    let currentView;

    view.subscribe((value) => {
        currentView = value;
    });

    // async function goToCreate() {
    //     navigate("create-coordination", {});
    // }

    // async function goToNotifications() {
    //     navigate("notifications", {});
    // }

    async function goToDashboard() {
        navigate("dashboard", {});
    }

    // async function goToBulletin() {
    //     navigate("all-coordinations", {});
    // }

</script>

<div class="navbar bg-base-100">
  <div class="navbar-start">
    <!-- <div class="dropdown"> -->
    <!--   <div tabindex="0" role="button" class="btn btn-ghost btn-circle"> -->
    <!--     <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h7" /></svg> -->
    <!--   </div> -->
    <!--   <ul tabindex="0" class="menu menu-sm dropdown-content mt-3 z-[1] p-2 shadow bg-base-100 rounded-box w-52"> -->
    <!--     <li><a>Home</a></li> -->
    <!--     <li><a>Portfolio</a></li> -->
    <!--     <li><a>About</a></li> -->
    <!--   </ul> -->
    <!-- </div> -->
<label class="flex cursor-pointer gap-2">
  <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="5"/><path d="M12 1v2M12 21v2M4.2 4.2l1.4 1.4M18.4 18.4l1.4 1.4M1 12h2M21 12h2M4.2 19.8l1.4-1.4M18.4 5.6l1.4-1.4"/></svg>
  <input type="checkbox" value="night" class="toggle theme-controller"/>
  <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z"></path></svg>
</label>
  </div>
  <div class="navbar-center">
    <a class="btn btn-ghost text-xl" on:click={() => navigate("instructions")}>IDSOV</a>
  </div>
  <div class="navbar-end">
    <button class="btn btn-ghost btn-circle" style="color: green;" on:click={() => navigate("create-patient-record")}>
      <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-person-plus-fill" viewBox="0 0 16 16">
        <path d="M1 14s-1 0-1-1 1-4 6-4 6 3 6 4-1 1-1 1zm5-6a3 3 0 1 0 0-6 3 3 0 0 0 0 6"/>
        <path fill-rule="evenodd" d="M13.5 5a.5.5 0 0 1 .5.5V7h1.5a.5.5 0 0 1 0 1H14v1.5a.5.5 0 0 1-1 0V8h-1.5a.5.5 0 0 1 0-1H13V5.5a.5.5 0 0 1 .5-.5"/>
      </svg>     
    </button>
    {#if currentView == "instructions"}
      <button class="btn btn-ghost btn-circle" style="color:#d5573b" on:click={() => navigate("instructions")}>
        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-house-fill" viewBox="0 0 16 16"><path d="M8.707 1.5a1 1 0 0 0-1.414 0L.646 8.146a.5.5 0 0 0 .708.708L8 2.207l6.646 6.647a.5.5 0 0 0 .708-.708L13 5.793V2.5a.5.5 0 0 0-.5-.5h-1a.5.5 0 0 0-.5.5v1.293z"/><path d="m8 3.293 6 6V13.5a1.5 1.5 0 0 1-1.5 1.5h-9A1.5 1.5 0 0 1 2 13.5V9.293z"/></svg>       
      </button>
    {:else}
      <button class="btn btn-ghost btn-circle" on:click={() => navigate("instructions")}>
        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-house-fill" viewBox="0 0 16 16"><path d="M8.707 1.5a1 1 0 0 0-1.414 0L.646 8.146a.5.5 0 0 0 .708.708L8 2.207l6.646 6.647a.5.5 0 0 0 .708-.708L13 5.793V2.5a.5.5 0 0 0-.5-.5h-1a.5.5 0 0 0-.5.5v1.293z"/><path d="m8 3.293 6 6V13.5a1.5 1.5 0 0 1-1.5 1.5h-9A1.5 1.5 0 0 1 2 13.5V9.293z"/></svg>       
      </button>
    {/if}
    {#if currentView == "dashboard"}
      <button class="btn btn-ghost btn-circle" style="color:#d5573b" on:click={goToDashboard}>
        <div class="indicator">
          <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-card-checklist" viewBox="0 0 16 16"><path d="M14.5 3a.5.5 0 0 1 .5.5v9a.5.5 0 0 1-.5.5h-13a.5.5 0 0 1-.5-.5v-9a.5.5 0 0 1 .5-.5zm-13-1A1.5 1.5 0 0 0 0 3.5v9A1.5 1.5 0 0 0 1.5 14h13a1.5 1.5 0 0 0 1.5-1.5v-9A1.5 1.5 0 0 0 14.5 2z"/><path d="M7 5.5a.5.5 0 0 1 .5-.5h5a.5.5 0 0 1 0 1h-5a.5.5 0 0 1-.5-.5m-1.496-.854a.5.5 0 0 1 0 .708l-1.5 1.5a.5.5 0 0 1-.708 0l-.5-.5a.5.5 0 1 1 .708-.708l.146.147 1.146-1.147a.5.5 0 0 1 .708 0M7 9.5a.5.5 0 0 1 .5-.5h5a.5.5 0 0 1 0 1h-5a.5.5 0 0 1-.5-.5m-1.496-.854a.5.5 0 0 1 0 .708l-1.5 1.5a.5.5 0 0 1-.708 0l-.5-.5a.5.5 0 0 1 .708-.708l.146.147 1.146-1.147a.5.5 0 0 1 .708 0"/></svg>         
          <!-- <span class="badge badge-xs badge-primary indicator-item"></span> -->
        </div>
      </button>
    {:else}
      <button class="btn btn-ghost btn-circle" on:click={goToDashboard}>
        <div class="indicator">
          <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-card-checklist" viewBox="0 0 16 16"><path d="M14.5 3a.5.5 0 0 1 .5.5v9a.5.5 0 0 1-.5.5h-13a.5.5 0 0 1-.5-.5v-9a.5.5 0 0 1 .5-.5zm-13-1A1.5 1.5 0 0 0 0 3.5v9A1.5 1.5 0 0 0 1.5 14h13a1.5 1.5 0 0 0 1.5-1.5v-9A1.5 1.5 0 0 0 14.5 2z"/><path d="M7 5.5a.5.5 0 0 1 .5-.5h5a.5.5 0 0 1 0 1h-5a.5.5 0 0 1-.5-.5m-1.496-.854a.5.5 0 0 1 0 .708l-1.5 1.5a.5.5 0 0 1-.708 0l-.5-.5a.5.5 0 1 1 .708-.708l.146.147 1.146-1.147a.5.5 0 0 1 .708 0M7 9.5a.5.5 0 0 1 .5-.5h5a.5.5 0 0 1 0 1h-5a.5.5 0 0 1-.5-.5m-1.496-.854a.5.5 0 0 1 0 .708l-1.5 1.5a.5.5 0 0 1-.708 0l-.5-.5a.5.5 0 0 1 .708-.708l.146.147 1.146-1.147a.5.5 0 0 1 .708 0"/></svg>         
          <!-- <span class="badge badge-xs badge-primary indicator-item"></span> -->
        </div>
      </button>
    {/if}
    <profile-detail agent-pub-key="{encodeHashToBase64(client.myPubKey)}"></profile-detail>
  </div>
</div>

<!-- <header> -->
<!--     <nav class="navbar"> -->
<!--         <div class="container-fluid idsov-header"> -->
<!--             <div> -->
<!--                 <a -->
<!--                     id="logo" -->
<!--                     class="navbar-brand" -->
<!--                     on:click={() => navigate("instructions")} -->
<!--                 > -->
<!--                     <img class="logo-image" src={Logo} alt="whos-in logo" /> -->
<!--                 </a> -->
<!--             </div> -->
<!--             <div> -->
<!--                 <\!-- {#if initialized} -\-> -->

<!--                 <ul class="nav navbar-nav float-right"> -->
<!--                     <\!-- <li class="bulletin" on:click={goToBulletin}> -->
<!--                         {#if currentView == "all-coordinations"} -->
<!--                             <div class="bulletin-icon" style="color:#1952bb"> -->
<!--                                 <FaBullhorn /> -->
<!--                             </div> -->
<!--                         {:else} -->
<!--                             <div class="bulletin-icon"> -->
<!--                                 <FaBullhorn /> -->
<!--                             </div> -->
<!--                         {/if} -->
<!--                     </li> -\-> -->

<!--                     <li class="dashboard" on:click={() => navigate("instructions")}> -->
<!--                         {#if currentView == "instructions"} -->
<!--                             <div class="dashboard-icon" style="color:#1952bb"> -->
<!--                                 <FaHome /> -->
<!--                             </div> -->
<!--                         {:else} -->
<!--                             <div class="dashboard-icon"> -->
<!--                                 <FaHome /> -->
<!--                             </div> -->
<!--                         {/if} -->
<!--                     </li> -->

<!--                     <svg -->
<!--                         xmlns="http://www.w3.org/2000/svg" -->
<!--                         style="margin: 0 10" -->
<!--                         width="1" -->
<!--                         height="30" -->
<!--                         viewBox="0 0 1 30" -->
<!--                         ><defs -->
<!--                             ><style> -->
<!--                                 .a { -->
<!--                                     fill: none; -->
<!--                                     stroke: rgba(0, 0, 0, 0.15); -->
<!--                                 } -->
<!--                             </style></defs -->
<!--                         ><line -->
<!--                             class="a" -->
<!--                             y2="30" -->
<!--                             transform="translate(0.5)" -->
<!--                         /></svg -->
<!--                     > -->

<!--                     <li class="dashboard" on:click={goToDashboard}> -->
<!--                         {#if currentView == "dashboard"} -->
<!--                             <div class="dashboard-icon" style="color:#1952bb"> -->
<!--                                 <FaList /> -->
<!--                             </div> -->
<!--                         {:else} -->
<!--                             <div class="dashboard-icon"> -->
<!--                                 <FaList /> -->
<!--                             </div> -->
<!--                         {/if} -->
<!--                     </li> -->

<!--                     <svg -->
<!--                         xmlns="http://www.w3.org/2000/svg" -->
<!--                         style="margin: 0 10" -->
<!--                         width="1" -->
<!--                         height="30" -->
<!--                         viewBox="0 0 1 30" -->
<!--                         ><defs -->
<!--                             ><style> -->
<!--                                 .a { -->
<!--                                     fill: none; -->
<!--                                     stroke: rgba(0, 0, 0, 0.15); -->
<!--                                 } -->
<!--                             </style></defs -->
<!--                         ><line -->
<!--                             class="a" -->
<!--                             y2="30" -->
<!--                             transform="translate(0.5)" -->
<!--                         /></svg -->
<!--                     > -->
<!--                     <li class="middle-of-header-right"> -->
<!--                         <div -->
<!--                             class="new-action-button" -->
<!--                             on:click={() => { -->
<!--                                 navigate("create-patient-record", {}); -->
<!--                             }} -->
<!--                         > -->
<!--                             <div class="icon"> -->
<!--                                 <FaPlusCircle /> -->
<!--                             </div> -->
<!--                             <\!-- <i class="fas fa-plus white-circle-plus"></i> -\-> -->
<!--                             <\!-- <img class="nav-image" src="/assets/add_circle_black_24dp-b42cee553b2665d6f62bd5d9ffc02837cf3c5a3084fc6a5674f5edf83776f565.svg" alt="Add circle black 24dp" border="0"> -\-> -->
<!--                             <span id="new-action">New Patient Record</span> -->
<!--                         </div> -->
<!--                     </li> -->

<!--                     <svg -->
<!--                         xmlns="http://www.w3.org/2000/svg" -->
<!--                         style="margin: 0 10" -->
<!--                         width="1" -->
<!--                         height="30" -->
<!--                         viewBox="0 0 1 30" -->
<!--                         ><defs -->
<!--                             ><style> -->
<!--                                 .a { -->
<!--                                     fill: none; -->
<!--                                     stroke: rgba(0, 0, 0, 0.15); -->
<!--                                 } -->
<!--                             </style></defs -->
<!--                         ><line -->
<!--                             class="a" -->
<!--                             y2="30" -->
<!--                             transform="translate(0.5)" -->
<!--                         /></svg -->
<!--                     > -->
<!--                     <li class="notifications-li"> -->
<!--                         <\!-- <my-profile></my-profile> -\-> -->
<!--                         <\!-- <agent-avatar -->
<!--                             disable-tooltip={true} -->
<!--                             disable-copy={true} -->
<!--                             size={30} -->
<!--                             agent-pub-key={encodeHashToBase64(client.myPubKey)} -->
<!--                         ></agent-avatar> -\-> -->
<!--                         <profile-detail agent-pub-key="{encodeHashToBase64(client.myPubKey)}"></profile-detail> -->
<!--                         <\!-- <agent-mention agent-pub-key="{encodeHashToBase64(client.myPubKey)}"></agent-mention> -\-> -->
<!--                     </li> -->
<!--                 </ul> -->
<!--                 <\!-- {/if} -\-> -->
<!--             </div> -->
<!--             <\!-- /.navbar-collapse -\-> -->
<!--         </div> -->
<!--         <\!-- /.container-fluid -\-> -->
<!--     </nav> -->
<!-- </header> -->

<!-- <label class="flex cursor-pointer gap-2"> -->
<!--   <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="5"/><path d="M12 1v2M12 21v2M4.2 4.2l1.4 1.4M18.4 18.4l1.4 1.4M1 12h2M21 12h2M4.2 19.8l1.4-1.4M18.4 5.6l1.4-1.4"/></svg> -->
<!--   <input type="checkbox" value="synthwave" class="toggle theme-controller"/> -->
<!--   <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z"></path></svg> -->
<!-- </label> -->

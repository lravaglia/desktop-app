<script lang="ts">
  import TopRow from './lib/TopRow.svelte'
  import { currentTab, Tab } from './stores'
</script>

<TopRow />

<main class="container">
  {#if $currentTab == Tab.Home}
    <h1>Home</h1>
  {:else if $currentTab == Tab.Help}
    <h1>Help</h1>
  {/if}
</main>

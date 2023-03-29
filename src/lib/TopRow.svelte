<script lang="ts">
  import { currentTab, Tab } from "../stores";

  /**
   * @param tab The tab to navigate to.
   * @returns A callback to set the tab store.
   */
  const setTab = (newTab: Tab) => (): void => currentTab.set(newTab);
</script>

<nav>
  <ul>
    <li><button type="button" on:click={setTab(Tab.Home)}>Home</button></li>
    <li><button type="button" on:click={setTab(Tab.Help)}>Help</button></li>
  </ul>
</nav>

<style>
  nav {
    background: #30638e;
    color: #d1495b;
    padding: 1% 10%;
    margin: 0%;
  }

  ul {
    list-style: none;
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 0;
    margin: 0;
  }

  li {
    display: block;
    padding: 0;
    margin: 0.25rem 0%;
  }

  button {
    all: unset;
    font-size: 1.5rem;
  }
</style>

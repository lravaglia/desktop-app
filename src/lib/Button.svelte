<script lang="ts">
  // The function to be called when the button is clicked.
  export let action: () => Promise<void> | void;
</script>

<button type="button" on:click={action}>
  <slot />
</button>

<style>
  button {
    all: unset;
    font-size: 1.5rem;
  }
</style>

<script lang="ts">
  import { privateKeyStore } from "$lib/private_key_store";
  import type { FormEventHandler } from "svelte/elements";

  let keyName: string;
  let keyContent: string;

  const handleSubmit: FormEventHandler<HTMLFormElement> = (ev) => {
    privateKeyStore.update((current) => ({
      ...current,
      [keyName]: keyContent,
    }));

    ev.currentTarget.reset();
  };
</script>

<form on:submit|preventDefault={handleSubmit}>
  <input name="key_name" placeholder="Name of your age key" bind:value={keyName} />
  <textarea bind:value={keyContent} />
  <button>Add Key</button>
</form>
<pre>
  {JSON.stringify($privateKeyStore, null, 4)}
</pre>
